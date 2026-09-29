use crate::utils;

pub fn normalize_pokemon_name(name: &str) -> String {
    name.trim().to_lowercase().replace("♀", "-f").replace("♂", "-m").replace([' ', '.'], "-")
}

pub(crate) fn format_gender_rate(gender_rate: i32) -> String {
    if gender_rate == -1 {
        "Genderless".to_string()
    } else {
        let female_percent = (gender_rate as f32 / 8.0) * 100.0;
        let male_percent = 100.0 - female_percent;
        format!("♂ {male_percent:.1}% / ♀ {female_percent:.1}%")
    }
}

pub(crate) fn format_height(decimeters: u32) -> String {
    let meters = decimeters as f32 / 10.0;
    let total_inches = (meters * 39.3701).round() as u32;
    let feet = total_inches / 12;
    let inches = total_inches % 12;
    format!("{meters:.1} m ({feet}′{inches}″)")
}

pub(crate) fn format_weight(hectograms: u32) -> String {
    let kg = hectograms as f32 / 10.0;
    let lbs = kg * 2.20462;
    format!("{kg:.1} kg ({lbs:.1} lbs)")
}

pub(crate) fn format_form_label(raw_name: &str) -> String {
    let parts: Vec<&str> = raw_name.split('-').collect();

    if parts.len() <= 1 {
        return format!("{} (Regular)", utils::uppercase_first(raw_name));
    }

    let base_name = utils::uppercase_first(parts[0]);
    let suffix = parts[1..].join("-");

    // Check if the suffix is a recognized form or variant
    let form_parts = parts[1..]
        .iter()
        .map(|p| match *p {
            "mega" => "Mega".to_string(),
            "gmax" => "Gigantamax".to_string(),
            "alola" => "Alolan".to_string(),
            "galar" => "Galarian".to_string(),
            "hisui" => "Hisuian".to_string(),
            "paldea" => "Paldean".to_string(),
            other => utils::uppercase_first(other)
        })
        .collect::<Vec<_>>()
        .join(" ");

    // Handle single-form gender/variant cases like "deoxys-normal" or default varieties
    if suffix.eq_ignore_ascii_case("default") || suffix.eq_ignore_ascii_case("normal") {
        format!("{base_name} (Regular)")
    } else {
        format!("{base_name} ({form_parts})")
    }
}

pub fn get_type_color(type_name: &str) -> u32 {
    match type_name.to_lowercase().as_str() {
        "normal" => 0xA8A878,
        "fire" => 0xF08030,
        "water" => 0x6890F0,
        "grass" => 0x78C850,
        "electric" => 0xF8D030,
        "ice" => 0x98D8D8,
        "fighting" => 0xC03028,
        "poison" => 0xA040A0,
        "ground" => 0xE0C068,
        "flying" => 0xA890F0,
        "psychic" => 0xF85888,
        "bug" => 0xA8B820,
        "rock" => 0xB8A038,
        "ghost" => 0x705898,
        "dragon" => 0x7038F8,
        "dark" => 0x705848,
        "steel" => 0xB8B8D0,
        "fairy" => 0xEE99AC,
        "stellar" => 0x43B02A,
        _ => 0x68A090
    }
}
