use poise::serenity_prelude as serenity;

use crate::{Context, Error};
use std::time::Instant;

/// Shows the bot's latency
#[poise::command(slash_command)]
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

/// Shows the user's avatar
#[poise::command(slash_command)]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "The user whose avatar you want"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    member: Option<serenity::Member>,
) -> Result<(), Error> {
    let author = ctx.author_member().await.expect("Not a member");
    let member = member.as_ref().unwrap_or(&author);
    ctx.send(|msg| {
        msg.embed(|em| {
            em.title("Avatar")
                .description(format!("{}'s avatar", member.user.name))
                .image(member.user.avatar_url().expect("No image found"))
        })
    })
    .await?;
    Ok(())
}

/// Shows the user's info
#[poise::command(slash_command)]
pub async fn userinfo(
    ctx: Context<'_>,
    #[description = "The user whose info you want"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    member: Option<serenity::Member>,
) -> Result<(), Error> {
    let author = ctx.author_member().await.expect("Not a member");
    let member = member.as_ref().unwrap_or(&author);
    let roles = member.roles(&ctx).unwrap();
    let mut role_list = String::new();
    for role in roles {
        role_list.push_str(format!("`{}` ", &role.name[..])[..].as_ref());
    }
    ctx.send(|msg| {
        msg.embed(|em| {
            em.title("User info")
                .description(format!("{}'s user info", member.user.name))
                .thumbnail(member.user.avatar_url().expect("No avatar found"))
                .field("ID", member.user.id, true)
                .field(
                    "Nickname",
                    member.clone().nick.unwrap_or("None".into()),
                    true,
                )
                .field("Account created", member.user.created_at(), false)
                .field("Joined server", member.joined_at.unwrap(), false)
                .field("Roles", role_list, false)
        })
    })
    .await?;
    Ok(())
}
