use poise::serenity_prelude::{self as serenity, EditChannel, GetMessages, Timestamp};

use crate::{Context, Error};

/// Kick a member
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderation",
    required_permissions = "KICK_MEMBERS",
    required_bot_permissions = "KICK_MEMBERS"
)]
pub async fn kick(
    ctx: Context<'_>,
    #[description = "The member you want to kick"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    member: serenity::Member,
    #[description = "The reason for it"]
    #[rest = true]
    reason: Option<String>,
) -> Result<(), Error> {
    let reason = reason.unwrap_or("no reason whatsoever".to_string());
    member.kick_with_reason(&ctx, &reason).await?;

    ctx.say(format!(
        "Successfully kicked `{}` for `{}`",
        member.user.name, reason
    ))
    .await?;
    Ok(())
}

/// Ban a member
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderation",
    required_permissions = "BAN_MEMBERS",
    required_bot_permissions = "BAN_MEMBERS"
)]
pub async fn ban(
    ctx: Context<'_>,
    #[description = "The member you want to ban"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    member: serenity::Member,
    #[description = "The amount of days to delete messages"] ddays: Option<u8>,
    #[description = "The reason for it"]
    #[rest = true]
    reason: Option<String>,
) -> Result<(), Error> {
    let reason = reason.unwrap_or("no reason whatsoever".to_string());
    member
        .ban_with_reason(&ctx, ddays.unwrap_or(7), &reason)
        .await?;

    ctx.say(format!(
        "Successfully banned `{}` for `{}`",
        member.user.name, reason
    ))
    .await?;
    Ok(())
}

/// Unban a banned user
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderation",
    required_permissions = "BAN_MEMBERS",
    required_bot_permissions = "BAN_MEMBERS"
)]
pub async fn unban(
    ctx: Context<'_>,
    #[description = "The user you want to unban"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    user: serenity::User,
) -> Result<(), Error> {
    ctx.guild_id()
        .expect("Could not retrieve guild_id")
        .unban(&ctx, user.id)
        .await?;

    ctx.say(format!("Successfully unbanned `{}`", user.name))
        .await?;
    Ok(())
}

#[derive(Debug)]
struct InvalidDuration;

impl std::fmt::Display for InvalidDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "This is an invalid duration.\nExample: 1d, 4h, 49s")
    }
}

impl std::error::Error for InvalidDuration {}

fn parse_duration(duration: &str) -> Result<i64, InvalidDuration> {
    let mut i = 0;
    for (idx, c) in duration.chars().enumerate() {
        if !c.is_numeric() {
            i += idx;
            break;
        }
    }
    if i == 0 {
        return Err(InvalidDuration);
    }
    let (amount, unit) = duration.split_at(i);
    let amount = amount.parse::<i64>().expect("Could not parse to i64");
    let amount = match unit {
        "s" | "sec" => amount,
        "m" | "min" => amount * 60,
        "h" | "hr" | "hour" => amount * 3600,
        "d" | "day" => amount * 3600 * 24,
        _ => 0,
    };
    Ok(amount)
}

/// Mute/timeout a member
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderation",
    required_permissions = "MODERATE_MEMBERS",
    required_bot_permissions = "MODERATE_MEMBERS",
    aliases("timeout")
)]
pub async fn mute(
    ctx: Context<'_>,
    #[description = "The member you want to mute"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    mut member: serenity::Member,
    #[description = "The duration for it"] duration: Option<String>,
    #[description = "The reason for it"]
    #[rest = true]
    reason: Option<String>,
) -> Result<(), Error> {
    let reason = reason.unwrap_or("no reason whatsoever".to_string());
    let actual_duration = duration.unwrap_or("1h".to_string());
    let duration = parse_duration(&actual_duration)?;
    let timestamp =
        Timestamp::from_unix_timestamp(Timestamp::unix_timestamp(&Timestamp::now()) + duration)
            .expect("Failed to convert to duration");
    member
        .disable_communication_until_datetime(&ctx, timestamp)
        .await?;

    ctx.say(format!(
        "Successfully muted `{}` for `{}` because of `{}`",
        member.user.name, actual_duration, reason
    ))
    .await?;
    Ok(())
}

/// Unmute/Remove timeout from a member
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderation",
    required_permissions = "MODERATE_MEMBERS",
    required_bot_permissions = "MODERATE_MEMBERS"
)]
pub async fn unmute(
    ctx: Context<'_>,
    #[description = "The member you want to unmute"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    mut member: serenity::Member,
) -> Result<(), Error> {
    member.enable_communication(&ctx).await?;

    ctx.say(format!("Successfully unmuted `{}`", member.user.name))
        .await?;
    Ok(())
}

/// Purge/clear messages in a channel
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderation",
    required_permissions = "MANAGE_MESSAGES",
    required_bot_permissions = "MANAGE_MESSAGES",
    aliases("clear")
)]
pub async fn purge(
    ctx: Context<'_>,
    #[description = "The amounts of messages you want to delete"] amount: Option<u8>,
) -> Result<(), Error> {
    let amount = amount.unwrap_or(10);
    let current_channel = ctx.channel_id();
    for message in current_channel
        .messages(&ctx, GetMessages::new().before(ctx.id()).limit(amount))
        .await?
        .into_iter()
    {
        message.delete(&ctx).await?;
    }

    ctx.say(format!(
        "Successfully purged `{amount}` messages from this channel"
    ))
    .await?;
    Ok(())
}

/// Set slowmode in a channel
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderation",
    required_permissions = "MANAGE_MESSAGES",
    required_bot_permissions = "MANAGE_MESSAGES",
    aliases("sm")
)]
pub async fn slowmode(
    ctx: Context<'_>,
    #[description = "The time of slowmode"] duration: Option<String>,
) -> Result<(), Error> {
    let actual_duration = duration.unwrap_or("1h".to_string());
    let duration = parse_duration(&actual_duration)?;
    ctx.channel_id()
        .edit(
            &ctx,
            EditChannel::new()
                .rate_limit_per_user(duration.try_into().expect("Failed to convert to u16")),
        )
        .await?;

    ctx.say(format!(
        "Successfully added slowmode for `{actual_duration}` in this channel"
    ))
    .await?;
    Ok(())
}
