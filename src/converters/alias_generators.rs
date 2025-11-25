use std::borrow::Cow;

use crate::tekken::character::Character;

// Noone writes the first plus after a letter when referring to a move.
// For example, d+2 is referred to as d1, and d+1+2 as d1+2
pub fn drop_first_plus_after_letter(character: Character, move_id: &str) -> Cow<'_, str> {
    if move_id.len() <= 1 {
        return Cow::Borrowed(move_id);
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

    Cow::Owned(new_alias)
}

/// Replace f,f with ff, and f,f,f with fff
pub fn remove_commas_from_ff_notation(move_id: &str) -> Cow<'_, str> {
    let lowercased = move_id.to_lowercase();

    if !lowercased.contains("f,f") {
        return Cow::Borrowed(move_id);
    }

    let replaced_fff = lowercased.replace("f,f,f", "fff");
    let replaced_ff = replaced_fff.replace("f,f", "ff");

    if replaced_ff != lowercased {
        Cow::Owned(replaced_ff)
    } else {
        Cow::Borrowed(move_id)
    }
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
        let alias = drop_first_plus_after_letter(character, move_id);
        assert_eq!(expected_alias, alias);
    }

    #[rstest]
    #[case("f,F+1+2", "ff+1+2")]
    #[case("f,f,f+3", "fff+3")]
    #[case("f,f3", "ff3")]
    #[case("f,f,f,f,f,f", "ffffff")]
    #[case("f,f,f,f,f,f", "ffffff")]
    #[case("f,F+1+2,1+2", "ff+1+2,1+2")]

    fn test_fix_ff_notation(#[case] s: &str, #[case] expected: &str) {
        let alias = remove_commas_from_ff_notation(s);
        assert_eq!(expected, alias);
    }
}
