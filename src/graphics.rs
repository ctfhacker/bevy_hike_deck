use bevy::prelude::*;

use crate::assets::AssetList;
use crate::game::{Position, Tile};
use crate::globals::{MIN_DISTANCE, PIECE_SPEED, TILE_SIZE, TILE_Z};
use crate::piece::Piece;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GraphicsWaitEvent>()
            .add_systems(Startup, load_assets)
            .add_systems(Update, spawn_tile_renderer)
            .add_systems(Update, spawn_piece_renderer)
            .add_systems(Update, update_piece_position);
    }
}

#[derive(Event)]
pub struct GraphicsWaitEvent;

#[derive(Debug, Resource)]
pub struct GraphicsAssets {
    pub sprite_textures: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut asset_list: ResMut<AssetList>,
) {
    let texture = asset_server.load("ascii.png");
    asset_list.0.push(texture.clone().untyped());

    let layout = TextureAtlasLayout::from_grid(Vec2::splat(10.), 16, 16, None, None);
    let layout = texture_atlas_layouts.add(layout);
    commands.insert_resource(GraphicsAssets {
        sprite_textures: texture,
        layout,
    });
}

pub fn spawn_tile_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position), Added<Tile>>,
    assets: Res<GraphicsAssets>,
) {
    for (entity, position) in query.iter() {
        let position = Transform::from_translation(Vec3::new(
            TILE_SIZE * position.x as f32,
            TILE_SIZE * position.y as f32,
            TILE_Z,
        ));

        commands.entity(entity).insert(SpriteSheetBundle {
            sprite: Sprite {
                color: Color::OLIVE,
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..default()
            },
            texture: assets.sprite_textures.clone(),
            atlas: TextureAtlas {
                layout: assets.layout.clone(),
                index: 177,
            },
            transform: position,
            ..default()
        });
    }
}

pub fn spawn_piece_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position, &Piece), Added<Piece>>,
    assets: Res<GraphicsAssets>,
) {
    for (entity, position, piece) in query.iter() {
        let sprite_index = match piece {
            Piece::Player => 1,
            Piece::Npc => 63,
        };

        let position = position.to_world();

        commands.entity(entity).insert(SpriteSheetBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..default()
            },
            texture: assets.sprite_textures.clone(),
            atlas: TextureAtlas {
                layout: assets.layout.clone(),
                index: sprite_index,
            },
            transform: Transform::from_translation(position),
            ..default()
        });
    }
}

pub fn update_piece_position(
    mut query: Query<(&Position, &mut Transform), With<Piece>>,
    time: Res<Time>,
    mut event_wait: EventWriter<GraphicsWaitEvent>,
) {
    let mut animating = false;

    for (position, mut transform) in query.iter_mut() {
        let target_pos = position.to_world();
        let distance = (target_pos - transform.translation).length();
        if distance > MIN_DISTANCE {
            transform.translation = transform
                .translation
                .lerp(target_pos, PIECE_SPEED * time.delta_seconds());

            animating = true;
        } else {
            transform.translation = target_pos;
        }
    }

    // Still animating, be sure not to advance in the game state
    if animating {
        event_wait.send(GraphicsWaitEvent);
    }
}
