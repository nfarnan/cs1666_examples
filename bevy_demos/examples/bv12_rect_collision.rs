use bevy::{prelude::*, window::PresentMode};
use std::convert::From;

use bevy::color::palettes::css::{CRIMSON, SEA_GREEN};

const TITLE: &str = "bv05 Rect Collision";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;
const PLAYER_SIZE: f32 = 32.;
// 5px/frame @60Hz == 300px/s
const PLAYER_SPEED: f32 = 300.;
// 1px/frame^2 @60Hz == 3600px/s^2
const ACCEL_RATE: f32 = 3600.;

#[derive(Component)]
struct Player;

#[derive(Component, Deref, DerefMut)]
struct Velocity {
    velocity: Vec2,
}

impl Velocity {
    fn new() -> Self {
        Self {
            velocity: Vec2::splat(0.),
        }
    }
}

#[derive(Component)]
struct Block;

struct Sides {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
}

impl From<Vec3> for Sides {
    fn from(pos: Vec3) -> Self {
        Self {
            top: pos.y + PLAYER_SIZE / 2.,
            bottom: pos.y - PLAYER_SIZE / 2.,
            left: pos.x - PLAYER_SIZE / 2.,
            right: pos.x + PLAYER_SIZE / 2.,
        }
    }
}

// Don't bother using this, use bevy::math::bounding
fn my_collision(a_pos: Vec3, b_pos: Vec3) -> bool {
    let a: Sides = a_pos.into();
    let b: Sides = b_pos.into();

    if a.bottom > b.top || a.top < b.bottom || a.right < b.left || a.left > b.right {
        false
    } else {
        true
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::Srgba(Srgba::gray(0.25))))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                resolution: (WIN_W as u32, WIN_H as u32).into(),
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
        Velocity::new(),
    ));

    commands.spawn((
        Sprite::from_color(CRIMSON, Vec2::splat(PLAYER_SIZE)),
        Transform {
            translation: Vec3::new(WIN_W / 4., 0., 0.),
            ..default()
        },
        Block,
        Velocity::new(),
    ));
}

fn move_player(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    player: Single<(&mut Transform, &mut Velocity), With<Player>>,
    block: Single<&Transform, (With<Block>, Without<Player>)>,
) {
    let (mut transform, mut velocity) = player.into_inner();
    let block_transform = block.into_inner();

    let mut dir = Vec2::ZERO;

    if input.pressed(KeyCode::KeyA) {
        dir.x -= 1.;
    }

    if input.pressed(KeyCode::KeyD) {
        dir.x += 1.;
    }

    if input.pressed(KeyCode::KeyW) {
        dir.y += 1.;
    }

    if input.pressed(KeyCode::KeyS) {
        dir.y -= 1.;
    }

    let deltat = time.delta_secs();
    let accel = ACCEL_RATE * deltat;

    **velocity = if dir.length() > 0. {
        (**velocity + (dir.normalize_or_zero() * accel)).clamp_length_max(PLAYER_SPEED)
    } else if velocity.length() > accel {
        **velocity + (velocity.normalize_or_zero() * -accel)
    } else {
        Vec2::ZERO
    };
    let change = **velocity * deltat;

    let max = Vec3::new(
        WIN_W / 2. - PLAYER_SIZE / 2.,
        WIN_H / 2. - PLAYER_SIZE / 2.,
        0.,
    );
    let min = max.clone() * -1.;

    // Better ideas?
    let new_pos = transform.translation + change.extend(0.).clamp(min, max);
    if !my_collision(new_pos, block_transform.translation) {
        transform.translation = new_pos;
    }
}
