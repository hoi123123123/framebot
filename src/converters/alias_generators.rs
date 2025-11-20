use crate::tekken::character::Character;

// Noone writes the first plus after a letter when referring to a move.
// For example, d+2 is referred to as d1, and d+1+2 as d1+2
pub fn drop_first_plus_after_letter(character: Character, move_id: &str) -> Option<String> {
    if move_id.len() <= 1 {
        return None;
    }

    // Drop the character name, alias matchers don't include the
    // character name when comparing the query
    let chars = {
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
        if !plus_skipped && ch == '+' && prev_char.is_ascii_alphabetic() {
            plus_skipped = true;
            continue;
        }
        prev_char = ch;
        new_alias.push(ch);
    }

    Some(new_alias)
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
    #[case("B+1+2", "B1+2", Character::Anna)]
    #[case("uf+1", "uf1", Character::Anna)]
    #[case("Alisa-d+3+4", "d3+4", Character::Alisa)]
    #[case("Jack-8-d+1+2", "d1+2", Character::Jack8)]
    #[case("Jack-8-f,F+1", "f,F1", Character::Jack8)]
    fn test_drop_first_plus_after_letter(
        #[case] move_id: &str,
        #[case] expected_alias: &str,
        #[case] character: Character,
    ) {
        let alias = drop_first_plus_after_letter(character, move_id).unwrap();
        assert_eq!(expected_alias, alias);
    }
}
