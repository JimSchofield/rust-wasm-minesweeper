use bevy::prelude::*;
use bevy::log;
use board_plugin::BoardPlugin;
use board_plugin::resources::{BoardAssets, SpriteMaterial};
use board_plugin::resources::BoardOptions;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    Paused,
    Out,
}

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

    app.add_plugin(BoardPlugin {
        running_state: AppState::InGame,
    })
        .add_state(AppState::Out)
        .add_startup_system(setup_board)
        .add_startup_system(camera_setup)
        .add_system(state_handler);

    app.run();
}

fn setup_board(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    asset_server: Res<AssetServer>,
) {
    // Board plugin options
    commands.insert_resource(BoardOptions {
        map_size: (20, 20),
        bomb_count: 40,
        tile_padding: 1.,
        safe_start: true,
        ..Default::default()
    });

    // Board assets
    commands.insert_resource(BoardAssets {
        label: "Default".to_string(),
        board_material: SpriteMaterial {
            color: Color::WHITE,
            ..Default::default()
        },
        tile_material: SpriteMaterial {
            color: Color::DARK_GRAY,
            ..Default::default()
        },
        covered_tile_material: SpriteMaterial {
            color: Color::GRAY,
            ..Default::default()
        },
        bomb_counter_font: asset_server.load("fonts/pixeled.ttf"),
        bomb_counter_colors: BoardAssets::default_colors(),
        flag_material: SpriteMaterial {
            texture: asset_server.load("sprites/flag.png"),
            color: Color::WHITE,
        },
        bomb_material: SpriteMaterial {
            texture: asset_server.load("sprites/bomb.png"),
            color: Color::WHITE,
        },
    });

    // Plugin activation
    state.set(AppState::InGame).unwrap();
}

fn state_handler(
    mut state: ResMut<State<AppState>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        log::debug!("pause detected");
        if state.current() == &AppState::InGame {
            log::info!("pausing");
            state.push(AppState::Paused).unwrap();
        }
        if state.current() == &AppState::Paused {
            log::info!("resuming");
            state.pop().unwrap();
        }
    }
    if keys.just_pressed(KeyCode::C) {
        log::debug!("clearing detected!");
        if state.current() == &AppState::InGame {
            log::info!("clearing game");
            state.set(AppState::Out).unwrap();
        }
    }
    if keys.just_pressed(KeyCode::G) {
        log::debug!("loading detected!");
        if state.current() == &AppState::Out {
            log::info!("loading game");
            state.set(AppState::InGame).unwrap();
        }
    }
}


fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
