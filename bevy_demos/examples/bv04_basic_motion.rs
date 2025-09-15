use bevy::{prelude::*, window::PresentMode};

const TITLE: &str = "bv04 Basic Motion";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;
const PLAYER_SIZE: f32 = 32.;

#[derive(Component)]
struct Player;

use bevy::color::palettes::css::SEA_GREEN;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::Srgba(Srgba::gray(0.25))))
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
        .add_systems(Update, move_player)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_color(SEA_GREEN, Vec2::splat(PLAYER_SIZE)),
        Player,
    ));
}

fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    mut player_transform: Single<&mut Transform, With<Player>>,
) {
    let mut vel = Vec3::ZERO;

    if input.pressed(KeyCode::KeyA) {
        vel.x -= 1.;
    }

    if input.pressed(KeyCode::KeyD) {
        vel.x += 1.;
    }

    if input.pressed(KeyCode::KeyW) {
        vel.y += 1.;
    }

    if input.pressed(KeyCode::KeyS) {
        vel.y -= 1.;
    }

    player_transform.translation += vel;
}

/* TODO:
 * Can we slowly ramp up to speed limit instead of max accel?
 * What about different refresh rates?
 * How do we stay inside the window?
 * How do we avoid speeding up along the diagonal?
 */
