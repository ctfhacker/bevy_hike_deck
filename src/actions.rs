use bevy::prelude::*;

use std::collections::VecDeque;

use crate::game::{Board, Position};
use crate::piece::Actor;
use crate::player::Player;

pub trait Action: Send + Sync {
    /// Execute the current action on the world. Returns `true` if success, `false` otherwise
    fn execute(&self, world: &mut World) -> bool;
}

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActorQueue>()
            .add_event::<TickEvent>()
            .add_event::<NextActorEvent>()
            .add_event::<ActionsCompleteEvent>()
            .add_event::<InvalidPlayerActionEvent>()
            .add_systems(Update, process_action_queue.run_if(on_event::<TickEvent>()));
    }
}

#[derive(Default, Resource, Debug)]
pub struct ActorQueue(pub VecDeque<Entity>);

#[derive(Event)]
pub struct TickEvent;

#[derive(Event)]
pub struct NextActorEvent;

#[derive(Event)]
pub struct ActionsCompleteEvent;

#[derive(Event)]
pub struct InvalidPlayerActionEvent;

pub fn process_action_queue(world: &mut World) {
    // Attempt to get mut access to the actor queue
    let Some(mut queue) = world.get_resource_mut::<ActorQueue>() else {
        return;
    };

    // Check for the next event, otherwise tell everyone that there are no more actions
    let Some(entity) = queue.0.pop_front() else {
        world.send_event(ActionsCompleteEvent);
        return;
    };

    // Ensure the entity in the queue is `impl Actor`
    let Some(mut actor) = world.get_mut::<Actor>(entity) else {
        return;
    };

    // Get the action to execute on the world from the event
    let Some(action) = actor.0.take() else {
        return;
    };

    let successful_action = !action.execute(world);
    let is_player = world.get::<Player>(entity).is_some();

    // If the entity performing an Action is the Player and the action failed, send an InvalidPlayerActionEvent
    if is_player && !successful_action {
        world.send_event(InvalidPlayerActionEvent);
        return;
    }

    // Finished this action, proceed to the next one
    world.send_event(NextActorEvent);
}

/// A movement of the given [`Entity`] to the given [`Position`]
#[derive(Debug)]
pub struct WalkAction {
    pub entity: Entity,
    pub new_position: Position,
}

impl Action for WalkAction {
    fn execute(&self, world: &mut World) -> bool {
        // Get the game board
        let Some(board) = world.get_resource::<Board>() else {
            return false;
        };

        // If the requested new position does not exist on the board, do not attempt to walk there
        if !board.tiles.contains_key(&self.new_position) {
            return false;
        }

        // Get the position of the given entity
        let Some(mut position) = world.get_mut::<Position>(self.entity) else {
            return false;
        };

        // Found the position for this entity, update it with the new position
        *position = self.new_position;

        true
    }
}
