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
        let full_query = format!("{character}-{query}").to_lowercase();

        let matched_move = moves
            .iter()
            .map(|m| (jaro(&m.id.to_ascii_lowercase(), &full_query), m))
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
        let full_query = query.to_ascii_lowercase();

        let matched_move = moves
            .iter()
            .filter_map(|m| {
                m.name
                    .as_ref()
                    .map(|name| (jaro(&name.to_ascii_lowercase(), &full_query), m))
            })
            .max_by(|x, y| x.0.total_cmp(&y.0))?;

        Some(CharacterMoveMatch {
            character,
            character_move: matched_move.1.clone(),
            score: matched_move.0,
        })
    }

    fn match_by_alt(
        &self,
        character: Character,
        query: &str,
        moves: &[CharacterMove],
    ) -> Option<CharacterMoveMatch> {
        let full_query = format!("{query}").to_lowercase();

        let matched_move = moves
            .iter()
            .flat_map(|m| m.alt.iter().map(move |alt| (m, alt)))
            .map(|(m, alt)| (jaro(&alt.to_ascii_lowercase(), &full_query), m))
            .max_by(|x, y| x.0.total_cmp(&y.0))?;

        Some(CharacterMoveMatch {
            character,
            character_move: matched_move.1.clone(),
            score: matched_move.0,
        })
    }

    fn match_by_alias(
        &self,
        character: Character,
        query: &str,
        moves: &[CharacterMove],
    ) -> Option<CharacterMoveMatch> {
        let full_query = format!("{query}").to_lowercase();

        let matched_move = moves
            .iter()
            .flat_map(|m| m.alias.iter().map(move |alias| (m, alias)))
            .map(|(m, alias)| (jaro(&alias.to_ascii_lowercase(), &full_query), m))
            .max_by(|x, y| x.0.total_cmp(&y.0))?;

        Some(CharacterMoveMatch {
            character,
            character_move: matched_move.1.clone(),
            score: matched_move.0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Character::Bryan, "1,2,3")]
    #[case(Character::Kazuya, "1,1,2")]
    fn test_exact_id_match(#[case] character: Character, #[case] query: &str) {
        let id_match = JaroMoveMatcher
            .match_by_id(character, query, &sample_moves())
            .unwrap();

        assert_eq!(id_match.character, character);
        assert_eq!(id_match.score, 1f64);
        assert_eq!(id_match.character_move.id, format!("{character}-{query}"));
    }

    #[rstest]
    #[case(Character::Bryan, "One Two Low Kick", "1,2,3")]
    #[case(Character::Bryan, "one two low kick", "1,2,3")]
    #[case(Character::Kazuya, "Flash Punch Combo", "1,1,2")]
    #[case(Character::Kazuya, "flash punch combo", "1,1,2")]
    fn test_exact_name_match(#[case] character: Character, #[case] query: &str, #[case] id: &str) {
        let id_match = JaroMoveMatcher
            .match_by_name(character, query, &sample_moves())
            .unwrap();

        assert_eq!(id_match.character, character);
        assert_eq!(id_match.score, 1f64);
        assert_eq!(id_match.character_move.id, format!("{character}-{id}"));
    }

    #[rstest]
    #[case(Character::Kazuya, "112", "1,1,2")]
    #[case(Character::Bryan, "123", "1,2,3")]
    fn test_id_without_commas_match(
        #[case] character: Character,
        #[case] query: &str,
        #[case] id: &str,
    ) {
        let id_match = JaroMoveMatcher
            .match_by_id(character, query, &sample_moves())
            .unwrap();

        assert_eq!(id_match.character, character);
        assert_eq!(id_match.character_move.id, format!("{character}-{id}"));
    }

    #[test]
    fn test_id_case_insensitive_match() {
        let query = "cs.2";
        let character = Character::Paul;

        let id_match = JaroMoveMatcher
            .match_by_id(character, query, &sample_moves())
            .unwrap();

        assert_eq!(id_match.character, Character::Paul);
        assert_eq!(
            id_match.character_move.id,
            format!("{character}-{}", "CS.2")
        );
    }

    #[test]
    fn test_match_alt() {
        let query = "qcf+2";
        let character = Character::Paul;

        let alt_match = JaroMoveMatcher
            .match_by_alt(character, query, &sample_moves())
            .unwrap();

        assert_eq!(alt_match.character, Character::Paul);
        assert_eq!(
            alt_match.character_move.id,
            format!("{character}-{}", "CS.2")
        );
    }

    #[test]
    fn test_match_alias() {
        let query = "deathfist";
        let character = Character::Paul;

        let alias_match = JaroMoveMatcher
            .match_by_alias(character, query, &sample_moves())
            .unwrap();

        assert_eq!(alias_match.character, Character::Paul);
        assert_eq!(
            alias_match.character_move.id,
            format!("{character}-{}", "CS.2")
        );
    }

    fn sample_moves() -> Vec<CharacterMove> {
        vec![
            CharacterMove {
                id: "Bryan-1,2,3".into(),
                name: Some("One Two Low Kick".into()),
                ..Default::default()
            },
            CharacterMove {
                id: "Bryan-1,2,1".into(),
                name: Some("One Two Body Blow".into()),
                ..Default::default()
            },
            CharacterMove {
                id: "Kazuya-1,1,2".into(),
                name: Some("Flash Punch Combo".into()),
                ..Default::default()
            },
            CharacterMove {
                id: "Kazuya-1,1".into(),
                name: None,
                ..Default::default()
            },
            CharacterMove {
                id: "Paul-2".into(),
                name: Some("Right Jab".into()),
                ..Default::default()
            },
            CharacterMove {
                id: "Paul-CS.2".into(),
                name: Some("Phoenix Smasher".into()),
                alt: vec!["qcf+2".into()],
                alias: vec!["deathfist".into()],
                ..Default::default()
            },
            CharacterMove {
                id: "Paul-qcf".into(),
                name: Some("Cormorant Step".into()),
                ..Default::default()
            },
        ]
    }
}
