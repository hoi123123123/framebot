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

    pub fn query_move(&self, character: Character, query: &[String]) -> Option<CharacterMoveMatch> {
        let move_query = query
            .iter()
            .map(|q| q.trim_ascii())
            .collect::<Vec<_>>()
            .join(" ");

        let moves = self.store.moves(character)?;

        let id_match = self.matcher.match_by_id(character, &move_query, &moves);
        if let Some(ref m) = id_match
            && m.score >= 1f64
        {
            return id_match;
        }

        let name_match = self.matcher.match_by_name(character, &move_query, &moves);
        if let Some(ref m) = name_match
            && m.score >= 1f64
        {
            return name_match;
        }

        let alt_match = self.matcher.match_by_alt(character, &move_query, &moves);
        if let Some(ref m) = alt_match
            && m.score >= 1f64
        {
            return alt_match;
        }

        let alias_match = self.matcher.match_by_alias(character, &move_query, &moves);
        if let Some(ref m) = alias_match
            && m.score >= 1f64
        {
            return alias_match;
        }

        let best_match = vec![id_match, name_match, alt_match, alias_match]
            .into_iter()
            .filter_map(|x| x)
            .max_by(|x, y| x.score.total_cmp(&y.score))?;

        Some(best_match)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use async_trait::async_trait;
    use rstest::*;

    use crate::tekken::character_move::CharacterMove;

    use super::*;

    #[tokio::test]
    #[rstest]
    #[case(1.0, 0.5, 0.4, 0.3)]
    #[case(0.0, 1.0, 0.4, 0.3)]
    #[case(0.0, 0.5, 1.0, 0.3)]
    #[case(0.0, 0.5, 0.4, 1.0)]
    async fn test_best_match(
        #[case] id_score: f64,
        #[case] name_score: f64,
        #[case] alt_score: f64,
        #[case] alias_score: f64,
    ) {
        let mock_matcher = MockMoveMatcher {
            id_score,
            name_score,
            alt_score,
            alias_score,
        };
        let service = FrameService::try_new(MockMoveRepository, mock_matcher)
            .await
            .unwrap();
        let query = vec!["bla".into()];

        let character_move = service.query_move(Character::Paul, &query).unwrap();

        assert_eq!(character_move.score, 1.0);
    }

    struct MockMoveRepository;

    #[async_trait]
    impl MoveRepository for MockMoveRepository {
        #[allow(unused)]
        async fn character_moves(&self, character: Character) -> Result<Vec<CharacterMove>> {
            Ok(sample_moves())
        }
    }

    struct MockMoveMatcher {
        id_score: f64,
        name_score: f64,
        alt_score: f64,
        alias_score: f64,
    }

    #[allow(unused)]
    impl MoveMatcher for MockMoveMatcher {
        fn match_by_id(
            &self,
            character: Character,
            query: &str,
            moves: &[CharacterMove],
        ) -> Option<CharacterMoveMatch> {
            Some(CharacterMoveMatch {
                character: Character::Alisa,
                score: self.id_score,
                character_move: CharacterMove::default(),
            })
        }

        fn match_by_name(
            &self,
            character: Character,
            query: &str,
            moves: &[CharacterMove],
        ) -> Option<CharacterMoveMatch> {
            Some(CharacterMoveMatch {
                character: Character::Alisa,
                score: self.name_score,
                character_move: CharacterMove::default(),
            })
        }

        fn match_by_alt(
            &self,
            character: Character,
            query: &str,
            moves: &[CharacterMove],
        ) -> Option<CharacterMoveMatch> {
            Some(CharacterMoveMatch {
                character: Character::Alisa,
                score: self.alt_score,
                character_move: CharacterMove::default(),
            })
        }

        fn match_by_alias(
            &self,
            character: Character,
            query: &str,
            moves: &[CharacterMove],
        ) -> Option<CharacterMoveMatch> {
            Some(CharacterMoveMatch {
                character: Character::Alisa,
                score: self.alias_score,
                character_move: CharacterMove::default(),
            })
        }
    }

    fn sample_moves() -> Vec<CharacterMove> {
        vec![CharacterMove {
            ..Default::default()
        }]
    }
}
