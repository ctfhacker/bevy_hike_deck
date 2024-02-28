//! Provides the game states for this game

use bevy::prelude::*;

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq)]
pub enum MainState {
    #[default]
    LoadAssets,
    Game
}
