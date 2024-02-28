//! Provides the Asset management system for this game

use bevy::asset::{LoadState, LoadedAsset};
use bevy::prelude::*;

use crate::states::MainState;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetList>().add_systems(
            Update,
            check_asset_loading.run_if(in_state(MainState::LoadAssets)),
        );
    }
}

#[derive(Default, Resource)]
pub struct AssetList(pub Vec<UntypedHandle>);

pub fn check_asset_loading(
    asset_server: Res<AssetServer>,
    asset_list: Res<AssetList>,
    mut next_state: ResMut<NextState<MainState>>,
) {
    info!("HERE");

    for handle in &asset_list.0 {
        match asset_server.get_load_state(handle) {
            x => {
                dbg!(x);
            }
        }
    }

    next_state.set(MainState::Game);
}
