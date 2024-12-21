mod commands;

use anyhow::Context as _;
use poise::serenity_prelude as serenity;
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;

use commands::{help::*, moderation::*, utils::*};

pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[shuttle_runtime::main]
async fn poise(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some('>'.into()),
                case_insensitive_commands: true,
                ..Default::default()
            },
            commands: vec![
                help(),
                ping(),
                avatar(),
                userinfo(),
                kick(),
                ban(),
                unban(),
                mute(),
                unmute(),
                purge(),
                slowmode(),
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(
        discord_token,
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
    )
    .framework(framework)
    .await
    .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
