use bevy::prelude::*;

use crate::actions::{ActionsCompleteEvent, InvalidPlayerActionEvent, TickEvent};
use crate::graphics::GraphicsWaitEvent;
use crate::input::PlayerInputReadyEvent;
use crate::states::{GameState, MainState};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainState::Game), game_start)
            .add_systems(OnExit(MainState::Game), game_end)
            .add_systems(
                Update,
                (
                    turn_update_start.run_if(on_event::<PlayerInputReadyEvent>()),
                    turn_update_end.run_if(on_event::<ActionsCompleteEvent>()),
                    turn_update_cancel.run_if(on_event::<InvalidPlayerActionEvent>()),
                    tick,
                ),
            );
    }
}

fn game_start(mut next_state: ResMut<NextState<GameState>>) {
    info!("GAME START!");
    next_state.set(GameState::WaitingForInput);
}

fn game_end(mut next_state: ResMut<NextState<GameState>>) {
    info!("GAME END!");
    next_state.set(GameState::None);
}

fn tick(event_wait: EventReader<GraphicsWaitEvent>, mut event_tick: EventWriter<TickEvent>) {
    if event_wait.is_empty() {
        event_tick.send(TickEvent);
    }
}

fn turn_update_start(
    mut next_state: ResMut<NextState<GameState>>,
    mut event_tick: EventWriter<TickEvent>,
) {
    next_state.set(GameState::TurnUpdate);
    event_tick.send(TickEvent);
}

fn turn_update_end(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::WaitingForInput);
}

fn turn_update_cancel(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::WaitingForInput);
}
