use poise::serenity_prelude::{Colour, CreateEmbed};
use scraper::Html;
use tracing::info;

use anyhow::Result;

use crate::{
    Context, Error,
    converters::okizeme::to_okizeme_url,
    matchers::CharacterMoveMatch,
    tekken::{TEKKEN_RED, character::Character, character_move::CharacterMove},
};

macro_rules! define_character_commands {
( $( ($name:ident, $fnname:ident) ),* ) => {
        pub mod generated_character_commands {
            use super::*;
            use crate::{Context, Error};
            use crate::tekken::character::Character;
            use tracing::instrument;
            use poise::command;

            $(
                #[instrument(skip(ctx))]
                #[command(slash_command, prefix_command)]
                pub async fn $fnname(ctx: Context<'_>, query: Vec<String>) -> Result<(), Error> {
                    let character = Character::$name;

                    let move_info = ctx
                        .data()
                        .frame_service
                        .query_move(character, &query);

                    reply_with_move_info(ctx, move_info).await
                }
            )*
        }
    };
}

define_character_commands! {
    (Alisa, alisa),
    (Anna, anna),
    (ArmorKing, armorking),
    (Asuka, asuka),
    (Azucena, azucena),
    (Bryan, bryan),
    (Claudio, claudio),
    (Clive, clive),
    (DevilJin, deviljin),
    (Dragunov, dragunov),
    (Eddy, eddy),
    (Fahkumram, fahkumram),
    (Feng, feng),
    (Heihachi, heihachi),
    (Hwoarang, hwoarang),
    (Jack8, jack8),
    (Jin, jin),
    (Jun, jun),
    (Kazuya, kazuya),
    (King, king),
    (Kuma, kuma),
    (Lars, lars),
    (Law, law),
    (Lee, lee),
    (Leo, leo),
    (Leroy, leroy),
    (Lidia, lidia),
    (Lili, lili),
    (Nina, nina),
    (Panda, panda),
    (Paul, paul),
    (Raven, raven),
    (Reina, reina),
    (Shaheen, shaheen),
    (Steve, steve),
    (Victor, victor),
    (Xiaoyu, xiaoyu),
    (Yoshimitsu, yoshimitsu),
    (Zafina, zafina)
}

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
