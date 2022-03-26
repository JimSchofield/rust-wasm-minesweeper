use bevy::prelude::*;
use board_plugin::BoardPlugin;
use board_plugin::resources::BoardOptions;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: "Mine Sweeper".to_string(),
        width: 700.0,
        height: 800.0,
        ..Default::default()
    })
        .add_plugins(DefaultPlugins);

        #[cfg(feature = "debug")]
        app.add_plugin(WorldInspectorPlugin::new());

        app.insert_resource(BoardOptions {
            map_size: (20, 20),
            bomb_count: 40,
            tile_padding: 3.0,
            safe_start: true,
            ..Default::default()
        })
        .add_plugin(BoardPlugin);


    app.add_startup_system(camera_setup);

    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
