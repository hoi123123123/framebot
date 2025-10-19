use std::collections::HashMap;

use crate::{
    repositories::MoveRepository,
    tekken::{character::Character, character_move::CharacterMove},
};
use anyhow::Result;
use futures::future::join_all;
use strum::IntoEnumIterator;

type MoveMap = HashMap<Character, Vec<CharacterMove>>;

#[derive(Debug)]
pub struct MoveStore<M: MoveRepository> {
    move_repository: M,
    moves: MoveMap,
}

impl<M: MoveRepository> MoveStore<M> {
    pub async fn try_new(move_repository: M) -> Result<Self> {
        let moves = Self::create_new_move_map(&move_repository).await?;

        Ok(Self {
            move_repository,
            moves,
        })
    }

    pub async fn refresh_moves(&mut self) -> Result<()> {
        let move_map = Self::create_new_move_map(&self.move_repository).await?;
        self.moves = move_map;
        Ok(())
    }

    async fn create_new_move_map(move_repository: &M) -> Result<MoveMap> {
        let futures: Vec<_> = Character::iter()
            .map(|character| async move {
                let result = move_repository.character_moves(character).await;
                (character, result)
            })
            .collect();

        let mut move_map = HashMap::new();
        let results = join_all(futures).await;

        for (character, result) in results {
            let move_data = result?;
            move_map.insert(character, move_data);
        }

        Ok(move_map)
    }

    // TODO: No cloning
    pub fn moves(&self, character: Character) -> Option<Vec<CharacterMove>> {
        self.moves.get(&character).cloned()
    }
}
