use crate::prelude::*;
use bevy::prelude::*;

/// Movement for NPCs
#[derive(Component)]
pub struct Walk;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainState::Game), spawn_npcs);
    }
}

fn spawn_npcs(mut commands: Commands) {
    for (x, y) in [(3, 3), (5, 3)] {
        commands.spawn((Actor::default(), Piece::Npc, Position { x, y }, Walk));
    }
}
