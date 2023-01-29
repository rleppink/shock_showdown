use std::collections::HashMap;

use crate::prelude::*;

pub fn setup_key_maps(mut commands: Commands) {
    let key_maps = vec![
        PlayerKeyMap {
            player_number: 1,
            key_map: HashMap::from([
                (KeyCode::Up, Action::MoveUp),
                (KeyCode::Left, Action::MoveLeft),
                (KeyCode::Down, Action::MoveDown),
                (KeyCode::Right, Action::MoveRight),
                (KeyCode::Space, Action::PickUpThrow),
            ]),
        },
        PlayerKeyMap {
            player_number: 6,
            key_map: HashMap::from([
                (KeyCode::W, Action::MoveUp),
                (KeyCode::A, Action::MoveLeft),
                (KeyCode::S, Action::MoveDown),
                (KeyCode::D, Action::MoveRight),
                (KeyCode::Tab, Action::PickUpThrow),
            ]),
        },
    ];

    commands.insert_resource(PlayerKeyMaps(key_maps));
}
