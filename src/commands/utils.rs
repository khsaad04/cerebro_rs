use poise::{
    serenity_prelude::{self as serenity, CreateEmbed},
    CreateReply,
};

use crate::{Context, Error};
use std::time::Instant;

/// Shows the bot's latency
#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start_time = Instant::now();
    let msg = ctx.say("Calculating ping...").await?;
    let end_time = Instant::now();

    msg.edit(
        ctx,
        CreateReply::default().content(format!("{} ms", (end_time - start_time).as_millis())),
    )
    .await?;
    Ok(())
}

/// Shows the user's avatar
#[poise::command(prefix_command, slash_command)]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "The user whose avatar you want"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    member: Option<serenity::Member>,
) -> Result<(), Error> {
    let author = ctx.author_member().await.expect("Not a member");
    let member = member.as_ref().unwrap_or(&author);
    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::default()
                .title("Avatar")
                .description(format!("{}'s avatar", member.user.name))
                .image(member.user.avatar_url().expect("No image found")),
        ),
    )
    .await?;
    Ok(())
}

/// Shows the user's info
#[poise::command(prefix_command, slash_command)]
pub async fn userinfo(
    ctx: Context<'_>,
    #[description = "The user whose info you want"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    member: Option<serenity::Member>,
) -> Result<(), Error> {
    let author = ctx.author_member().await.expect("Not a member");
    let member = member.as_ref().unwrap_or(&author);
    let roles = member.roles(ctx).expect("No role data found");
    let mut role_list = String::new();
    for role in roles {
        role_list.push_str(format!("`{}` ", &role.name[..])[..].as_ref());
    }
    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::default()
                .title("User info")
                .description(format!("{}'s user info", member.user.name))
                .thumbnail(member.user.avatar_url().expect("No image found"))
                .field("id", member.user.id.to_string(), true)
                .field(
                    "nickname",
                    member.clone().nick.unwrap_or("none".into()),
                    true,
                )
                .field(
                    "account created",
                    member.user.created_at().to_string(),
                    false,
                )
                .field(
                    "joined server",
                    member
                        .joined_at
                        .expect("Could not retrieve joining info")
                        .to_string(),
                    false,
                )
                .field("roles", role_list, false),
        ),
    )
    .await?;
    Ok(())
}
