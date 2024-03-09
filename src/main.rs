use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod actions;
mod assets;
mod game;
mod globals;
mod graphics;
mod input;
mod manager;
mod npcs;
mod piece;
mod player;
mod prelude;
mod states;

fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.transform.translation = Vec3::new(
        4. * globals::TILE_SIZE,
        4. * globals::TILE_SIZE,
        camera.transform.translation.z,
    );
    camera.transform.scale = Vec3::new(0.6, 0.6, 1.0);
    commands.spawn(camera);
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = App::new();
    app.add_plugins(
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
    .init_state::<states::GameState>()
    .add_systems(Startup, setup_camera)
    .add_plugins(game::BoardPlugin)
    .add_plugins(assets::Plugin)
    .add_plugins(graphics::Plugin)
    .add_plugins(player::Plugin)
    .add_plugins(input::Plugin)
    .add_plugins(actions::Plugin)
    .add_plugins(manager::Plugin)
    .add_plugins(npcs::Plugin);

    /*
    app.add_plugins(
        WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
    );
    */

    #[cfg(feature = "flow")]
    bevy_mod_debugdump::print_schedule_graph(&mut app, OnExit(states::GameState::WaitingForInput));

    #[cfg(not(feature = "flow"))]
    app.run();
}
