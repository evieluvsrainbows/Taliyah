use crate::{Context, Error};
use poise::command;
use reqwest::StatusCode;
use serde::Deserialize;
use serenity::all::{CreateActionRow, CreateButton, CreateEmbed, CreateEmbedFooter};

#[derive(Debug, Deserialize)]
struct XkcdComic {
    num: u16,      // the numeric ID of the xkcd comic.
    alt: String,   // the caption of the xkcd comic.
    img: String,   // the image URL of the xkcd comic.
    title: String  // the title of the xkcd comic.
}

/// Retrieves the latest or a specific comic from xkcd.
#[command(slash_command)]
pub async fn xkcd(context: Context<'_>, #[description = "The specific comic no. to retrieve."] number: Option<u16>) -> Result<(), Error> {
    let comic = match number {
        None => "https://xkcd.com/info.0.json",
        Some(number) => &format!("https://xkcd.com/{number}/info.0.json")
    };

    let client = &context.data().reqwest_container;
    let request = client.get(comic).send().await?;
    if request.status() == StatusCode::NOT_FOUND {
        context.reply("You did not provide a valid comic id.").await?;
        return Ok(());
    }

    let response: XkcdComic = request.json().await?;
    let num = response.num;
    let page = format!("https://xkcd.com/{num}/");
    let wiki = format!("https://explainxkcd.com/wiki/index.php/{num}");

    let embed = CreateEmbed::new()
        .title(&response.title)
        .color(0xfafafa)
        .description(&response.alt)
        .image(&response.img)
        .footer(CreateEmbedFooter::new(format!("xkcd comic no. {num}")));

    let links = CreateActionRow::Buttons(vec![CreateButton::new_link(page).label("View on xkcd"), CreateButton::new_link(wiki).label("View wiki")]);
    context.send(poise::CreateReply::default().embed(embed).components(vec![links])).await?;

    Ok(())
}
