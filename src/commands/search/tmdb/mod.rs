use crate::{Context, Error};
use chrono::NaiveDate;
use poise::CreateReply;
use serde::Deserialize;
use serenity::all::{CreateActionRow, CreateButton, CreateEmbed};

#[derive(Deserialize, Debug)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>
}

#[derive(Deserialize, Debug)]
pub struct SearchResult {
    pub id: u64
}

#[derive(Deserialize, Debug)]
pub struct Collection {
    pub name: String,                // The name of the collection.
    pub overview: String,            // The overview of the collection.
    pub poster_path: String,         // The poster belonging to the collection.
    pub parts: Vec<SimplifiedMovie>  // The movies part of the collection.
}

#[derive(Deserialize, Debug)]
pub struct SimplifiedMovie {
    pub id: u64,              // The TMDb ID belonging to the movie.
    pub overview: String,     // The overview of the movie.
    pub release_date: String, // The release date of the movie.
    pub title: String         // The title of the movie.
}

#[poise::command(slash_command, subcommands("collection"))]
pub async fn tmdb(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Retrieves information about a collection on TMDb.
#[poise::command(slash_command)]
pub async fn collection(ctx: Context<'_>, #[description = "The name of the collection."] name: String) -> Result<(), Error> {
    let data = &ctx.data();
    let client = &data.reqwest_container;
    let api_key = &data.config.api.entertainment.tmdb;
    let search_response = client.get("https://api.themoviedb.org/3/search/collection").query(&[("api_key", api_key), ("query", &name)]);
    let search_result: SearchResponse = search_response.send().await?.json().await?;
    let search_results = search_result.results;
    if search_results.is_empty() {
        ctx.reply(format!("Nothing found for `{name}`. Please try another name.")).await?;
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
    let rows: Vec<CreateActionRow> = parts.chunks(5).map(|c| CreateActionRow::Buttons(c.iter().map(|p| {
        let id = &p.id;
        let title = &p.title;
        let summary = &p.overview;
        let release_date = match &NaiveDate::parse_from_str(&p.release_date, "%Y-%m-%d") {
            Ok(date) => date.format("%B %-e, %Y").to_string(),
            Err(_) => "Unreleased".to_string(),
        };
        fields.push((format!("{title} ({release_date})"), summary, false));
        CreateButton::new_link(format!("https://themoviedb.org/movie/{id}")).label(title)
    }).collect())).collect();

    let embed = CreateEmbed::new().title(name).url(url).thumbnail(poster).color(0x0001_d277).description(overview).fields(fields);
    ctx.send(CreateReply::default().embed(embed).components(rows)).await?;

    Ok(())
}
