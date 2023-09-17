use bevy::{prelude::*, window::PresentMode};

const TITLE: &str = "bv03 Key Events";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::CYAN))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, change_color)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn change_color(input: Res<Input<KeyCode>>, mut clear_color: ResMut<ClearColor>) {
    let key_color = [
        (KeyCode::W, Color::RED),
        (KeyCode::A, Color::GREEN),
        (KeyCode::S, Color::BLUE),
        (KeyCode::D, Color::FUCHSIA),
    ];
    for (k, c) in key_color {
        if input.just_pressed(k) {
            clear_color.0 = c;
        }
    }
}
