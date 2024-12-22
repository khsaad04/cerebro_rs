use poise::{
    serenity_prelude::{self as serenity, CreateAttachment},
    CreateReply,
};
use reqwest::StatusCode;

use crate::{Context, Error};
use std::{cmp::Ordering, time::Instant};

/// Show the bot's latency
#[poise::command(prefix_command, slash_command, category = "Utilities")]
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

/// Show the bot's uptime
#[poise::command(prefix_command, slash_command, category = "Utilities")]
pub async fn uptime(ctx: Context<'_>) -> Result<(), Error> {
    let uptime = ctx.data().bot_start_time.elapsed();

    let div_mod = |a, b| (a / b, a % b);

    let seconds = uptime.as_secs();
    let (minutes, seconds) = div_mod(seconds, 60);
    let (hours, minutes) = div_mod(minutes, 60);
    let (days, hours) = div_mod(hours, 24);

    let mut uptime_str = "Uptime:".to_string();
    if days > 0 {
        uptime_str.push_str(&format!(" {days}d"));
    }
    if hours > 0 {
        uptime_str.push_str(&format!(" {hours}h"));
    }
    if minutes > 0 {
        uptime_str.push_str(&format!(" {minutes}m"));
    }
    if seconds > 0 {
        uptime_str.push_str(&format!(" {seconds}s"));
    }
    ctx.say(uptime_str).await?;

    Ok(())
}

/// Show the user's avatar
#[poise::command(prefix_command, slash_command, category = "Utilities", aliases("av"))]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "The user whose avatar you want"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    member: Option<serenity::Member>,
) -> Result<(), Error> {
    let author = ctx.author_member().await.expect("Not a member");
    let member = member.as_ref().unwrap_or(&author);
    ctx.send(
        CreateReply::default()
            .content(format!("`{}`'s avatar", member.user.name))
            .attachment(
                CreateAttachment::url(&ctx, &member.user.avatar_url().expect("No image found"))
                    .await?,
            ),
    )
    .await?;
    Ok(())
}

/// Show weather
#[poise::command(prefix_command, slash_command, category = "Utilities")]
pub async fn weather(
    ctx: Context<'_>,
    #[description = "Location for the weather"]
    #[rest = true]
    location: String,
) -> Result<(), Error> {
    let resp = reqwest::get(format!("https://wttr.in/{location}?format=4")).await?;
    let status_code = resp.status();
    let response: String;
    if status_code == 404 {
        response = format!("Location {location} not found");
    } else if status_code == 400 {
        response = "Are you trying to hack weather?".to_string();
    } else if status_code.cmp(&StatusCode::from_u16(400)?) == Ordering::Greater {
        response = format!("An error occured: {status_code}");
    } else {
        response = resp.text().await?;
    }
    ctx.say(response).await?;
    Ok(())
}
