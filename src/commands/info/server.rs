use crate::{Context, Error, utils};
use itertools::Itertools;
use poise::{CreateReply, serenity_prelude::*};

/// A set of commands for interacting with the server.
#[poise::command(slash_command, guild_only, subcommands("info", "features", "roles"))]
pub async fn server(_context: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Shows information about the current server.
#[poise::command(slash_command, guild_only)]
pub async fn info(context: Context<'_>) -> Result<(), Error> {
    let guild = context.guild().unwrap().clone();
    let id = guild.id;
    let name = &guild.name;
    let icon = guild.icon_url();
    let owner = match guild.member(context.serenity_context(), guild.owner_id).await {
        Ok(member) => member.user.tag(),
        Err(_) => "Unknown".to_string()
    };

    let main_channel = guild.system_channel_id.map(|id| format!("<#{id}>")).unwrap_or_else(|| "None".to_string());
    let creation_date = id.created_at().format("%B %e, %Y").to_string();
    let member_count = guild.members.len();
    let online_count = guild.presences.len();
    let text_channels = guild.channels.values().filter(|c| c.kind == ChannelType::Text).count();
    let voice_channels = guild.channels.values().filter(|c| c.kind == ChannelType::Voice).count();
    let total_channels = guild.channels.len();
    let animated_emojis = guild.emojis.values().filter(|e| e.animated).count();
    let total_emojis = guild.emojis.len();
    let static_emojis = total_emojis - animated_emojis;
    let verification_level = match guild.verification_level {
        VerificationLevel::None => "None",
        VerificationLevel::Low => "Low (Verified Email)",
        VerificationLevel::Medium => "Medium (5m on Discord)",
        VerificationLevel::High => "High (10m in Server)",
        VerificationLevel::Higher => "Highest (Verified Phone)",
        _ => "Unknown"
    };

    let mfa_level = match guild.mfa_level {
        MfaLevel::None => "Not Required",
        MfaLevel::Elevated => "Required 2FA",
        _ => "Unknown"
    };

    let explicit_filter = match guild.explicit_content_filter {
        ExplicitContentFilter::None => "Disabled",
        ExplicitContentFilter::WithoutRole => "Members w/o Role",
        ExplicitContentFilter::All => "Everyone",
        _ => "Unknown"
    };

    let boost_count = guild.premium_subscription_count.unwrap_or(0);
    let boost_tier = match guild.premium_tier {
        PremiumTier::Tier0 => "Level 0",
        PremiumTier::Tier1 => "Level 1",
        PremiumTier::Tier2 => "Level 2",
        PremiumTier::Tier3 => "Level 3",
        _ => "Unknown"
    };

    let role_count = guild.roles.len().saturating_sub(1);
    let highest_role = guild.roles.values().max_by_key(|r| (r.position, r.id));
    let highest_role_name = highest_role.map(|r| r.name.as_str()).unwrap_or("None");
    let highest_role_color = highest_role.map(|r| r.colour);

    let mut embed = CreateEmbed::new()
        .title(name)
        // General Details
        .field("Owner", owner, true)
        .field("Created", creation_date, true)
        .field("Main Channel", main_channel, true)
        // Statistics
        .field("Total Members", member_count.to_string(), true)
        .field("Online Members", online_count.to_string(), true)
        .field("Total Channels", total_channels.to_string(), true)
        .field("Text Channels", text_channels.to_string(), true)
        .field("Voice Channels", voice_channels.to_string(), true)
        .field("Total Emojis", total_emojis.to_string(), true)
        .field("Static Emojis", static_emojis.to_string(), true)
        .field("Animated Emojis", animated_emojis.to_string(), true)
        // Boost Status
        .field("Boost Tier", boost_tier, true)
        .field("Boost Count", boost_count.to_string(), true)
        // Security
        .field("Verification Level", verification_level, true)
        .field("MFA Level", mfa_level, true)
        .field("Explicit Filter", explicit_filter, true)
        // Roles
        .field("Total Roles", role_count.to_string(), true)
        .field("Highest Role", highest_role_name, true)
        .footer(CreateEmbedFooter::new(format!("Server ID: {id}")));

    if let Some(icon) = icon {
        embed = embed.thumbnail(icon);
    }

    if let Some(color) = highest_role_color {
        embed = embed.color(color);
    }

    context.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}

/// Displays the roles that have been set up in the server.
#[poise::command(slash_command, guild_only)]
pub async fn roles(context: Context<'_>) -> Result<(), Error> {
    let guild = context.guild().unwrap().clone();
    let guild_id = guild.id;
    let guild_name = &guild.name;
    let guild_icon = guild.icon_url().unwrap_or_default();

    let guild_roles_sorted: Vec<_> = guild.roles.values().filter(|r| r.id.get() != guild_id.get()).sorted_by_key(|r| r.position).rev().collect();

    let total_roles = guild_roles_sorted.len();

    let highest_role = guild.roles.values().max_by_key(|r| (r.position, r.id));
    let highest_role_name = highest_role.map(|r| r.name.as_str()).unwrap_or("None");
    let highest_role_color = highest_role.map(|r| r.colour);

    let roles_formatted = if total_roles == 0 {
        "No custom roles found.".to_string()
    } else {
        guild_roles_sorted.iter().map(|r| utils::escape_markdown(&r.name)).join(" / ")
    };

    let roles_field_value = if roles_formatted.len() > 1024 {
        let mut safe_end = 1020;
        while !roles_formatted.is_char_boundary(safe_end) {
            safe_end -= 1;
        }
        format!("{}...", &roles_formatted[..safe_end])
    } else {
        roles_formatted
    };

    let mut embed = CreateEmbed::new()
        .title("Server Roles")
        .field("Highest Role", format!("`{highest_role_name}`"), true)
        .field("Role Count", total_roles.to_string(), true)
        .field("Roles", roles_field_value, false)
        .footer(CreateEmbedFooter::new(format!("{guild_name} server ID: {guild_id}")));

    if !guild_icon.is_empty() {
        embed = embed.author(CreateEmbedAuthor::new(guild_name).icon_url(guild_icon));
    } else {
        embed = embed.author(CreateEmbedAuthor::new(guild_name));
    }

    if let Some(color) = highest_role_color {
        embed = embed.color(color);
    }

    context.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}

/// Displays the unlocked features and perks of the current server.
#[poise::command(slash_command, guild_only)]
pub async fn features(context: Context<'_>) -> Result<(), Error> {
    let guild = context.guild().unwrap().clone();
    let guild_name = &guild.name;
    let guild_icon = guild.icon_url().unwrap_or_default();

    let raw_features: Vec<String> = guild.features.iter().map(|f| f.to_string()).collect();
    let features = utils::server::get_categorized_features(&raw_features);

    let total_features = features.cosmetics.len() + features.community.len() + features.perks.len();

    let mut embed = CreateEmbed::new()
        .title("Server Features & Perks")
        .footer(CreateEmbedFooter::new(format!("Total Features Unlocked: {total_features}")));

    if !guild_icon.is_empty() {
        embed = embed.author(CreateEmbedAuthor::new(guild_name).icon_url(guild_icon));
    } else {
        embed = embed.author(CreateEmbedAuthor::new(guild_name));
    }

    if total_features == 0 {
        embed = embed.description("This server has no unlocked special features.");
    } else {
        let format_list = |items: &[&'static str]| -> String { items.iter().map(|item| format!("• {item}")).join("\n") };

        if !features.cosmetics.is_empty() {
            embed = embed.field("🎨 Cosmetics & Customization", format_list(&features.cosmetics), false);
        }

        if !features.community.is_empty() {
            embed = embed.field("👥 Community & Features", format_list(&features.community), false);
        }

        if !features.perks.is_empty() {
            embed = embed.field("🚀 Boost Perks & Limits", format_list(&features.perks), false);
        }
    }

    context.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}
