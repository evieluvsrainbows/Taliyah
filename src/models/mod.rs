use serde::Deserialize;

pub mod tmdb;

#[derive(Deserialize)]
pub struct Comic {
    pub num: u16,      // the numeric ID of the xkcd comic.
    pub alt: String,   // the caption of the xkcd comic.
    pub img: String,   // the image URL of the xkcd comic.
    pub title: String  // the title of the xkcd comic.
}
