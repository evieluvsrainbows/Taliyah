use serde::Deserialize;

pub(crate) struct PokemonData {
    pub(crate) pokemon: Pokemon,
    pub(crate) species: Option<PokemonSpecies>
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Pokemon {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) height: u32,
    pub(crate) weight: u32,
    pub(crate) base_experience: Option<u32>,
    pub(crate) types: Vec<PokemonTypeSlot>,
    pub(crate) abilities: Vec<PokemonAbilitySlot>,
    pub(crate) stats: Vec<PokemonStat>,
    pub(crate) sprites: Sprites
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct PokemonTypeSlot {
    pub(crate) slot: u32,
    pub(crate) r#type: NamedApiResource
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct PokemonAbilitySlot {
    pub(crate) is_hidden: bool,
    pub(crate) ability: NamedApiResource
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct PokemonStat {
    pub(crate) base_stat: u32,
    pub(crate) effort: u32,
    pub(crate) stat: NamedApiResource
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Sprites {
    pub(crate) front_default: Option<String>,
    pub(crate) front_shiny: Option<String>,
    pub(crate) other: Option<OtherSprites>
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct OtherSprites {
    #[serde(rename = "official-artwork")]
    pub(crate) artwork: Option<OfficialArtwork>
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct OfficialArtwork {
    pub(crate) front: Option<String>,
    pub(crate) front_shiny: Option<String>
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct PokemonSpecies {
    pub(crate) gender_rate: i32,
    pub(crate) capture_rate: u32,
    pub(crate) base_happiness: Option<u32>,
    pub(crate) is_legendary: bool,
    pub(crate) is_mythical: bool,
    pub(crate) is_baby: bool,
    pub(crate) generation: NamedApiResource,
    pub(crate) egg_groups: Vec<NamedApiResource>,
    pub(crate) flavor_text_entries: Vec<FlavorTextEntry>,
    pub(crate) evolution_chain: UrlResource
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct FlavorTextEntry {
    pub(crate) flavor_text: String,
    pub(crate) language: NamedApiResource
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct UrlResource {
    pub(crate) url: String
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct NamedApiResource {
    pub(crate) name: String
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct EvolutionChain {
    pub(crate) chain: ChainLink
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct ChainLink {
    pub(crate) species: NamedApiResource,
    pub(crate) evolves_to: Vec<ChainLink>
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct TypeResponse {
    pub(crate) damage_relations: DamageRelations
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct DamageRelations {
    pub(crate) double_damage_from: Vec<NamedApiResource>,
    pub(crate) half_damage_from: Vec<NamedApiResource>,
    pub(crate) no_damage_from: Vec<NamedApiResource>
}
