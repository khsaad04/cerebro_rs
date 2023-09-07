use poise::serenity_prelude::{self as serenity, Color};

use crate::{Context, Error};

/// Kicks a member
#[poise::command(prefix_command, slash_command)]
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
#[poise::command(prefix_command, slash_command)]
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
