//! An 2d-vector of `i32`

use bevy::prelude::*;
use std::collections::HashMap;
use std::ops::{Add, AddAssign};

use crate::globals::{PIECE_Z, TILE_SIZE};
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

impl Position {
    pub const UP: Position = Position { x: 0, y: 1 };
    pub const DOWN: Position = Position { x: 0, y: -1 };
    pub const LEFT: Position = Position { x: -1, y: 0 };
    pub const RIGHT: Position = Position { x: 1, y: 0 };

    pub fn to_world(self) -> Vec3 {
        Vec3::new(
            TILE_SIZE * self.x as f32,
            TILE_SIZE * self.y as f32,
            PIECE_Z,
        )
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
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
