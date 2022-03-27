use bevy::sprite::SpriteBundle;use bevy::prelude::*;

#[derive(Component)]
pub struct PauseGame;

pub fn pause_game(
    mut commands: Commands,
    window: Res<WindowDescriptor>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/pixeled.ttf");

    commands.spawn()
        .insert(PauseGame)
        .insert(Name::new("Pause Screen"))
        .insert(Transform::from_xyz(0., 0., 0.))
        .insert(GlobalTransform::default())
        .insert_bundle(SpriteBundle {
            sprite: Sprite { 
                color: Color::rgba(0., 0., 0., 0.5), 
                custom_size: Some(Vec2::new(window.width, window.height)),
                ..Default::default()
            },
            transform: Transform::from_xyz(
                0.,
                0.,
                10.0,
            ),
            ..Default::default()
        })
        .insert_bundle(Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "Pause".to_string(),
                    style: TextStyle {
                        font,
                        font_size: 70.0,
                        color: Color::ORANGE,
                    },
                }],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            },
            transform: Transform::from_xyz(0., 0., 11.0),
            ..Default::default()
        });
}

pub fn cleanup_pause_game(
    mut commands: Commands,
    query: Query<Entity, With<PauseGame>>
) {
    for screen in query.iter() {
        commands.entity(screen).despawn();
    } 
}
