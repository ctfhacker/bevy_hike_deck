use bevy::prelude::*;

use std::collections::VecDeque;

use crate::actions::{ActorQueue, WalkAction};
use crate::game::Position;
use crate::piece::Actor;
use crate::player::Player;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        /*
        app.add_event::<PlayerInputReadyEvent>().add_systems(
            Update,
            player_position.run_if(in_state(GameState::WaitingForInput)),
        );
        */
        app.add_event::<PlayerInputReadyEvent>()
            .add_systems(Update, player_position);
    }
}

#[derive(Debug, Event)]
pub struct PlayerInputReadyEvent;

const WASD_KEYS: &[(KeyCode, Position)] = &[
    (KeyCode::KeyW, Position::UP),
    (KeyCode::KeyA, Position::LEFT),
    (KeyCode::KeyS, Position::DOWN),
    (KeyCode::KeyD, Position::RIGHT),
];

fn player_position(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(Entity, &Position, &mut Actor), With<Player>>,
    mut queue: ResMut<ActorQueue>,
    mut event_input: EventWriter<PlayerInputReadyEvent>,
) {
    let Ok((entity, position, mut actor)) = player_query.get_single_mut() else {
        return;
    };

    for (key, dir) in WASD_KEYS {
        if !keys.just_pressed(*key) {
            continue;
        }

        let action = WalkAction {
            entity,
            new_position: *position + *dir,
        };

        // Add this moved actor to the action queue
        actor.0 = Some(Box::new(action));
        queue.0 = VecDeque::from([entity]);
        event_input.send(PlayerInputReadyEvent);
    }
}
