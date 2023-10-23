use poise::serenity_prelude::{self as serenity, Color, Timestamp};

use crate::{Context, Error};

struct Duration {
    amount: i64,
    _unit: String,
}

impl TryFrom<String> for Duration {
    type Error = i64;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut i = 0;
        for (idx, c) in value.chars().enumerate() {
            if !c.is_numeric() {
                i += idx;
                break;
            }
        }
        let (amount, unit) = value.split_at(i);
        let amount = amount.parse::<i64>().unwrap();
        let amount = match unit {
            "s" | "sec" => amount,
            "m" | "min" => amount * 60,
            "h" | "hr" | "hour" => amount * 3600,
            "d" | "day" => amount * 3600 * 24,
            _ => 0,
        };
        Ok(Self {
            amount,
            _unit: unit.to_string(),
        })
    }
}

/// Kicks a member
#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "KICK_MEMBERS",
    required_bot_permissions = "KICK_MEMBERS"
)]
pub async fn kick(
    ctx: Context<'_>,
    #[description = "The member you want to kick"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    member: serenity::Member,
    #[description = "The reason for it"] reason: Option<String>,
) -> Result<(), Error> {
    let reason = reason.unwrap_or("no reason whatsoever".to_string());
    member.kick_with_reason(&ctx, &reason[..]).await?;
    ctx.send(|msg| {
        msg.embed(|em| {
            em.title("Member Kicked")
                .description(format!(
                    "Successfully kicked {} for {}",
                    member.user.name, reason
                ))
                .color(Color::BLUE)
        })
    })
    .await?;
    Ok(())
}

/// Bans a member
#[poise::command(
    slash_command,
    required_permissions = "BAN_MEMBERS",
    required_bot_permissions = "BAN_MEMBERS"
)]
pub async fn ban(
    ctx: Context<'_>,
    #[description = "The member you want to ban"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    member: serenity::Member,
    #[description = "The amount of messages to delete"] delete_message_duration: Option<u8>,
    #[description = "The reason for it"] reason: Option<String>,
) -> Result<(), Error> {
    let del = delete_message_duration.unwrap_or(7);
    let reason = reason.unwrap_or("no reason whatsoever".to_string());
    member.ban_with_reason(&ctx, del, &reason[..]).await?;
    ctx.send(|msg| {
        msg.embed(|em| {
            em.title("Member Banned")
                .description(format!(
                    "Successfully banned {} for {}",
                    member.user.name, reason
                ))
                .color(Color::BLUE)
        })
    })
    .await?;
    Ok(())
}

/// Unbans a banned user
#[poise::command(
    slash_command,
    required_permissions = "BAN_MEMBERS",
    required_bot_permissions = "BAN_MEMBERS"
)]
pub async fn unban(
    ctx: Context<'_>,
    #[description = "The user you want to unban"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    user: serenity::User,
) -> Result<(), Error> {
    ctx.guild().unwrap().unban(&ctx, user.id).await?;
    ctx.send(|msg| {
        msg.embed(|em| {
            em.title("Member Unbanned")
                .description(format!("Successfully unbanned {}", user.name))
                .color(Color::BLUE)
        })
    })
    .await?;
    Ok(())
}

/// Mute/timeout a member
#[poise::command(
    slash_command,
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
    #[description = "The reason for it"] reason: Option<String>,
) -> Result<(), Error> {
    let actual_duration = duration.unwrap_or("1h".to_string());
    let duration = Duration::try_from(actual_duration.clone()).unwrap();
    let timestamp = Timestamp::from_unix_timestamp(
        Timestamp::unix_timestamp(&Timestamp::now()) + duration.amount,
    )
    .unwrap();

    member
        .disable_communication_until_datetime(&ctx, timestamp)
        .await?;

    let reason = reason.unwrap_or("no reason whatsoever".to_string());
    ctx.send(|msg| {
        msg.embed(|em| {
            em.title("Member Muted")
                .description(format!(
                    "Successfully Muted {} for {} because of {}",
                    member.user.name, actual_duration, reason
                ))
                .color(Color::BLUE)
        })
    })
    .await?;
    Ok(())
}

/// Unmute/Remove timeout from a member
#[poise::command(
    slash_command,
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
    ctx.send(|msg| {
        msg.embed(|em| {
            em.title("Member Muted")
                .description(format!("Successfully Unmuted {}", member.user.name))
                .color(Color::BLUE)
        })
    })
    .await?;
    Ok(())
}

/// Purge/clear messages in a channel
#[poise::command(
    slash_command,
    required_permissions = "MANAGE_MESSAGES",
    required_bot_permissions = "MANAGE_MESSAGES",
    aliases("clear")
)]
pub async fn purge(
    ctx: Context<'_>,
    #[description = "The amounts of messages you want to delete"] amount: Option<u64>,
) -> Result<(), Error> {
    let amount = amount.unwrap_or(10);
    let current_channel = ctx.channel_id();
    for message in current_channel
        .messages(&ctx, |retriever| retriever.before(ctx.id()).limit(amount))
        .await?
        .into_iter()
    {
        message.delete(&ctx).await?;
    }
    ctx.send(|msg| {
        msg.embed(|em| {
            em.title("Purged")
                .description(format!(
                    "Successfully purged {} messages from this channel",
                    amount
                ))
                .color(Color::BLUE)
        })
    })
    .await?;
    Ok(())
}

/// Set slowmode in a channel
#[poise::command(
    slash_command,
    required_permissions = "MANAGE_MESSAGES",
    required_bot_permissions = "MANAGE_MESSAGES",
    aliases("sm")
)]
pub async fn slowmode(
    ctx: Context<'_>,
    #[description = "The time of slowmode"] duration: Option<String>,
) -> Result<(), Error> {
    let actual_duration = duration.unwrap_or("1h".to_string());
    let duration = Duration::try_from(actual_duration.clone()).unwrap();
    ctx.channel_id()
        .edit(&ctx, |c| {
            c.rate_limit_per_user(duration.amount.try_into().unwrap())
        })
        .await?;
    ctx.send(|msg| {
        msg.embed(|em| {
            em.title("Slowmode")
                .description(format!(
                    "Successfully added slowmode for {} in this channel",
                    actual_duration
                ))
                .color(Color::BLUE)
        })
    })
    .await?;
    Ok(())
}
