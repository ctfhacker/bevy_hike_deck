use bevy::prelude::*;

use crate::actions::Action;

#[derive(Component)]
pub enum Piece {
    Player,
}

#[derive(Component, Default)]
pub struct Actor(pub Option<Box<dyn Action>>);
