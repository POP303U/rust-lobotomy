use bevy::prelude::*;
use bevy::window::PresentMode;

pub const CLEAR: Color = Color::rgba(0.1, 0.1, 0.1, 0.1);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "I am a window!".into(),
                resolution: (500., 300.).into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(CLEAR))
        .add_systems(Startup, spawn_cam)
        .add_systems(Startup, spawn_sprite)
        .add_systems(Startup, spawn_audio)
        .run();
}

fn spawn_cam(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("lobotomy.png"),
        ..default()
    });
}

fn spawn_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(AudioBundle {
        source: asset_server.load("lobotomy-dash.ogg"),
        ..default()
    });
}
