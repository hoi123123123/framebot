use crate::tekken::{character::Character, character_move::CharacterMove};

pub fn to_okizeme_url(character: Character, move_info: &CharacterMove) -> String {
    let mut url = String::new();
    url.push_str(OKIZEME_DB_BASE_URL);
    url.push_str(&to_okizeme_name(character));
    url.push('/');
    url.push_str(&to_okizeme_input(character, &move_info.id));

    // Otherwise the link in the discord embed breaks
    url = url.replace(' ', "%20");

    url
}

fn to_okizeme_name(character: Character) -> String {
    match character {
        Character::ArmorKing => "armor-king".into(),
        Character::DevilJin => "devil-jin".into(),
        _ => character.to_string().to_lowercase(),
    }
}

fn to_okizeme_input(character: Character, move_id: &str) -> String {
    let mut name_part = character.to_string();
    name_part.push('-');

    let move_part = &move_id[name_part.len()..];
    move_part.into()
}

const OKIZEME_DB_BASE_URL: &str = "https://okizeme.gg/database/";
