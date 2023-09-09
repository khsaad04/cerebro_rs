use poise::serenity_prelude::{self as serenity, Color, Timestamp};

use crate::{Context, Error};

enum Duration {
    Seconds(i64),
    Minutes(i64),
    Hours(i64),
    Days(i64),
}

impl From<String> for Duration {
    fn from(input: String) -> Self {
        let mut i = 0;
        for (idx, c) in input.chars().enumerate() {
            if !c.is_numeric() {
                i += idx;
                break;
            }
        }
        let (amount, unit) = input.split_at(i);
        match unit {
            "s" | "sec" => Self::Seconds(amount.parse::<i64>().unwrap()),
            "m" | "min" => Self::Minutes(amount.parse::<i64>().unwrap() * 60),
            "h" | "hr" | "hour" => Self::Hours(amount.parse::<i64>().unwrap() * 60 * 60),
            "d" | "day" => Self::Days(amount.parse::<i64>().unwrap() * 3600 * 24),
            _ => Self::Seconds(0),
        }
    }
}

/// Kicks a member
#[poise::command(prefix_command, slash_command, required_permissions = "KICK_MEMBERS")]
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
#[poise::command(prefix_command, slash_command, required_permissions = "BAN_MEMBERS")]
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
#[poise::command(prefix_command, slash_command, required_permissions = "BAN_MEMBERS")]
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
    prefix_command,
    slash_command,
    required_permissions = "MODERATE_MEMBERS"
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
    let duration = Duration::from(actual_duration.clone());
    let duration = match duration {
        Duration::Seconds(time) => time,
        Duration::Minutes(time) => time,
        Duration::Hours(time) => time,
        Duration::Days(time) => time,
    };
    let timestamp =
        Timestamp::from_unix_timestamp(Timestamp::unix_timestamp(&Timestamp::now()) + duration)
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
    prefix_command,
    slash_command,
    required_permissions = "MODERATE_MEMBERS"
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
