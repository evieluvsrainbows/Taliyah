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

async fn fetch_species(client: &reqwest::Client, id: u32) -> Option<PokemonSpecies> {
    let url = format!("https://pokeapi.co/api/v2/pokemon-species/{id}/");
    client.get(&url).send().await.ok()?.json().await.ok()
}

async fn fetch_full_data(client: &reqwest::Client, name: &str) -> Result<Option<PokemonData>, Error> {
    let normalized = utils::pokemon::normalize_pokemon_name(name);
    let Some(pokemon) = fetch_pokemon(client, &normalized).await? else {
        return Ok(None);
    };

    let species = fetch_species(client, pokemon.id).await;
    Ok(Some(PokemonData { pokemon, species }))
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

fn build_components(evo_stages: &[String], current_name: &str, is_shiny: bool, disabled: bool, timeout_expired: bool) -> Vec<serenity::CreateActionRow> {
    let mut rows = Vec::new();

    if evo_stages.len() > 1 && !timeout_expired {
        let options = evo_stages
            .iter()
            .map(|stage| {
                let is_selected = stage.eq_ignore_ascii_case(current_name);
                serenity::CreateSelectMenuOption::new(utils::uppercase_first(stage), stage).default_selection(is_selected)
            })
            .collect();

        let select_menu = serenity::CreateSelectMenu::new("select_evolution", serenity::CreateSelectMenuKind::String { options })
            .placeholder("Navigate Evolution Line...")
            .disabled(disabled);

        rows.push(serenity::CreateActionRow::SelectMenu(select_menu));
    }

    let smogon_url = format!("https://www.smogon.com/dex/sv/pokemon/{}/", current_name.to_lowercase());
    let bulbapedia_url = format!("https://bulbapedia.bulbagarden.net/wiki/{}_(Pokémon)", utils::uppercase_first(current_name));

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

    let evo_summary = if evo_stages.is_empty() {
        "N/A".to_string()
    } else {
        evo_stages
            .iter()
            .map(|n| {
                let formatted = utils::uppercase_first(n);
                if n.eq_ignore_ascii_case(&pokemon.name) { format!("**{formatted}**") } else { formatted }
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

    let mut embed = serenity::CreateEmbed::new()
        .title(format!("#{:03} {}{}", pokemon.id, utils::uppercase_first(&pokemon.name), badge_str))
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
        .field("Base Exp", base_exp_str, true)
        .field("Base Friendship", happiness_str, true)
        .field("Generation", gen_str, true)
        .color(primary_type_color)
        .thumbnail(image_url);

    if let Some(desc) = flavor_text {
        embed = embed.description(format!("*\"{desc}\"*"));
    }

    let components = build_components(evo_stages, &pokemon.name, is_shiny, components_disabled, false);

    Ok((embed, components))
}

/// Fetch detailed information about a Pokémon.
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

    let session_evo_stages = match &session_data.species {
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
        let loading_components = build_components(&session_evo_stages, &session_data.pokemon.name, session_shiny, true, false);

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

                if let Some(selected_name) = selected_value {
                    if let Some(new_data) = fetch_full_data(client, selected_name).await? {
                        session_data = new_data;

                        let (new_embed, new_components) = build_pokemon_reply(client, &session_data, &session_evo_stages, session_shiny, false).await?;

                        current_embed = new_embed;
                        current_components = new_components;

                        handle
                            .edit(context, poise::CreateReply::default().embed(current_embed.clone()).components(current_components.clone()))
                            .await?;
                    }
                }
            }
            _ => {}
        }
    }

    let timeout_components = build_components(&session_evo_stages, &session_data.pokemon.name, session_shiny, false, true);
    let _ = handle.edit(context, poise::CreateReply::default().embed(current_embed).components(timeout_components)).await;

    Ok(())
}

#[poise::command(slash_command, install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel", subcommands("info"))]
pub async fn pokemon(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}
