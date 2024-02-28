//! An 2d-vector of `i32`

use bevy::prelude::*;
use std::collections::HashMap;

use crate::states::MainState;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct IntVec2 {
    x: i32,
    y: i32,
}

#[derive(Component, Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Tile;

#[derive(Debug, Default, Resource)]
pub struct Board {
    pub tiles: HashMap<Position, Entity>,
}

// Initialize the game map when entering the [`MainState::Game`] state
pub fn spawn_map(mut commands: Commands, mut board: ResMut<Board>) {
    board.tiles = HashMap::new();
    for x in 0..8 {
        for y in 0..8 {
            let pos = Position { x, y };
            let tile_id = commands.spawn((pos, Tile)).id();

            dbg!(tile_id);
            board.tiles.insert(pos, tile_id);
        }
    }
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>()
            .add_systems(OnEnter(MainState::Game), spawn_map);
    }
}
