use bevy::prelude::*;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .add_systems(Update, player_movement)
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((Player, SpriteBundle {
        texture: asset_server.load("player-sprite.png"),
        transform: Transform::from_xyz(0.0, -200.0, 0.0),
        ..default()
    }));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("enemy-sprite.png"),
        transform: Transform::from_xyz(0.0, 200.0, 0.0),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture:asset_server.load("ball-sprite.png"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

fn player_movement(input: Res<ButtonInput<KeyCode>>, mut player: Query<(&mut Transform, &mut Player)>) {
    for (mut transform, mut _p) in &mut player {
        let direction = transform.local_x();
        if input.pressed(KeyCode::KeyA) {
            transform.translation += direction * -1.0;
        }
        if input.pressed(KeyCode::KeyD) {
            transform.translation += direction * 1.0;
        }
    }
}
