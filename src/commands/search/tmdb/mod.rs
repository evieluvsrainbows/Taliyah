use crate::{
    models::tmdb::Movie,
    utils::{format_int, locale},
    Context, Error
};
use chrono::NaiveDate;
use humantime::format_duration;
use itertools::Itertools;
use poise::CreateReply;
use serde::Deserialize;
use serenity::all::{CreateActionRow, CreateButton, CreateEmbed, CreateEmbedFooter};
use std::time::Duration;

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

#[poise::command(slash_command, subcommands("collection", "movie"))]
pub async fn tmdb(_context: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Retrieves information about a collection on TMDb.
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
    context.send(CreateReply::default().embed(embed).components(rows)).await?;

    Ok(())
}

/// Retrieves detailed information about a film from TMDb.
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
    }

    let id = results.first().unwrap().id;
    let endpoint = format!("https://api.themoviedb.org/3/movie/{id}");
    let response = client.get(&endpoint).query(&[("api_key", &api_key)]).send().await?;
    let result: Movie = response.json().await?;

    let tagline = match result.tagline {
        Some(tagline) => {
            if tagline.is_empty() {
                String::new()
            } else {
                format!("*{tagline}*")
            }
        }
        None => String::new()
    };

    let overview = match result.overview {
        Some(overview) => {
            if !tagline.is_empty() {
                format!("\n\n{overview}")
            } else {
                overview
            }
        }
        None => String::new()
    };

    let studios = if result.production_companies.is_empty() {
        "No Known Studios".to_string()
    } else {
        result.production_companies.iter().map(|c| &c.name).join("\n")
    };

    let collection = match result.belongs_to_collection {
        Some(collection) => collection.name,
        None => "N/A".to_string()
    };

    let homepage = match result.homepage {
        Some(homepage) => {
            if homepage.is_empty() {
                "No Website"
            } else {
                &format!("[Website]({homepage})")
            }
        }
        None => "No Website"
    };

    let id = result.id.to_string();
    let title = result.title.as_str();
    let status = result.status;
    let language = locale::get_language_name_from_iso(&result.original_language).to_string();
    let release_date = result.release_date.unwrap().format("%B %e, %Y").to_string();
    let budget = format_int(result.budget);
    let revenue = format_int(result.revenue);
    let imdb = format!("[IMDb](https://www.imdb.com/title/{})", result.imdb_id.unwrap());
    let url = format!("https://www.themoviedb.org/movie/{id}");
    let genres = result.genres.iter().map(|g| &g.name).join("\n");
    let popularity = format!("{}%", result.popularity.round());
    let poster_uri = result.poster_path.unwrap();
    let poster = format!("https://image.tmdb.org/t/p/original/{}", &poster_uri.replace('/', ""));
    let user_score = format!("{}/100", (result.vote_average * 10.0).round());
    let user_score_count = result.vote_count;
    let runtime = format_duration(Duration::from_secs(result.runtime.unwrap() * 60)).to_string();
    let external_links = format!("{homepage} | {imdb}");

    let embed = CreateEmbed::new()
        .title(title)
        .url(url)
        .color(0x01b4e4)
        .thumbnail(poster)
        .description(format!("{tagline}{overview}"))
        .fields(vec![
            ("Status", status, true),
            ("Film ID", id, true),
            ("Language", language, true),
            ("Runtime", runtime, true),
            ("Release Date", release_date, true),
            ("Collection", collection, true),
            ("Popularity", popularity, true),
            ("User Score", format!("{user_score} ({user_score_count} votes)"), true),
            ("Budget", format!("${budget}"), true),
            ("Box Office", format!("${revenue}"), true),
            ("Genres", genres, true),
            ("Studios", studios, true),
            ("External Links", external_links, false),
        ])
        .footer(CreateEmbedFooter::new("Powered by the The Movie Database API."));

    context.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}
