use crate::{Context, Error};
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
#[poise::command(slash_command)]
pub async fn xkcd(context: Context<'_>, #[description = "Gets a specific comic."] number: Option<u16>, #[description = "Gets a random comic."] random: Option<bool>) -> Result<(), Error> {
    let client = &context.data().reqwest_container;
    let comic = match number {
        None if random.unwrap_or_default() => {
            let request = client.get("https://xkcd.com/info.0.json").send().await?;
            let id = request.json::<XkcdComic>().await?.num;
            let mut rng = rand::thread_rng();
            let range = loop {
                let num = rng.gen_range(1..id);
                if num != 404 {
                    break num;
                };
            };
            &format!("https://xkcd.com/{range}/info.0.json")
        }
        None => "https://xkcd.com/info.0.json",
        Some(_num) if random.unwrap_or_default() => {
            context.reply("You cannot provide both a number and the random flag. Please use one or the other!").await?;
            return Ok(());
        }
        Some(number) => &format!("https://xkcd.com/{number}/info.0.json")
    };

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
