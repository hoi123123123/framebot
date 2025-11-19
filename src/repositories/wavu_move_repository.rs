use crate::{
    repositories::MoveRepository,
    tekken::{character::Character, character_move::CharacterMove},
};

use anyhow::Result;
use async_trait::async_trait;
use scraper::Html;
use serde::Deserialize;

pub struct WavuMoveRepository;

#[async_trait]
impl MoveRepository for WavuMoveRepository {
    async fn character_moves(&self, character: Character) -> Result<Vec<CharacterMove>> {
        let client = reqwest::Client::new();

        let params = [
            ("action", "cargoquery"),
            ("tables", "Move"),
            ("fields", &QUERY_FIELDS.join(",")),
            ("where", &format!("id LIKE '{character}%'")),
            ("having", ""),
            ("order_by", "id"),
            ("limit", "500"),
            ("format", "json"),
        ];

        let response = client
            .get(WAVU_API_URL)
            .query(&params)
            .send()
            .await?
            .json::<MoveTableQueryResponse>()
            .await?;

        Ok(response
            .cargoquery
            .into_iter()
            .map(|entry| entry.title.into())
            .collect())
    }
}

const WAVU_API_URL: &str = "https://wavu.wiki/w/api.php";

const QUERY_FIELDS: [&str; 20] = [
    "id",
    "num",
    "name",
    "input",
    "alias",
    "alt",
    "parent",
    "target",
    "damage",
    "reach",
    "tracksLeft",
    "tracksRight",
    "startup",
    "recv",
    "tot",
    "crush",
    "block",
    "hit",
    "ch",
    "notes",
];

#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
struct MoveTableQueryResponse {
    cargoquery: Vec<MoveTableResponseEntry>,
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
struct MoveTableResponseEntry {
    title: MoveTableRow,
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MoveTableRow {
    id: String,
    num: Option<String>,
    name: Option<String>,
    input: Option<String>,
    alias: Option<String>,
    alt: Option<String>,
    parent: Option<String>,
    target: Option<String>,
    damage: Option<String>,
    reach: Option<String>,
    tracks_left: Option<String>,
    tracks_right: Option<String>,
    startup: Option<String>,
    recv: Option<String>,
    tot: Option<String>,
    crush: Option<String>,
    block: Option<String>,
    hit: Option<String>,
    ch: Option<String>,
    notes: Option<String>,
}

impl MoveTableRow {
    fn decode_bullet_list(bullet_list_html: &Option<impl AsRef<str>>) -> Vec<String> {
        let Some(html) = bullet_list_html else {
            return vec![];
        };

        let decoded = html_escape::decode_html_entities(html);
        let document = Html::parse_fragment(&decoded);

        document
            .root_element()
            .text()
            .map(|s| s.replace("* ", ""))
            .collect::<String>()
            .lines()
            .filter(|s| !s.trim().is_empty())
            .map(|s| format!("* {s}"))
            .collect::<Vec<String>>()
    }

    fn decode_bullet_list_remove_bullets(
        bullet_list_html: &Option<impl AsRef<str>>,
    ) -> Vec<String> {
        let decoded = MoveTableRow::decode_bullet_list(bullet_list_html);

        decoded
            .into_iter()
            .map(|s| {
                s.strip_prefix("* ")
                    .map(|s_without_prefix| s_without_prefix.to_string())
                    .unwrap_or(s)
            })
            .collect()
    }

    /// Justframe input is shown as "#" on wavu wiki, but in the API you get
    /// "${justFrame}" for some reason
    ///
    /// Returns `Some` with the fixed string if the justframe pattern was replaced,
    /// `None` otherwise
    fn fix_justframe_notation(s: &str) -> Option<String> {
        let justframe_pattern = "${justFrame}";
        if s.contains(justframe_pattern) {
            return Some(s.replace(justframe_pattern, "#"));
        }
        None
    }
}

impl From<MoveTableRow> for CharacterMove {
    fn from(row: MoveTableRow) -> Self {
        let fixed_id = MoveTableRow::fix_justframe_notation(&row.id);
        CharacterMove {
            id: fixed_id.unwrap_or(row.id),
            name: row.name,
            input: row.input,
            alias: MoveTableRow::decode_bullet_list_remove_bullets(&row.alias),
            alt: MoveTableRow::decode_bullet_list_remove_bullets(&row.alt),
            parent: row.parent,
            target: row.target,
            damage: row.damage,
            reach: row.reach,
            startup_frames: row.startup,
            recovery_frames: row.recv,
            total_frames: row.tot,
            crush: row.crush,
            on_block: row.block,
            on_hit: row.hit,
            on_counter_hit: row.ch,
            notes: MoveTableRow::decode_bullet_list(&row.notes),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_justframe_notation() {
        let fixed = MoveTableRow::fix_justframe_notation("Kazuya-f,n,d,df${justFrame}2");
        assert_eq!(fixed.unwrap(), "Kazuya-f,n,d,df#2");
    }

    #[test]
    fn test_replace_justframe_notation_no_justframe_pattern_match() {
        let fixed = MoveTableRow::fix_justframe_notation("Kazuya-f,n,d,df2");
        assert!(fixed.is_none());
    }
}
