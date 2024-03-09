use crate::{actions::NextActorEvent, prelude::*};
use bevy::prelude::*;
use rand::prelude::*;

/// Movement for NPCs
#[derive(Component)]
pub struct Walk;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainState::Game), spawn_npcs)
            .add_systems(OnExit(GameState::WaitingForInput), populate_actor_queue)
            .add_systems(Update, plan_walk.run_if(on_event::<NextActorEvent>()));
    }
}

fn spawn_npcs(mut commands: Commands) {
    for (x, y) in [(3, 3), (5, 3)] {
        commands.spawn((Actor::default(), Piece::Npc, Position { x, y }, Walk));
    }
}

fn populate_actor_queue(
    query: Query<Entity, (With<Actor>, Without<Player>)>,
    mut queue: ResMut<ActorQueue>,
) {
    queue.0.extend(query.iter());
}

fn plan_walk(mut query: Query<(&Position, &mut Actor), With<Walk>>, queue: ResMut<ActorQueue>) {
    // Get the current actor at the front of the queue
    let Some(curr_entity) = queue.0.get(0) else {
        return;
    };

    // Get the attributes from the query
    let Ok((curr_position, mut actor)) = query.get_mut(*curr_entity) else {
        return;
    };

    let mut rng = rand::thread_rng();
    let new_direction = [
        Position::UP,
        Position::DOWN,
        Position::LEFT,
        Position::RIGHT,
    ]
    .choose(&mut rng)
    .unwrap();

    let new_action = WalkAction {
        entity: *curr_entity,
        new_position: *curr_position + *new_direction,
    };

    actor.0 = Some(Box::new(new_action));
}
