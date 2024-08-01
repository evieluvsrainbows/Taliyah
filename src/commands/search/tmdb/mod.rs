use crate::{
    models::tmdb::{Movie, Show, SimplifiedMovie},
    utils::{calculate_average_sum, format_int, locale},
    Context, Error
};
use chrono::NaiveDate;
use humantime::format_duration;
use itertools::Itertools;
use poise::CreateReply;
use serde::Deserialize;
use serenity::all::{CreateActionRow, CreateButton, CreateEmbed, CreateEmbedFooter};
use std::time::Duration;

#[derive(Deserialize)]
struct SearchResponse {
    pub results: Vec<SearchResult>
}

#[derive(Deserialize)]
struct SearchResult {
    pub id: u64
}

#[derive(Deserialize)]
#[rustfmt::skip]
struct Collection {
    pub name: String,                           // The name of the collection.
    pub overview: String,                       // The overview of the collection.
    pub poster_path: String,                    // The poster belonging to the collection.
    pub parts: Vec<SimplifiedMovie>             // The movies part of the collection.
}

/// Commands for interacting with The Movie Database (themoviedb.org).
#[poise::command(slash_command, subcommands("collection", "movie", "show"))]
pub async fn tmdb(_context: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Retrieves detailed information about a given collection.
#[poise::command(slash_command)]
pub async fn collection(context: Context<'_>, #[description = "The name of the collection."] name: String) -> Result<(), Error> {
    let data = &context.data();
    let client = &data.reqwest_container;
    let api_key = &data.config.api.entertainment.tmdb;
    let search_response = client.get("https://api.themoviedb.org/3/search/collection").query(&[("api_key", api_key), ("query", &name)]);
    let search_result: SearchResponse = search_response.send().await?.json().await?;
    let search_results = search_result.results;
    if search_results.is_empty() {
        context.reply(format!("Nothing found for `{name}`. Please try another name.")).await?;
        return Ok(());
    }

    let id = search_results.first().unwrap().id;
    let response = client.get(format!("https://api.themoviedb.org/3/collection/{id}")).query(&[("api_key", &api_key)]).send().await?;
    let result: Collection = response.json().await?;

    let name = result.name;
    let poster = format!("https://image.tmdb.org/t/p/original{}", result.poster_path);
    let url = format!("https://www.themoviedb.org/collection/{id}");
    let overview = result.overview;

    let mut parts = result.parts;
    let mut fields = Vec::with_capacity(parts.len());
    parts.sort_by_cached_key(|p| p.id);

    #[rustfmt::skip]
    let rows: Vec<CreateActionRow> = parts.chunks(5).map(|car| CreateActionRow::Buttons(car.iter().map(|part| {
        let id = &part.id;
        let title = &part.title;
        let summary = &part.overview;
        let release_date = match &NaiveDate::parse_from_str(&part.release_date, "%Y-%m-%d") {
            Ok(date) => date.format("%B %-e, %Y").to_string(),
            Err(_) => "Unreleased".to_string(),
        };
        fields.push((format!("{title} ({release_date})"), summary, false));
        CreateButton::new_link(format!("https://themoviedb.org/movie/{id}")).label(title)
    }).collect())).collect();

    let embed = CreateEmbed::new().title(name).url(url).thumbnail(poster).color(0x0001_d277).description(overview).fields(fields);
    context.send(CreateReply::default().embed(embed).components(rows)).await?;

    Ok(())
}

/// Retrieves detailed information about a given film.
#[poise::command(slash_command)]
pub async fn movie(context: Context<'_>, #[description = "Film name"] name: String, #[description = "Film release year"] year: Option<u16>) -> Result<(), Error> {
    let data = &context.data();
    let api_key = &data.config.api.entertainment.tmdb;
    let client = &data.reqwest_container;
    let endpoint = "https://api.themoviedb.org/3/search/movie";
    let response = match year {
        Some(year) => client.get(endpoint).query(&[("api_key", api_key), ("query", &name), ("year", &year.to_string())]),
        None => client.get(endpoint).query(&[("api_key", api_key), ("query", &name)])
    };

    let result: SearchResponse = response.send().await?.json().await?;
    let results = result.results;
    if results.is_empty() {
        context.say(format!("No results found for `{name}`. Please try looking for another movie.")).await?;
        return Ok(());
    }

    let id = results.first().unwrap().id;
    let endpoint = format!("https://api.themoviedb.org/3/movie/{id}");
    let response = client.get(&endpoint).query(&[("api_key", &api_key)]).send().await?;
    let result: Movie = response.json().await?;

    let id = result.id.to_string();
    let status = result.status;
    let title = result.title;
    let tagline = result.tagline.filter(|t| !t.is_empty()).map(|t| format!("*{t}*")).unwrap_or_default();
    let overview = result.overview.map(|ow| if !tagline.is_empty() { format!("\n\n{ow}") } else { ow }).unwrap_or_default();
    let homepage = result.homepage.filter(|h| !h.is_empty()).map(|h| format!("[Website]({h})")).unwrap_or("No Website".to_string());
    let collection = result.collection.map(|c| c.name).unwrap_or("N/A".to_string());
    let studios = result.production_companies.iter().map(|c| &c.name).join("\n");
    let language = locale::get_language_name_from_iso(&result.original_language).to_string();
    let release_date = result.release_date.unwrap().format("%B %e, %Y").to_string();
    let budget = format!("${}", format_int(result.budget));
    let revenue = format!("${}", format_int(result.revenue));
    let imdb = format!("[IMDb](https://www.imdb.com/title/{})", result.imdb_id.unwrap());
    let url = format!("https://www.themoviedb.org/movie/{id}");
    let genres = result.genres.iter().map(|g| &g.name).join("\n");
    let poster_uri = result.poster_path.unwrap();
    let poster = format!("https://image.tmdb.org/t/p/original/{}", &poster_uri.replace('/', ""));
    let user_score_count = result.vote_count;
    let user_score = format!("{}% ({user_score_count} votes)", (result.vote_average * 10.0).round());
    let runtime = format_duration(Duration::from_secs(result.runtime.unwrap() * 60)).to_string();
    let external_links = format!("{homepage} | {imdb}");

    let mut fields = Vec::with_capacity(12);
    fields.push(("Status", &*status, true));
    fields.push(("Film ID", &*id, true));
    fields.push(("Language", &*language, true));
    fields.push(("Runtime", &*runtime, true));
    fields.push(("Release Date", &*release_date, true));
    fields.push(("Collection", &*collection, true));
    fields.push(("User Score", &*user_score, true));
    fields.push(("Box Office", &*revenue, true));
    fields.push(("Budget", &*budget, true));
    fields.push(("Genres", &*genres, true));
    fields.push(("Studios", if !&studios.is_empty() { &*studios } else { "No Known Studios" }, true));
    fields.push(("External Links", &*external_links, true));

    let embed = CreateEmbed::new()
        .title(title)
        .url(url)
        .color(0x01b4e4)
        .thumbnail(poster)
        .description(format!("{tagline}{overview}"))
        .fields(fields)
        .footer(CreateEmbedFooter::new("Powered by TMDb."));

    context.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}

/// Retrieves detailed information about a given television series.
#[poise::command(slash_command)]
pub async fn show(context: Context<'_>, #[description = "The TV series to look up."] name: String) -> Result<(), Error> {
    let data = &context.data();
    let api_key = &data.config.api.entertainment.tmdb;
    let client = &data.reqwest_container;
    let endpoint = "https://api.themoviedb.org/3/search/tv";
    let response = client.get(endpoint).query(&[("api_key", api_key), ("query", &name)]);
    let result: SearchResponse = response.send().await?.json().await?;
    let results = result.results;
    if results.is_empty() {
        context.say(format!("No results found for `{name}`. Please try looking for another series.")).await?;
        return Ok(());
    }

    let id = results.first().unwrap().id;
    let endpoint = format!("https://api.themoviedb.org/3/tv/{id}");
    let response = client.get(&endpoint).query(&[("api_key", &api_key)]).send().await.unwrap();
    let result: Show = response.json().await.unwrap();
    let poster_path = result.poster_path.unwrap();
    let poster = format!("https://image.tmdb.org/t/p/original/{}", &poster_path.replace('/', ""));

    let title = result.name;
    let tagline = if !result.tagline.is_empty() { format!("*{}*", result.tagline) } else { String::new() };
    let overview = result.overview;
    let status = result.status;
    let format = result.format;
    let creators = result.created_by.iter().map(|c| &c.name).join("\n");
    let user_score_count = result.vote_count;
    let user_score = format!("{}% ({user_score_count} votes)", (result.vote_average * 10.0).round());
    let language = locale::get_language_name_from_iso(&result.original_language).to_string();
    let languages = result.languages.iter().map(|l| locale::get_language_name_from_iso(l)).join("\n");
    let origin_countries = result.origin_country.iter().map(|c| locale::get_country_name_from_iso(c)).join("\n");
    let first_aired = result.first_air_date.format("%B %-e, %Y").to_string();
    let last_aired = result.last_air_date.format("%B %-e, %Y").to_string();
    let average_runtime = calculate_average_sum(&result.episode_run_time);
    let runtime = format_duration(Duration::from_secs(average_runtime as u64 * 60)).to_string();
    let networks = result.networks.iter().map(|n| &n.name).join("\n");
    let studios = result.studios.iter().map(|s| &s.name).join("\n");
    let seasons = result.number_of_seasons.to_string();
    let episodes = result.number_of_episodes.to_string();
    let genres = result.genres.iter().map(|genre| &genre.name).join("\n");
    let url = format!("https://themoviedb.org/tv/{id}");

    let mut fields = Vec::with_capacity(15);
    fields.push(("Overview", &*overview, false));
    fields.push(("Status", &*status, true));
    fields.push(("Format", &*format, true));
    fields.push(("Created By", if !creators.is_empty() { &*creators } else { "Unknown" }, true));
    fields.push(("Average Runtime", if !result.episode_run_time.is_empty() { &*runtime } else { "Unknown" }, true));
    fields.push(("User Score", &*user_score, true));
    fields.push(("First Aired", &*first_aired, true));
    fields.push(("Last Aired", &*last_aired, true));
    fields.push(("Main Language", &*language, true));
    fields.push(("Origin Countries", &*origin_countries, true));
    fields.push(("Languages", &*languages, true));
    fields.push(("Seasons", &*seasons, true));
    fields.push(("Episodes", &*episodes, true));
    fields.push(("Genres", &*genres, true));
    fields.push(("Studios", if !result.studios.is_empty() { &*studios } else { "Unknown" }, true));
    fields.push(("Networks", &*networks, true));

    let embed = CreateEmbed::new().title(title).url(url).color(0x01b4e4).thumbnail(poster).description(tagline).fields(fields);
    context.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}
