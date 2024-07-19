use chrono::prelude::NaiveDate;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Movie {
    pub adult: bool,                                  // Whether or not the movie has an adult rating.
    pub belongs_to_collection: Option<Collection>,    // The movie's collection, if applicable.
    pub backdrop_path: Option<String>,                // The URL of the movie's backdrop.
    pub budget: u64,                                  // The movie's total budget.
    pub genres: Vec<Genre>,                           // Genres that apply to the movie.
    pub homepage: Option<String>,                     // The movie's website.
    pub id: u64,                                      // The movie's The Movie Database identifier.
    pub imdb_id: Option<String>,                      // The movie's IMDb identifier.
    pub original_language: String,                    // The movie's original language.
    pub original_title: String,                       // The movie's original title.
    pub overview: Option<String>,                     // The movie's overview / description.
    pub popularity: f64,                              // The movie's popularity.
    pub poster_path: Option<String>,                  // The movie's poster URL.
    pub production_companies: Vec<ProductionCompany>, // The movie's production companies.
    pub production_countries: Vec<ProductionCountry>, // The movie's production countries.
    pub release_date: Option<NaiveDate>,              // The movie's release date.
    pub revenue: u64,                                 // The movie's total amount of revenue.
    pub runtime: Option<u64>,                         // The movie's runtime duration, in minutes.
    pub status: String,                               // The movie's current status as listed on The Movie Database.
    pub tagline: Option<String>,                      // The movie's tagline.
    pub title: String,                                // The movie's title.
    pub video: bool,                                  // Whether or not this movie has a video available.
    pub vote_average: f64,                            // The movie's average user score on The Movie Database.
    pub vote_count: f64                               // The movie's total amount of votes on The Movie Database.
}

#[derive(Deserialize)]
pub struct Collection {
    pub id: u64,               // The ID of the collection.
    pub name: String,          // The name of the collection.
    pub poster_path: String,   // The poster of the collection.
    pub backdrop_path: String  // the backdrop of the collection.
}

#[derive(Deserialize)]
pub struct Genre {
    pub id: u64,      // The genre's ID.
    pub name: String  // The genre's name.
}

#[derive(Deserialize)]
pub struct ProductionCompany {
    pub name: String,           // The friendly name of the production company.
    pub id: u64,                // The ID of the production company on The Movie Database.
    pub origin_country: String  // The country of origin of the production company.
}

#[derive(Deserialize)]
pub struct ProductionCountry {
    pub iso_3166_1: String, // The ISO standard shortcode of the production country.
    pub name: String        // The friendly name of the production country.
}
