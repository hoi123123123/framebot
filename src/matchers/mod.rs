use async_trait::async_trait;

use crate::tekken::{character::Character, character_move::CharacterMove};

pub mod jaro_matcher;

#[async_trait]
pub trait MoveMatcher: Send + Sync {
    fn match_by_id(
        &self,
        character: Character,
        query: &str,
        moves: &[CharacterMove],
    ) -> Option<CharacterMoveMatch>;

    fn match_by_name(
        &self,
        character: Character,
        query: &str,
        moves: &[CharacterMove],
    ) -> Option<CharacterMoveMatch>;
}

#[derive(Debug, PartialEq)]
pub struct CharacterMoveMatch {
    pub character: Character,
    pub character_move: CharacterMove,
    pub score: f64,
}
