use bevy::prelude::*;

use crate::game::Position;
use crate::player::Player;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_position);
    }
}

const WASD_KEYS: &[(KeyCode, Position)] = &[
    (KeyCode::KeyW, Position::UP),
    (KeyCode::KeyA, Position::LEFT),
    (KeyCode::KeyS, Position::DOWN),
    (KeyCode::KeyD, Position::RIGHT),
];

fn player_position(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Position, With<Player>>,
) {
    let Ok(mut position) = player_query.get_single_mut() else {
        return;
    };

    for (key, dir) in WASD_KEYS {
        if !keys.just_pressed(*key) {
            continue;
        }

        position.x += dir.x;
        position.y += dir.y;
    }
}
