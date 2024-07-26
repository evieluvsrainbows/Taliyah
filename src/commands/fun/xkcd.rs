use crate::{Context, Error};
use poise::command;
use rand::Rng;
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
pub async fn xkcd(
    context: Context<'_>,
    #[description = "Retrieve a specific comic."] number: Option<u16>,
    #[description = "Retrieve a random comic."]
    #[flag]
    random: bool
) -> Result<(), Error> {
    if number.is_some() && random {
        context.reply("You cannot provide both a number and the random flag. Please use one or the other!").await?;
        return Ok(());
    }

    // there is likely a way to make this code cleaner, but this code works
    // for now until when or if a better solution is discovered.
    let comic = match number {
        None => {
            if random {
                // update this whenever xkcd pushes new comics.
                let xkcd_range = {
                    let mut rng = rand::thread_rng();
                    rng.gen_range(1..2963)
                };
                &format!("https://xkcd.com/{xkcd_range}/info.0.json")
            } else {
                "https://xkcd.com/info.0.json"
            }
        }
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
