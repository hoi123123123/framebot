use crate::tekken::{character::Character, character_move::CharacterMove};

// Noone writes the first plus after a letter when referring to a move.
// For example, d+2 is referred to as d1, and d+1+2 as d1+2
pub fn drop_first_plus_after_letter(character: Character, character_move: &mut CharacterMove) {
    let move_id = &character_move.id;

    if move_id.len() <= 1 {
        return;
    }

    // Drop the character name, alias matchers don't include the
    // character name when comparing the query
    let chars = {
        // TODO: The character names are pretty Wavu specific, if another data source
        //       does not follow the same naming scheme this would break.
        //       Maybe this logic should be moved to the Wavu repo instead
        let character_name_prefix = character.to_string() + "-";

        let without_prefix = match move_id.strip_prefix(&character_name_prefix) {
            Some(s) => s,
            None => move_id,
        };

        without_prefix.chars().collect::<Vec<char>>()
    };

    let mut prev_char = chars[0];
    let mut plus_skipped = false;
    let mut new_alias = String::with_capacity(chars.len());

    for ch in chars {
        if !plus_skipped && ch == '+' && prev_char.is_ascii_lowercase() {
            plus_skipped = true;
            continue;
        }
        prev_char = ch;
        new_alias.push(ch);
    }

    character_move.alias.push(new_alias);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("d+2", "d2", Character::Anna)]
    #[case("d++2", "d+2", Character::Anna)]
    #[case("d+++2", "d++2", Character::Anna)]
    #[case("d+1+2", "d1+2", Character::Anna)]
    #[case("d+2,d+2", "d2,d+2", Character::Anna)]
    #[case("ff+1+2", "ff1+2", Character::Anna)]
    #[case("b1+2", "b1+2", Character::Anna)]
    #[case("B+1+2", "B+1+2", Character::Anna)]
    #[case("uf+1", "uf1", Character::Anna)]
    #[case("Alisa-d+3+4", "d3+4", Character::Alisa)]
    #[case("Jack-8-d+1+2", "d1+2", Character::Jack8)]
    fn test_drop_first_plus_after_letter(
        #[case] move_id: &str,
        #[case] expected_alias: &str,
        #[case] character: Character,
    ) {
        let mut character_move = CharacterMove {
            id: move_id.into(),
            ..Default::default()
        };

        drop_first_plus_after_letter(character, &mut character_move);

        let alias = &character_move.alias[0];
        assert_eq!(expected_alias, alias);
    }
}
