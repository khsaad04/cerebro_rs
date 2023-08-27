use poise::serenity_prelude as serenity;

use crate::{Context, Error};
use std::time::Instant;

/// Show the bot's latency
#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start_time = Instant::now();
    let msg = ctx.say("Calculating ping...").await?;
    let end_time = Instant::now();

    msg.edit(ctx, |m| {
        m.content(format!("{} ms", (end_time - start_time).as_millis()))
    })
    .await?;
    Ok(())
}

/// Show's the user's avatar
#[poise::command(prefix_command, slash_command)]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "The user whose avatar you want"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    member: Option<serenity::User>,
) -> Result<(), Error> {
    let member = member.as_ref().unwrap_or_else(|| ctx.author());
    ctx.send(|msg| {
        msg.embed(|em| {
            em.title("Avatar")
                .description(format!("{}'s avatar", member.name))
                .image(member.avatar_url().unwrap())
        })
    })
    .await?;
    Ok(())
}
