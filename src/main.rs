use bevy::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Wall;

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
    commands.spawn((Ball, SpriteBundle {
        texture:asset_server.load("ball-sprite.png"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    }));
    // may want to separate the wall sprites if collision becomes too difficult to figure out with
    // the "zone" being one total sprite
    commands.spawn((Wall, SpriteBundle {
        texture:asset_server.load("left-wall.png"),
        ..default()
    }));
    commands.spawn((Wall, SpriteBundle {
        texture:asset_server.load("right-wall.png"),
        ..default()
    }));
}

fn player_movement(input: Res<ButtonInput<KeyCode>>, mut player: Query<(&mut Transform, &mut Player)>) {
    for (mut transform, mut _p) in &mut player {
        let direction = transform.local_x();
        if input.pressed(KeyCode::KeyA) {
            transform.translation += direction * -2.0;
        }
        if input.pressed(KeyCode::KeyD) {
            transform.translation += direction * 2.0;
        }
    }
}

// need to create zone wall system here
