use bevy::{prelude::*, window::PresentMode};

const TITLE: &str = "bv03 Key Events";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

use bevy::color::palettes::css::{AQUA, BLUE, FUCHSIA, GREEN, RED};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::Srgba(AQUA)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, change_color)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn change_color(input: Res<ButtonInput<KeyCode>>, mut clear_color: ResMut<ClearColor>) {
    let key_color = [
        (KeyCode::KeyW, RED),
        (KeyCode::KeyA, GREEN),
        (KeyCode::KeyS, BLUE),
        (KeyCode::KeyD, FUCHSIA),
    ];
    for (k, c) in key_color {
        if input.just_pressed(k) {
            clear_color.0 = Color::Srgba(c);
        }
    }
}
