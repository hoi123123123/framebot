use poise::{
    command,
    serenity_prelude::{Colour, CreateEmbed},
};
use scraper::Html;
use tracing::{info, instrument};

use anyhow::Result;

use crate::{
    Context, Error,
    converters::okizeme::to_okizeme_url,
    matchers::CharacterMoveMatch,
    tekken::{TEKKEN_RED, character::Character, character_move::CharacterMove},
};

async fn reply_with_move_info(
    ctx: Context<'_>,
    move_info: Option<CharacterMoveMatch>,
) -> Result<(), Error> {
    let Some(info) = move_info else {
        ctx.say("No move found".to_string()).await?;
        info!("No move was found");
        return Ok(());
    };

    info!("Found move {}", &info.character_move.id);

    let embed = build_embed_for_move_info(info.character, &info.character_move);
    let reply = poise::CreateReply::default().embed(embed);
    ctx.send(reply).await?;

    Ok(())
}

fn build_embed_for_move_info(character: Character, move_info: &CharacterMove) -> CreateEmbed {
    let mut title = String::new();
    title.push_str(&move_info.id);
    if let Some(move_name) = &move_info.name {
        let decoded_name = decode_move_name(move_name);
        title.push_str(&format!(" ({decoded_name})"));
    }

    CreateEmbed::new()
        .title(title)
        .description(format!(
            "[okizeme.gg]({})",
            to_okizeme_url(character, move_info)
        ))
        .colour(Colour::new(TEKKEN_RED))
        .thumbnail(character.portrait_url())
        .fields(vec![
            (
                "Hit Level",
                move_info.target.as_deref().unwrap_or_default(),
                true,
            ),
            (
                "Damage",
                move_info.damage.as_deref().unwrap_or_default(),
                true,
            ),
            (
                "Startup",
                move_info.startup_frames.as_deref().unwrap_or_default(),
                true,
            ),
        ])
        .fields(vec![
            (
                "On Hit",
                move_info.on_hit.as_deref().unwrap_or_default(),
                true,
            ),
            (
                "On Block",
                move_info.on_block.as_deref().unwrap_or_default(),
                true,
            ),
            (
                "On Counter Hit",
                move_info.on_counter_hit.as_deref().unwrap_or_default(),
                true,
            ),
        ])
        .fields(vec![("Notes", format_notes(&move_info.notes), false)])
}

fn decode_move_name(move_name: &str) -> String {
    let decoded = html_escape::decode_html_entities(move_name);
    Html::parse_fragment(&decoded)
        .root_element()
        .text()
        .collect()
}

fn format_notes(notes: &[String]) -> String {
    notes
        .iter()
        .map(|s| format!("* {s}"))
        .collect::<Vec<String>>()
        .join("\n")
}

async fn character_command_inner(
    ctx: Context<'_>,
    character: Character,
    query: Vec<String>,
) -> Result<(), Error> {
    let move_info = ctx.data().frame_service.query_move(character, &query);
    reply_with_move_info(ctx, move_info).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command)]
pub async fn alisa(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Alisa, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command)]
pub async fn anna(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Anna, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command, aliases("ak"))]
pub async fn armorking(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::ArmorKing, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command)]
pub async fn asuka(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Asuka, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command)]
pub async fn azucena(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Azucena, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command)]
pub async fn bryan(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Bryan, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command)]
pub async fn claudio(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Claudio, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command)]
pub async fn clive(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Clive, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command, aliases("dj"))]
pub async fn deviljin(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::DevilJin, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command, aliases("drag"))]
pub async fn dragunov(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Dragunov, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command)]
pub async fn eddy(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Eddy, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command, aliases("fahk"))]
pub async fn fahkumram(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Fahkumram, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command)]
pub async fn feng(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Feng, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command, aliases("hei"))]
pub async fn heihachi(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Heihachi, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command, aliases("hwo"))]
pub async fn hwoarang(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Hwoarang, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command, aliases("jack"))]
pub async fn jack8(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Jack8, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command)]
pub async fn jin(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Jin, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command)]
pub async fn jun(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Jun, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command, aliases("kaz"))]
pub async fn kazuya(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Kazuya, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command)]
pub async fn king(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::King, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command)]
pub async fn kuma(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Kuma, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command)]
pub async fn lars(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Lars, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command)]
pub async fn law(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Law, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command)]
pub async fn lee(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Lee, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command)]
pub async fn leo(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Leo, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command)]
pub async fn leroy(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Leroy, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command)]
pub async fn lidia(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Lidia, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command)]
pub async fn lili(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Lili, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command)]
pub async fn nina(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Nina, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command, aliases("miary", "mz"))]
pub async fn miaryzo(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::MiaryZo, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command)]
pub async fn panda(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Panda, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command)]
pub async fn paul(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Paul, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command)]
pub async fn raven(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Raven, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command)]
pub async fn reina(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Reina, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command)]
pub async fn shaheen(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Shaheen, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command)]
pub async fn steve(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Steve, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command)]
pub async fn victor(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Victor, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command)]
pub async fn xiaoyu(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Xiaoyu, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command, aliases("yoshi"))]
pub async fn yoshimitsu(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Yoshimitsu, query).await
}

#[instrument(skip(ctx))]
#[command(slash_command, prefix_command)]
pub async fn zafina(
    ctx: Context<'_>,
    #[description = "Move inputs or move name"] query: Vec<String>,
) -> Result<(), Error> {
    character_command_inner(ctx, Character::Zafina, query).await
}
