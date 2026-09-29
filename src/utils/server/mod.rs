use std::collections::HashSet;

pub struct CategorizedFeatures {
    pub cosmetics: Vec<&'static str>,
    pub community: Vec<&'static str>,
    pub perks: Vec<&'static str>
}

pub fn get_categorized_features(raw_features: &[impl AsRef<str>]) -> CategorizedFeatures {
    let raw_set: HashSet<&str> = raw_features.iter().map(|s| s.as_ref()).collect();

    // 1. Cosmetics & Customization
    let cosmetics_flags = [
        ("ANIMATED_ICON", "Animated Server Icon"),
        ("ANIMATED_BANNER", "Animated Banner"),
        ("INVITE_SPLASH", "Custom Invite Background"),
        ("VANITY_URL", "Custom Vanity URL"),
        ("ROLE_ICONS", "Role Icons"),
        ("ENHANCED_ROLE_COLORS", "Enhanced Role Colors")
    ];

    let mut cosmetics: Vec<&'static str> = cosmetics_flags
        .iter()
        .filter(|&&(flag, _)| raw_set.contains(flag))
        .map(|&(_, label)| label)
        .chain(if raw_set.contains("BANNER") && !raw_set.contains("ANIMATED_BANNER") {
            Some("Server Banner")
        } else {
            None
        })
        .collect();
    cosmetics.sort();

    // 2. Community & Structure
    let community_flags = [
        ("COMMUNITY", "Community Server"),
        ("NEWS", "Announcement Channels"),
        ("GUILD_ONBOARDING", "Guild Onboarding"),
        ("MEMBER_VERIFICATION_GATE_ENABLED", "Membership Screening"),
        ("SOUNDBOARD", "Custom Soundboard"),
        ("GUILD_TAGS", "Guild Tags")
    ];

    let mut community: Vec<&'static str> = community_flags.iter().filter(|&&(flag, _)| raw_set.contains(flag)).map(|&(_, label)| label).collect();
    community.sort();

    // 3. Audio / Video / Tier Limits (Pick highest unlocked per group)
    let perk_tiers = [
        &[
            ("AUDIO_BITRATE_384_KBPS", "Max. 384 Kbps Bitrate in Voice Channels"),
            ("AUDIO_BITRATE_256_KBPS", "Max. 256 Kbps Bitrate in Voice Channels"),
            ("AUDIO_BITRATE_128_KBPS", "Max. 128 Kbps Bitrate in Voice Channels")
        ][..],
        &[("VIDEO_QUALITY_1080_60FPS", "1080p 60fps Video Stream"), ("VIDEO_QUALITY_720_60FPS", "720p 60fps Video Stream")][..],
        &[("MAX_FILE_SIZE_100_MB", "100 MB Upload Limit for Files"), ("MAX_FILE_SIZE_50_MB", "50 MB Upload Limit for Files")][..],
        &[
            ("STAGE_CHANNEL_VIEWERS_300", "Max. 300 Viewers in Stage Channels"),
            ("STAGE_CHANNEL_VIEWERS_150", "Max. 150 Viewers in Stage Channels"),
            ("STAGE_CHANNEL_VIEWERS_50", "Max. 50 Viewers in Stage Channels")
        ][..]
    ];

    let mut perks: Vec<&'static str> = perk_tiers
        .iter()
        .filter_map(|tier| tier.iter().find(|&&(flag, _)| raw_set.contains(flag)).map(|&(_, label)| label))
        .collect();
    perks.sort();

    CategorizedFeatures { cosmetics, community, perks }
}
