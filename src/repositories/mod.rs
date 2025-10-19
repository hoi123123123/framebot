use async_trait::async_trait;

use crate::tekken::{character::Character, character_move::CharacterMove};
use anyhow::Result;

pub mod wavu_move_repository;

#[async_trait]
pub trait MoveRepository: Send + Sync {
    async fn character_moves(&self, character: Character) -> Result<Vec<CharacterMove>>;
}
