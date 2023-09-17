use bevy::{prelude::*, window::PresentMode};
use std::convert::From;

const TITLE: &str = "bv11 Animation";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

const PLAYER_SPEED: f32 = 500.;
const ACCEL_RATE: f32 = 5000.;

const TILE_SIZE: f32 = 100.;

const LEVEL_LEN: f32 = 5000.;

const ANIM_TIME: f32 = 0.2;

#[derive(Component)]
struct Player;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct Brick;

#[derive(Component)]
struct Background;

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

impl From<Vec2> for Velocity {
    fn from(velocity: Vec2) -> Self {
        Self { velocity }
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
        .add_systems(Update, animate_player.after(move_player))
        .add_systems(Update, move_camera.after(move_player))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());

    let bg_texture_handle = asset_server.load("small_bg.png");

    let mut x_offset = 0.;
    while x_offset < LEVEL_LEN {
        commands
            .spawn(SpriteBundle {
                texture: bg_texture_handle.clone(),
                transform: Transform::from_xyz(x_offset, 0., 0.),
                ..default()
            })
            .insert(Background);

        x_offset += WIN_W;
    }

    let player_handle = asset_server.load("walking.png");
    let player_atlas =
        TextureAtlas::from_grid(player_handle, Vec2::splat(TILE_SIZE), 4, 1, None, None);
    let player_atlas_handle = texture_atlases.add(player_atlas);
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: player_atlas_handle,
            sprite: TextureAtlasSprite {
                index: 0,
                ..default()
            },
            transform: Transform::from_xyz(0., -(WIN_H / 2.) + (TILE_SIZE * 1.5), 900.),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(
            ANIM_TIME,
            TimerMode::Repeating,
        )))
        .insert(Velocity::new())
        .insert(Player);

    let brick_handle = asset_server.load("bricks.png");
    let brick_atlas =
        TextureAtlas::from_grid(brick_handle, Vec2::splat(TILE_SIZE), 4, 1, None, None);
    let brick_len = brick_atlas.len();
    let brick_atlas_handle = texture_atlases.add(brick_atlas);

    let mut i = 0;
    let mut t = Vec3::new(
        -WIN_W / 2. + TILE_SIZE / 2.,
        -WIN_H / 2. + TILE_SIZE / 2.,
        0.,
    );
    while (i as f32) * TILE_SIZE < LEVEL_LEN {
        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: brick_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: i % brick_len,
                    ..default()
                },
                transform: Transform {
                    translation: t,
                    ..default()
                },
                ..default()
            })
            .insert(Brick);

        i += 1;
        t += Vec3::new(TILE_SIZE, 0., 0.);
    }
}

fn move_player(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut player: Query<(&mut Transform, &mut Velocity), (With<Player>, Without<Background>)>,
) {
    let (mut pt, mut pv) = player.single_mut();

    let mut deltav = Vec2::splat(0.);

    if input.pressed(KeyCode::A) {
        deltav.x -= 1.;
    }

    if input.pressed(KeyCode::D) {
        deltav.x += 1.;
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
    if new_pos.x >= -(WIN_W / 2.) + TILE_SIZE / 2.
        && new_pos.x <= LEVEL_LEN - (WIN_W / 2. + TILE_SIZE / 2.)
    {
        pt.translation = new_pos;
    }

    let new_pos = pt.translation + Vec3::new(0., change.y, 0.);
    if new_pos.y >= -(WIN_H / 2.) + (TILE_SIZE * 1.5) && new_pos.y <= WIN_H / 2. - TILE_SIZE / 2. {
        pt.translation = new_pos;
    }
}

fn animate_player(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut player: Query<
        (
            &Velocity,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
            &mut AnimationTimer,
        ),
        With<Player>,
    >,
) {
    let (v, mut sprite, texture_atlas_handle, mut timer) = player.single_mut();
    if v.velocity.cmpne(Vec2::ZERO).any() {
        timer.tick(time.delta());

        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn move_camera(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let pt = player.single();
    let mut ct = camera.single_mut();

    ct.translation.x = pt.translation.x.clamp(0., LEVEL_LEN - WIN_W);
}
