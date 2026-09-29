use futures::future::join_all;
use poise::futures_util::StreamExt;
use poise::serenity_prelude as serenity;

use std::collections::HashMap;
use std::time::Duration;

use crate::models::pokemon::*;
use crate::{Context, Error, utils};

async fn fetch_pokemon(client: &reqwest::Client, normalized_name: &str) -> Result<Option<Pokemon>, Error> {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{normalized_name}");
    let res = client.get(&url).send().await?;

    if res.status() == reqwest::StatusCode::NOT_FOUND {
        Ok(None)
    } else {
        Ok(res.error_for_status()?.json().await?)
    }
}

async fn fetch_species(client: &reqwest::Client, species_url_or_id: &str) -> Option<PokemonSpecies> {
    let url = if species_url_or_id.starts_with("http") {
        species_url_or_id.to_string()
    } else {
        format!("https://pokeapi.co/api/v2/pokemon-species/{species_url_or_id}/")
    };
    client.get(&url).send().await.ok()?.json().await.ok()
}

async fn fetch_full_data(client: &reqwest::Client, name_or_id: &str) -> Result<Option<PokemonData>, Error> {
    let normalized = utils::pokemon::normalize_pokemon_name(name_or_id);
    let Some(pokemon) = fetch_pokemon(client, &normalized).await? else {
        return Ok(None);
    };

    let species = fetch_species(client, &pokemon.species.url).await;

    let available_forms = species
        .as_ref()
        .map(|s| {
            s.varieties
                .iter()
                .map(|v| FormOption {
                    name: v.pokemon.name.clone(),
                    label: utils::pokemon::format_form_label(&v.pokemon.name)
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(Some(PokemonData { pokemon, species, available_forms }))
}

async fn fetch_evolution_chain(client: &reqwest::Client, url: &str) -> Vec<String> {
    let Ok(res) = client.get(url).send().await else {
        return Vec::new();
    };
    let Ok(chain_data) = res.json::<EvolutionChain>().await else {
        return Vec::new();
    };

    fn flatten(link: &ChainLink) -> Vec<String> {
        let mut result = vec![link.species.name.clone()];
        for next in &link.evolves_to {
            result.extend(flatten(next));
        }
        result
    }

    flatten(&chain_data.chain)
}

async fn fetch_type_effectiveness(client: &reqwest::Client, types: &[PokemonTypeSlot]) -> Result<String, Error> {
    let futures = types.iter().map(|slot| {
        let url = format!("https://pokeapi.co/api/v2/type/{}/", slot.r#type.name);
        async move {
            let res = client.get(&url).send().await.ok()?;
            res.json::<TypeResponse>().await.ok()
        }
    });

    let type_datas = join_all(futures).await;

    let mut multipliers: HashMap<String, f32> = HashMap::new();
    for type_data in type_datas.into_iter().flatten() {
        for rel in type_data.damage_relations.double_damage_from {
            *multipliers.entry(rel.name).or_insert(1.0) *= 2.0;
        }
        for rel in type_data.damage_relations.half_damage_from {
            *multipliers.entry(rel.name).or_insert(1.0) *= 0.5;
        }
        for rel in type_data.damage_relations.no_damage_from {
            *multipliers.entry(rel.name).or_insert(1.0) *= 0.0;
        }
    }

    let mut weaknesses = Vec::new();
    let mut resistances = Vec::new();
    let mut immunities = Vec::new();

    for (type_name, mult) in multipliers {
        let formatted = format!("{} ({})", utils::uppercase_first(&type_name), utils::format_multiplier(mult));
        if mult >= 2.0 {
            weaknesses.push((mult, formatted));
        } else if mult > 0.0 && mult < 1.0 {
            resistances.push((mult, formatted));
        } else if mult == 0.0 {
            immunities.push((mult, formatted));
        }
    }

    weaknesses.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    resistances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    let format_group = |header: &str, items: Vec<(f32, String)>| {
        if items.is_empty() {
            None
        } else {
            let joined = items.into_iter().map(|(_, s)| s).collect::<Vec<_>>().join(", ");
            Some(format!("**{header}**\n{joined}"))
        }
    };

    let output_parts = [
        format_group("Weak (Take 2x/4x)", weaknesses),
        format_group("Resistant (Take 0.5x/0.25x)", resistances),
        format_group("Immune (Take 0x)", immunities)
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    if output_parts.is_empty() {
        Ok("Normal effective damage from all types.".to_string())
    } else {
        Ok(output_parts.join("\n"))
    }
}

fn get_best_image_url(pokemon: &Pokemon, show_shiny: bool) -> String {
    let artwork = pokemon.sprites.other.as_ref().and_then(|o| o.artwork.as_ref());

    let target = if show_shiny {
        artwork.and_then(|a| a.front_shiny.as_ref())
    } else {
        artwork.and_then(|a| a.front.as_ref())
    };

    if let Some(url) = target {
        return url.clone();
    }

    if show_shiny {
        pokemon.sprites.front_shiny.clone().unwrap_or_else(|| "https://via.placeholder.com/150".into())
    } else {
        pokemon.sprites.front_default.clone().unwrap_or_else(|| "https://via.placeholder.com/150".into())
    }
}

fn build_components(
    evo_stages: &[String],
    forms: &[FormOption],
    current_pokemon_name: &str,
    species_name: &str,
    is_shiny: bool,
    disabled: bool,
    timeout_expired: bool
) -> Vec<serenity::CreateActionRow> {
    let mut rows = Vec::new();

    if !timeout_expired {
        if evo_stages.len() > 1 {
            let options = evo_stages
                .iter()
                .map(|stage| {
                    let is_selected = stage.eq_ignore_ascii_case(species_name);
                    serenity::CreateSelectMenuOption::new(utils::uppercase_first(stage), stage).default_selection(is_selected)
                })
                .collect();

            let select_menu = serenity::CreateSelectMenu::new("select_evolution", serenity::CreateSelectMenuKind::String { options })
                .placeholder("Navigate Evolution Line...")
                .disabled(disabled);

            rows.push(serenity::CreateActionRow::SelectMenu(select_menu));
        }

        if forms.len() > 1 {
            let options = forms
                .iter()
                .map(|form| {
                    let is_selected = form.name.eq_ignore_ascii_case(current_pokemon_name);
                    serenity::CreateSelectMenuOption::new(&form.label, &form.name).default_selection(is_selected)
                })
                .collect();

            let form_menu = serenity::CreateSelectMenu::new("select_form", serenity::CreateSelectMenuKind::String { options })
                .placeholder("Select Form / Mega / Regional...")
                .disabled(disabled);

            rows.push(serenity::CreateActionRow::SelectMenu(form_menu));
        }
    }

    let smogon_url = format!("https://www.smogon.com/dex/sv/pokemon/{}/", current_pokemon_name.to_lowercase());
    let bulbapedia_url = format!("https://bulbapedia.bulbagarden.net/wiki/{}_(Pokémon)", utils::uppercase_first(species_name));

    let mut buttons = Vec::new();

    if !timeout_expired {
        let toggle_label = if is_shiny { "✨ Standard" } else { "✨ Shiny" };
        let toggle_style = if is_shiny { serenity::ButtonStyle::Success } else { serenity::ButtonStyle::Secondary };

        buttons.push(serenity::CreateButton::new("toggle_shiny").label(toggle_label).style(toggle_style).disabled(disabled));
    }

    buttons.push(serenity::CreateButton::new_link(smogon_url).label("Smogon"));
    buttons.push(serenity::CreateButton::new_link(bulbapedia_url).label("Bulbapedia"));

    rows.push(serenity::CreateActionRow::Buttons(buttons));
    rows
}

async fn build_pokemon_reply(
    client: &reqwest::Client,
    data: &PokemonData,
    evo_stages: &[String],
    is_shiny: bool,
    components_disabled: bool
) -> Result<(serenity::CreateEmbed, Vec<serenity::CreateActionRow>), Error> {
    let pokemon = &data.pokemon;
    let species = &data.species;

    let mut types = pokemon.types.clone();
    types.sort_by_key(|t| t.slot);

    let primary_type_color = types.first().map(|t| utils::pokemon::get_type_color(&t.r#type.name)).unwrap_or(0xEE1515);

    let types_str = types.iter().map(|t| format!("`{}`", utils::uppercase_first(&t.r#type.name))).collect::<Vec<_>>().join(" / ");

    let type_effectiveness_str = fetch_type_effectiveness(client, &types).await?;

    let abilities_str = pokemon
        .abilities
        .iter()
        .map(|a| {
            let name = utils::uppercase_first(&a.ability.name);
            if a.is_hidden { format!("{name} *(Hidden)*") } else { name }
        })
        .collect::<Vec<_>>()
        .join(", ");

    let total_stats: u32 = pokemon.stats.iter().map(|s| s.base_stat).sum();
    let mut ev_yields = Vec::new();

    let stats_str = pokemon
        .stats
        .iter()
        .map(|s| {
            let name = match s.stat.name.as_str() {
                "hp" => "HP",
                "attack" => "Atk",
                "defense" => "Def",
                "special-attack" => "Sp. Atk",
                "special-defense" => "Sp. Def",
                "speed" => "Speed",
                other => other
            };

            if s.effort > 0 {
                ev_yields.push(format!("{} {name}", s.effort));
            }

            format!("**{name}:** {}", s.base_stat)
        })
        .collect::<Vec<_>>()
        .join(" | ");

    let ev_yield_str = if ev_yields.is_empty() { "None".to_string() } else { ev_yields.join(", ") };

    let species_name = species.as_ref().map(|s| s.name.as_str()).unwrap_or(pokemon.name.as_str());

    let evo_summary = if evo_stages.is_empty() {
        "N/A".to_string()
    } else {
        evo_stages
            .iter()
            .map(|n| {
                let formatted = utils::uppercase_first(n);
                if n.eq_ignore_ascii_case(species_name) { format!("**{formatted}**") } else { formatted }
            })
            .collect::<Vec<_>>()
            .join(" ➔ ")
    };

    let flavor_text = species
        .as_ref()
        .and_then(|s| s.flavor_text_entries.iter().find(|e| e.language.name == "en").map(|e| e.flavor_text.replace(['\n', '\x0C'], " ")));

    let (gen_str, egg_str, catch_rate_str, gender_str, happiness_str) = species.as_ref().map_or(("N/A".into(), "N/A".into(), "N/A".into(), "N/A".into(), "N/A".into()), |sp| {
        let r#gen = sp.generation.name.replace("generation-", "Gen ").to_uppercase();
        let eggs = sp.egg_groups.iter().map(|e| utils::uppercase_first(&e.name)).collect::<Vec<_>>().join(", ");
        let catch = format!("{} / 255", sp.capture_rate);
        let gender = utils::pokemon::format_gender_rate(sp.gender_rate);
        let happiness = sp.base_happiness.map_or("N/A".to_string(), |h| h.to_string());
        (r#gen, eggs, catch, gender, happiness)
    });

    let base_exp_str = pokemon.base_experience.map_or("N/A".to_string(), |e| e.to_string());

    let badge_str = species.as_ref().map_or(String::new(), |sp| {
        let mut badges = Vec::new();
        if sp.is_legendary {
            badges.push("⭐ **Legendary**");
        }
        if sp.is_mythical {
            badges.push("✨ **Mythical**");
        }
        if sp.is_baby {
            badges.push("👶 **Baby**");
        }

        if badges.is_empty() { String::new() } else { format!("\n{}", badges.join(" • ")) }
    });

    let image_url = get_best_image_url(pokemon, is_shiny);
    let title_name = utils::pokemon::format_form_label(&pokemon.name);

    let mut embed = serenity::CreateEmbed::new()
        .title(format!("#{:03} {}{}", pokemon.id, title_name, badge_str))
        .field("Type", types_str, true)
        .field("Height", utils::pokemon::format_height(pokemon.height), true)
        .field("Weight", utils::pokemon::format_weight(pokemon.weight), true)
        .field("Type Matchups", type_effectiveness_str, false)
        .field("Abilities", abilities_str, false)
        .field("Base Stats", format!("{stats_str}\n**Total:** {total_stats}"), false)
        .field("Evolution Line", evo_summary, false)
        .field("Gender Ratio", gender_str, true)
        .field("Egg Groups", egg_str, true)
        .field("Catch Rate", catch_rate_str, true)
        .field("EV Yield", ev_yield_str, true)
        .field("Base Exp.", base_exp_str, true)
        .field("Base Friendship", happiness_str, true)
        .field("Generation", gen_str, true)
        .color(primary_type_color)
        .thumbnail(image_url);

    if let Some(desc) = flavor_text {
        embed = embed.description(format!("*\"{desc}\"*"));
    }

    let components = build_components(evo_stages, &data.available_forms, &pokemon.name, species_name, is_shiny, components_disabled, false);

    Ok((embed, components))
}

/// Fetch detailed information about a Pokémon with form support.
#[poise::command(slash_command)]
pub async fn info(context: Context<'_>, #[description = "Name or ID of the Pokémon"] name: String) -> Result<(), Error> {
    context.defer().await?;

    let client = &context.data().reqwest_client;
    let mut session_shiny = false;
    let session_pokemon_name = utils::pokemon::normalize_pokemon_name(&name);

    let Some(mut session_data) = fetch_full_data(client, &session_pokemon_name).await? else {
        context.say(format!("Could not find Pokémon: **{name}**")).await?;
        return Ok(());
    };

    let mut session_evo_stages = match &session_data.species {
        Some(sp) => fetch_evolution_chain(client, &sp.evolution_chain.url).await,
        None => Vec::new()
    };

    let (mut current_embed, mut current_components) = build_pokemon_reply(client, &session_data, &session_evo_stages, session_shiny, false).await?;

    let handle = context
        .send(poise::CreateReply::default().embed(current_embed.clone()).components(current_components.clone()))
        .await?;
    let message = handle.message().await?;

    let mut collector = serenity::ComponentInteractionCollector::new(context.serenity_context())
        .author_id(context.author().id)
        .message_id(message.id)
        .timeout(Duration::from_secs(120))
        .stream();

    while let Some(interaction) = collector.next().await {
        let species_name = session_data.species.as_ref().map(|s| s.name.as_str()).unwrap_or(session_data.pokemon.name.as_str());

        let loading_components = build_components(
            &session_evo_stages,
            &session_data.available_forms,
            &session_data.pokemon.name,
            species_name,
            session_shiny,
            true,
            false
        );

        interaction
            .create_response(
                context.serenity_context(),
                serenity::CreateInteractionResponse::UpdateMessage(serenity::CreateInteractionResponseMessage::new().embed(current_embed.clone()).components(loading_components))
            )
            .await?;

        match interaction.data.custom_id.as_str() {
            "toggle_shiny" => {
                session_shiny = !session_shiny;

                let (new_embed, new_components) = build_pokemon_reply(client, &session_data, &session_evo_stages, session_shiny, false).await?;

                current_embed = new_embed;
                current_components = new_components;

                handle
                    .edit(context, poise::CreateReply::default().embed(current_embed.clone()).components(current_components.clone()))
                    .await?;
            }
            "select_evolution" => {
                let selected_value = match &interaction.data.kind {
                    serenity::ComponentInteractionDataKind::StringSelect { values } => values.first(),
                    _ => None
                };

                if let Some(selected_name) = selected_value
                    && let Some(new_data) = fetch_full_data(client, selected_name).await?
                {
                    session_data = new_data;
                    session_evo_stages = match &session_data.species {
                        Some(sp) => fetch_evolution_chain(client, &sp.evolution_chain.url).await,
                        None => Vec::new()
                    };

                    let (new_embed, new_components) = build_pokemon_reply(client, &session_data, &session_evo_stages, session_shiny, false).await?;

                    current_embed = new_embed;
                    current_components = new_components;

                    handle
                        .edit(context, poise::CreateReply::default().embed(current_embed.clone()).components(current_components.clone()))
                        .await?;
                }
            }
            "select_form" => {
                let selected_value = match &interaction.data.kind {
                    serenity::ComponentInteractionDataKind::StringSelect { values } => values.first(),
                    _ => None
                };

                if let Some(selected_form_name) = selected_value
                    && let Some(new_data) = fetch_full_data(client, selected_form_name).await?
                {
                    session_data = new_data;

                    let (new_embed, new_components) = build_pokemon_reply(client, &session_data, &session_evo_stages, session_shiny, false).await?;

                    current_embed = new_embed;
                    current_components = new_components;

                    handle
                        .edit(context, poise::CreateReply::default().embed(current_embed.clone()).components(current_components.clone()))
                        .await?;
                }
            }
            _ => {}
        }
    }

    let species_name = session_data.species.as_ref().map(|s| s.name.as_str()).unwrap_or(session_data.pokemon.name.as_str());
    let timeout_components = build_components(
        &session_evo_stages,
        &session_data.available_forms,
        &session_data.pokemon.name,
        species_name,
        session_shiny,
        false,
        true
    );

    let _ = handle.edit(context, poise::CreateReply::default().embed(current_embed).components(timeout_components)).await;

    Ok(())
}

fn parse_evolution_details(details: &[EvolutionDetail]) -> String {
    if details.is_empty() {
        return "Special condition".to_string();
    }

    let mut conditions = Vec::new();
    for detail in details {
        match detail.trigger.name.as_str() {
            "level-up" => {
                if let Some(level) = detail.min_level {
                    conditions.push(format!("Level {level}"));
                }
                if let Some(item) = &detail.item {
                    conditions.push(format!("Use {}", utils::clean_name(&item.name)));
                }
                if let Some(held) = &detail.held_item {
                    conditions.push(format!("Hold {}", utils::clean_name(&held.name)));
                }
                if let Some(happiness) = detail.min_happiness {
                    conditions.push(format!("Friendship ≥ {happiness}"));
                }
                if let Some(move_req) = &detail.known_move {
                    conditions.push(format!("Knows {}", utils::clean_name(&move_req.name)));
                }
                if let Some(move_type) = &detail.known_move_type {
                    conditions.push(format!("Knows {} move", utils::clean_name(&move_type.name)));
                }
                if let Some(tod) = &detail.time_of_day
                    && !tod.is_empty()
                {
                    conditions.push(format!("during {}", utils::uppercase_first(tod)));
                }
                if let Some(loc) = &detail.location {
                    conditions.push(format!("at {}", utils::clean_name(&loc.name)));
                }
                if detail.needs_overworld_rain.unwrap_or(false) {
                    conditions.push("during Rain".to_string());
                }
                if let Some(gender) = detail.gender {
                    let g_str = if gender == 1 { "Female" } else { "Male" };
                    conditions.push(format!("({g_str})"));
                }
                if detail.turn_upside_down.unwrap_or(false) {
                    conditions.push("Turn upside down".to_string());
                }
                if let Some(stats) = detail.relative_physical_stats {
                    let stat_cond = match stats {
                        1 => "Atk > Def",
                        -1 => "Atk < Def",
                        _ => "Atk = Def"
                    };
                    conditions.push(stat_cond.to_string());
                }
                if conditions.is_empty() {
                    conditions.push("Level Up".to_string());
                }
            }
            "use-item" => {
                if let Some(item) = &detail.item {
                    conditions.push(format!("Use {}", utils::clean_name(&item.name)));
                } else {
                    conditions.push("Use Item".to_string());
                }
                if let Some(gender) = detail.gender {
                    let g_str = if gender == 1 { "Female" } else { "Male" };
                    conditions.push(format!("({g_str})"));
                }
            }
            "trade" => {
                if let Some(held) = &detail.held_item {
                    conditions.push(format!("Trade holding {}", utils::clean_name(&held.name)));
                } else {
                    conditions.push("Trade".to_string());
                }
            }
            "shed" => {
                conditions.push("Level 20 with empty party slot & Pokeball".to_string());
            }
            other => {
                conditions.push(utils::clean_name(other));
            }
        }
    }

    if conditions.is_empty() { "Base Form".to_string() } else { conditions.join(" + ") }
}

/// Recursively traverses the chain link to populate field lists for the embed.
fn populate_embed_fields(link: &ChainLink, embed: serenity::CreateEmbed) -> serenity::CreateEmbed {
    let base_name = utils::clean_name(&link.species.name);
    if link.evolves_to.is_empty() {
        return embed.field("Evolution Status", format!("**{base_name}** does not evolve."), false);
    }

    let mut updated_embed = embed.field("🌱 Base Stage", format!("**{base_name}**"), false);

    let is_multi_branch = link.evolves_to.len() > 1;
    let mut stage1_text = String::new();

    for child in &link.evolves_to {
        let target_name = utils::clean_name(&child.species.name);
        let condition = parse_evolution_details(&child.evolution_details);

        stage1_text.push_str(&format!("➔ **{target_name}** — *{condition}*\n"));
    }

    let stage1_header = if is_multi_branch { "🌿 Branch Evolutions" } else { "🌿 Stage 1" };

    updated_embed = updated_embed.field(stage1_header, stage1_text, false);

    let mut stage2_text = String::new();
    for child in &link.evolves_to {
        for grand_child in &child.evolves_to {
            let from_name = utils::clean_name(&child.species.name);
            let target_name = utils::clean_name(&grand_child.species.name);
            let condition = parse_evolution_details(&grand_child.evolution_details);

            stage2_text.push_str(&format!("➔ **{target_name}** *(from {from_name})* — *{condition}*\n"));
        }
    }

    if !stage2_text.is_empty() {
        updated_embed = updated_embed.field("🌳 Stage 2", stage2_text, false);
    }

    updated_embed
}

/// View detailed evolution requirements and conditions for a Pokémon.
#[poise::command(slash_command)]
pub async fn evolution(context: Context<'_>, #[description = "Name or ID of the Pokémon"] name: String) -> Result<(), Error> {
    context.defer().await?;

    let client = &context.data().reqwest_client;
    let normalized = name.trim().to_lowercase().replace(' ', "-");
    let species_url = format!("https://pokeapi.co/api/v2/pokemon-species/{normalized}/");
    let species_res = client.get(&species_url).send().await?;
    if species_res.status() == reqwest::StatusCode::NOT_FOUND {
        context.say(format!("Could not find species data for: **{name}**")).await?;
        return Ok(());
    }

    let species: PokemonSpecies = species_res.error_for_status()?.json().await?;
    let chain_res = client.get(&species.evolution_chain.url).send().await?;
    let chain_data: EvolutionChainResponse = chain_res.error_for_status()?.json().await?;
    let title_name = utils::clean_name(&species.name);
    let mut embed = serenity::CreateEmbed::new()
        .title(format!("🧬 Evolution Line: {title_name}"))
        .color(0x3B4CCA)
        .footer(serenity::CreateEmbedFooter::new("Data sourced from PokéAPI"));

    embed = populate_embed_fields(&chain_data.chain, embed);
    context.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}

#[poise::command(slash_command, install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel", subcommands("info", "evolution"))]
pub async fn pokemon(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}
