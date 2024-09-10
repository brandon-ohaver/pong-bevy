# pong-bevy


## TODOS

* Need to create repo DONE
* Need to make bevy dependency DONE
* Need to create player/opponent sprite DONE
* Need to create ball sprite DONE
* Need to setup starting position of player/enemy/ball
* Need to make controls for player
* Need to make ball move/learn game physics?
* Need to create collision with player/opponent sprites
* Need to make "walls" for ball to hit so it doesn't leave screen
* Need to opponent movement/AI
* Need to add scoring/point tracking
* Need to add spawn/despawn logic for the ball when it is scored
* Need to add sound

## ECS Example
```
use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(HelloPlugin)
    .run();
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

// "iterate over every Name component for entities that also have a Person component"
fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {        
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break;
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (update_people, greet_people).chain());
    }
}

```
