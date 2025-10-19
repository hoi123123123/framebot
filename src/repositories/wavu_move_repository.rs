use crate::{
    repositories::MoveRepository,
    tekken::{character::Character, character_move::CharacterMove},
};

use anyhow::Result;
use async_trait::async_trait;
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

impl From<MoveTableRow> for CharacterMove {
    fn from(row: MoveTableRow) -> Self {
        CharacterMove {
            id: row.id,
            name: row.name,
            input: row.input,
            alias: row.alias,
            alt: row.alt,
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
            notes: row.notes,
        }
    }
}
