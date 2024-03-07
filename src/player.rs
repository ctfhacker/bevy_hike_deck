use bevy::prelude::*;

use crate::game::Position;
use crate::piece::Piece;
use crate::states::MainState;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainState::Game), spawn_player);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands) {
    commands.spawn((Player, Piece::Player, Position { x: 0, y: 0 }));
}
