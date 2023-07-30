use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, greet_people))
        .run();
}

fn hello_world() {
    println!("Hello World!");
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Tom".to_string())));
    commands.spawn((Person, Name("Tim".to_string())));
    commands.spawn((Person, Name("Tum".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hello {}!", name.0);
    }
}
