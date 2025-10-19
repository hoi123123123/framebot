use anyhow::Result;

use crate::{
    matchers::{CharacterMoveMatch, MoveMatcher},
    move_store::MoveStore,
    repositories::MoveRepository,
    tekken::character::Character,
};

pub struct FrameService<R: MoveRepository, M: MoveMatcher> {
    store: MoveStore<R>,
    matcher: M,
}

impl<R: MoveRepository, M: MoveMatcher> FrameService<R, M> {
    pub async fn try_new(move_repository: R, matcher: M) -> Result<Self> {
        let move_store = MoveStore::try_new(move_repository).await?;
        Ok(Self {
            store: move_store,
            matcher,
        })
    }

    pub fn move_by_id_or_name(
        &self,
        character: Character,
        query: &[String],
    ) -> Option<CharacterMoveMatch> {
        let move_query = query
            .iter()
            .map(|q| q.trim_ascii())
            .collect::<Vec<_>>()
            .join(" ");

        let moves = self.store.moves(character)?;
        let id_match = self.matcher.match_by_id(character, &move_query, &moves)?;

        if id_match.score >= 1f64 {
            return Some(id_match);
        }

        let name_match = self.matcher.match_by_name(character, &move_query, &moves)?;
        if id_match.score >= name_match.score {
            return Some(id_match);
        }

        Some(name_match)
    }
}
