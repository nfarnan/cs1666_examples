use bevy::{prelude::*, window::PresentMode};
use std::convert::From;

const TITLE: &str = "bv12 Rect Collision";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;
const PLAYER_SIZE: f32 = 32.;
// 5px/frame @60Hz == 300px/s
const PLAYER_SPEED: f32 = 300.;
// 1px/frame^2 @60Hz == 3600px/s^2
const ACCEL_RATE: f32 = 3600.;

#[derive(Component)]
struct Player;

#[derive(Component)]
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

// Don't bother using this, use bevy::sprite::collide_aabb::collide()
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
        .insert_resource(ClearColor(Color::DARK_GRAY))
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
        .add_systems(Update, move_player)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(240, 140, 100),
                custom_size: Some(Vec2::splat(PLAYER_SIZE)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(-WIN_W / 4., 0., 0.),
                ..default()
            },
            ..default()
        })
        .insert(Velocity::new())
        .insert(Player);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(100, 170, 200),
                custom_size: Some(Vec2::splat(PLAYER_SIZE)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(WIN_W / 4., 0., 0.),
                ..default()
            },
            ..default()
        })
        .insert(Block);
}

fn move_player(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut player: Query<(&mut Transform, &mut Velocity), (With<Player>, Without<Block>)>,
    block: Query<&Transform, (With<Block>, Without<Player>)>,
) {
    let (mut pt, mut pv) = player.single_mut();
    let bt = block.single();

    let mut deltav = Vec2::splat(0.);

    if input.pressed(KeyCode::A) {
        deltav.x -= 1.;
    }

    if input.pressed(KeyCode::D) {
        deltav.x += 1.;
    }

    if input.pressed(KeyCode::W) {
        deltav.y += 1.;
    }

    if input.pressed(KeyCode::S) {
        deltav.y -= 1.;
    }

    let deltat = time.delta_seconds();
    let acc = ACCEL_RATE * deltat;

    pv.velocity = if deltav.length() > 0. {
        (pv.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(PLAYER_SPEED)
    } else if pv.velocity.length() > acc {
        pv.velocity + (pv.velocity.normalize_or_zero() * -acc)
    } else {
        Vec2::splat(0.)
    };
    let change = pv.velocity * deltat;

    let new_pos = pt.translation + Vec3::new(change.x, 0., 0.);
    if !my_collision(new_pos, bt.translation)
        && new_pos.x >= -(WIN_W / 2.) + PLAYER_SIZE / 2.
        && new_pos.x <= WIN_W / 2. - PLAYER_SIZE / 2.
    {
        pt.translation = new_pos;
    }

    let new_pos = pt.translation + Vec3::new(0., change.y, 0.);
    if !my_collision(new_pos, bt.translation)
        && new_pos.y >= -(WIN_H / 2.) + PLAYER_SIZE / 2.
        && new_pos.y <= WIN_H / 2. - PLAYER_SIZE / 2.
    {
        pt.translation = new_pos;
    }
}
