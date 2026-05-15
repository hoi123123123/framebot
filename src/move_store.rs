use std::{collections::HashMap, sync::Arc};

use crate::{
    repositories::MoveRepository,
    tekken::{character::Character, character_move::CharacterMove},
};
use anyhow::Result;
use strum::IntoEnumIterator;

type MoveMap = HashMap<Character, Vec<CharacterMove>>;

#[derive(Debug)]
pub struct MoveStore<M: MoveRepository> {
    move_repository: Arc<M>,
    moves: MoveMap,
}

impl<M: MoveRepository + Send + Sync + 'static> MoveStore<M> {
    pub async fn try_new(move_repository: M) -> Result<Self> {
        let move_repository = Arc::new(move_repository);
        let moves = Self::create_new_move_map(&move_repository).await?;
        Ok(Self {
            move_repository,
            moves,
        })
    }

    pub async fn refresh_moves(&mut self) -> Result<()> {
        self.moves = Self::create_new_move_map(&self.move_repository).await?;
        Ok(())
    }

    async fn create_new_move_map(move_repository: &Arc<M>) -> Result<MoveMap> {
        let handles: Vec<_> = Character::iter()
            .map(|character| {
                let repo = Arc::clone(move_repository);
                tokio::spawn(async move {
                    let result = repo.character_moves(character).await;
                    (character, result)
                })
            })
            .collect();

        let mut move_map = HashMap::new();
        for handle in handles {
            let (character, result) = handle.await?;
            move_map.insert(character, result?);
        }

        Ok(move_map)
    }

    pub fn moves(&self, character: Character) -> Option<Vec<CharacterMove>> {
        self.moves.get(&character).cloned()
    }
}
