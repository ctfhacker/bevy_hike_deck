use bevy::prelude::*;

mod assets;
mod game;
mod globals;
mod graphics;
mod input;
mod piece;
mod player;
mod states;

fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.transform.translation = Vec3::new(
        4. * globals::TILE_SIZE,
        4. * globals::TILE_SIZE,
        camera.transform.translation.z,
    );
    commands.spawn(camera);
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (globals::WINDOW_WIDTH, globals::WINDOW_HEIGHT).into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(Msaa::Off)
        .init_state::<states::MainState>()
        .add_systems(Startup, setup_camera)
        .add_plugins(game::BoardPlugin)
        .add_plugins(assets::Plugin)
        .add_plugins(graphics::Plugin)
        .add_plugins(player::Plugin)
        .add_plugins(input::Plugin)
        .run();
}
