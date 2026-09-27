use serenity::model::gateway::{Activity, ActivityType};

/// Formats raw Spotify artist lists ("Artist A; Artist B") cleanly with commas and ampersands.
pub fn format_spotify_artists(raw_artists: &str) -> String {
    let artists: Vec<&str> = raw_artists.split(';').map(str::trim).collect();
    match artists.as_slice() {
        [] => String::new(),
        [single] => (*single).to_string(),
        [first, second] => format!("{first} & {second}"),
        _ => {
            let (last, rest) = artists.split_last().unwrap();
            format!("{}, & {}", rest.join(", "), last)
        }
    }
}

/// Parses a presence activity into a human-readable string and optional album artwork URL.
pub fn parse_activity(activity: &Activity) -> (Option<String>, Option<String>) {
    if activity.kind == ActivityType::Custom {
        return (None, None);
    }

    let mut track_art = None;
    let name = activity.name.as_str();

    let text = match activity.kind {
        ActivityType::Listening if name == "Spotify" => {
            if let (Some(song), Some(artists), Some(assets), Some(uri)) = (activity.details.as_deref(), activity.state.as_deref(), activity.assets.as_ref(), activity.sync_id.as_deref()) {
                let album = assets.large_text.as_deref().unwrap_or("an album");
                let artist_formatted = format_spotify_artists(artists);
                let url = format!("https://open.spotify.com/track/{uri}");

                if let Some(art) = assets.large_image.as_deref() {
                    let art_id = art.trim_start_matches("spotify:");
                    track_art = Some(format!("https://i.scdn.co/image/{art_id}"));
                }

                format!("listening to **[{song}]({url})** on **{album}** by **{artist_formatted}** on Spotify")
            } else {
                "listening to".to_string()
            }
        }
        ActivityType::Listening => "listening to".to_string(),
        ActivityType::Playing if name == "Visual Studio Code" => {
            let task = activity.details.as_deref().unwrap_or("");
            let project = activity.state.as_deref().unwrap_or("");
            let app = activity.assets.as_ref().and_then(|a| a.small_text.as_deref()).unwrap_or(name);

            if let Some(file) = task.strip_prefix("Editing ") {
                let clean_project = project.strip_prefix("Workspace: ").unwrap_or(project);
                format!("editing the file **{file}** in project **{clean_project}** with **{app}**")
            } else if let Some(file) = task.strip_prefix("Debugging ") {
                let clean_project = project.strip_prefix("Debugging: ").unwrap_or(project);
                format!("debugging the file **{file}** in project **{clean_project}** with **{app}**")
            } else {
                format!("playing **{name}**")
            }
        }
        ActivityType::Playing => format!("playing **{name}**"),
        ActivityType::Competing => format!("competing in **{name}**"),
        ActivityType::Streaming => format!("streaming **{name}**"),
        _ => String::new()
    };

    if text.is_empty() { (None, None) } else { (Some(text), track_art) }
}
