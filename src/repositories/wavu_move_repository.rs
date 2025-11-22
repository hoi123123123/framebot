use crate::{
    converters::alias_generators,
    repositories::MoveRepository,
    tekken::{character::Character, character_move::CharacterMove},
};

use anyhow::Result;
use async_trait::async_trait;
use regex::Regex;
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

        let mut character_moves = response
            .cargoquery
            .into_iter()
            .map(|entry| entry.title.into())
            .collect::<Vec<CharacterMove>>();

        // Add aliases to increase the chance of finding the moves people actually intend to see
        for m in character_moves.iter_mut() {
            if let Some(alias) = alias_generators::drop_first_plus_after_letter(character, &m.id) {
                m.alias.push(alias);
            }
        }

        Ok(character_moves)
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
            return Vec::new();
        };

        let decoded = html_escape::decode_html_entities(html);

        Html::parse_fragment(&decoded)
            .root_element()
            .text()
            .flat_map(|s| s.lines())
            .map(MoveTableRow::remove_links)
            .map(|s| s.replace("* ", ""))
            .filter(|s| !s.trim().is_empty())
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

    /// Frame data fields sometimes contain links such as "[[Eddy combos#Staples|+31a(+24)]]",
    /// this function removes those kinds of links so we get "+31a(+24)" instead
    fn remove_links(s: &str) -> String {
        let re = Regex::new(r"\[\[.*?\|(.*?)\]\]").unwrap();
        re.replace(s, "$1").into()
    }
}

impl From<MoveTableRow> for CharacterMove {
    fn from(row: MoveTableRow) -> Self {
        let fixed_id = MoveTableRow::fix_justframe_notation(&row.id);

        CharacterMove {
            id: fixed_id.unwrap_or(row.id),
            name: row.name,
            input: row.input,
            alias: MoveTableRow::decode_bullet_list(&row.alias),
            alt: MoveTableRow::decode_bullet_list(&row.alt),
            parent: row.parent,
            target: row.target,
            damage: row.damage,
            reach: row.reach,
            startup_frames: row.startup,
            recovery_frames: row.recv,
            total_frames: row.tot,
            crush: row.crush,
            on_block: row.block.map(|s| MoveTableRow::remove_links(&s)),
            on_hit: row.hit.map(|s| MoveTableRow::remove_links(&s)),
            on_counter_hit: row.ch.map(|s| MoveTableRow::remove_links(&s)),
            notes: MoveTableRow::decode_bullet_list(&row.notes),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

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

    #[rstest]
    #[case("[[Eddy combos#Staples|+31a(+24)]]", "+31a(+24)")]
    #[case("[[Asuka_combos#Staples|+22a (+12)]]", "+22a (+12)")]
    #[case("+23a (+13)", "+23a (+13)")]
    #[case("+2", "+2")]
    #[case("", "")]
    #[case("[[Asuka_combos#Staples|]]", "")]
    #[case("[[Paul_movelist#Paul-H.CS.2|H.CS.2]] with Heat", "H.CS.2 with Heat")]
    #[case(
        "Hi [[Paul_movelist#Paul-H.CS.2|H.CS.2]] with Heat",
        "Hi H.CS.2 with Heat"
    )]
    fn test_remove_links(#[case] with_link: &str, #[case] without_link: &str) {
        let s = MoveTableRow::remove_links(with_link);
        assert_eq!(without_link, s)
    }
}
