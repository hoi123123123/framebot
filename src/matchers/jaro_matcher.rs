use async_trait::async_trait;
use strsim::jaro;

use crate::{
    matchers::{CharacterMoveMatch, MoveMatcher},
    tekken::{character::Character, character_move::CharacterMove},
};

#[derive(Default)]
pub struct JaroMoveMatcher;

#[async_trait]
impl MoveMatcher for JaroMoveMatcher {
    fn match_by_id(
        &self,
        character: Character,
        query: &str,
        moves: &[CharacterMove],
    ) -> Option<CharacterMoveMatch> {
        let full_query = format!("{character}-{query}");

        let matched_move = moves
            .iter()
            .map(|m| (jaro(&m.id, &full_query), m))
            .max_by(|x, y| x.0.total_cmp(&y.0))?;

        Some(CharacterMoveMatch {
            character,
            character_move: matched_move.1.clone(),
            score: matched_move.0,
        })
    }

    fn match_by_name(
        &self,
        character: Character,
        query: &str,
        moves: &[CharacterMove],
    ) -> Option<CharacterMoveMatch> {
        let matched_move = moves
            .iter()
            .filter_map(|m| m.name.as_ref().map(|name| (jaro(name, query), m)))
            .max_by(|x, y| x.0.total_cmp(&y.0))?;

        Some(CharacterMoveMatch {
            character,
            character_move: matched_move.1.clone(),
            score: matched_move.0,
        })
    }
}
