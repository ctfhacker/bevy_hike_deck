use bevy::prelude::*;
use bevy::render::texture::ImageSampler;

use crate::assets::AssetList;
use crate::game::{Position, Tile};
use crate::globals::TILE_SIZE;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_assets)
            .add_systems(Update, spawn_tile_renderer);
    }
}

#[derive(Resource)]
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
    let mut texture = asset_server.load("ascii.png");
    asset_list.0.push(texture.clone().untyped());

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(10), 16, 16, None, None);
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
            0.0,
        ));

        commands.entity(entity).insert(SpriteSheetBundle {
            sprite: Sprite {
                color: LegacyColor::OLIVE,
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
