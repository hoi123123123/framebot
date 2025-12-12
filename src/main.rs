use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use anyhow::Result;
use poise::serenity_prelude as serenity;
use tracing::{error, info};

use crate::commands::character_move::*;
use crate::commands::ping::ping;
use crate::matchers::jaro_matcher::JaroMoveMatcher;
use crate::repositories::wavu_move_repository::WavuMoveRepository;
use crate::services::frame_service::FrameService;

pub mod commands;
pub mod converters;
pub mod matchers;
pub mod move_store;
pub mod repositories;
pub mod services;
pub mod tekken;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, BotState, Error>;

pub struct BotState {
    frame_service: FrameService<WavuMoveRepository, JaroMoveMatcher>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=trace", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv()?;

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::DIRECT_MESSAGES
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                ping(),
                alisa(),
                anna(),
                armorking(),
                asuka(),
                azucena(),
                bryan(),
                claudio(),
                clive(),
                deviljin(),
                dragunov(),
                eddy(),
                fahkumram(),
                feng(),
                heihachi(),
                hwoarang(),
                jack8(),
                jin(),
                jun(),
                kazuya(),
                king(),
                kuma(),
                lars(),
                law(),
                lee(),
                leo(),
                leroy(),
                lidia(),
                lili(),
                nina(),
                miaryzo(),
                panda(),
                paul(),
                raven(),
                reina(),
                shaheen(),
                steve(),
                victor(),
                xiaoyu(),
                yoshimitsu(),
                zafina(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("+".into()),
                mention_as_prefix: true,
                ..Default::default()
            },
            on_error: |error| Box::pin(on_error(error)),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                info!("Starting bot setup");

                info!("Registering poise builtins");
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                info!("Initializing frame service");
                let frame_service =
                    FrameService::try_new(WavuMoveRepository, JaroMoveMatcher).await?;

                info!("Done setting up bot");
                Ok(BotState { frame_service })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client?.start().await?;

    Ok(())
}

async fn on_error(error: poise::FrameworkError<'_, BotState, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => {
            error!("Failed to start bot: {:?}", error)
        }
        poise::FrameworkError::Command { error, ctx, .. } => {
            error!("Error in command `{}`: {:?}", ctx.command().name, error);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                error!("Error while handling error: {}", e)
            }
        }
    }
}
