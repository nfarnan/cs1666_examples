use bevy::{prelude::*, window::PresentMode};
use std::convert::From;

const TITLE: &str = "bv11 Animation";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

const PLAYER_SPEED: f32 = 500.;
const ACCEL_RATE: f32 = 5000.;

const TILE_SIZE: u32 = 100;

const LEVEL_LEN: f32 = 5000.;

const ANIM_TIME: f32 = 0.2;

#[derive(Component)]
struct Player;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct AnimationFrameCount(usize);

#[derive(Component)]
struct Brick;

#[derive(Component)]
struct Background;

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

impl From<Vec2> for Velocity {
    fn from(velocity: Vec2) -> Self {
        Self { velocity }
    }
}

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
        .add_systems(Update, animate_player.after(move_player))
        .add_systems(Update, move_camera.after(move_player))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    let bg_texture_handle = asset_server.load("small_bg.png");

    let mut x_offset = 0.;
    while x_offset < LEVEL_LEN {
        commands.spawn((
            Sprite::from_image(bg_texture_handle.clone()),
            Transform::from_xyz(x_offset, 0., 0.),
            Background,
        ));

        x_offset += WIN_W;
    }

    let player_sheet_handle = asset_server.load("walking.png");
    let player_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 4, 1, None, None);
    let player_layout_len = player_layout.textures.len();
    let player_layout_handle = texture_atlases.add(player_layout);
    commands.spawn((
        Sprite::from_atlas_image(
            player_sheet_handle,
            TextureAtlas {
                layout: player_layout_handle,
                index: 0,
            },
        ),
        Transform::from_xyz(0., -(WIN_H / 2.) + ((TILE_SIZE as f32) * 1.5), 900.),
        AnimationTimer(Timer::from_seconds(ANIM_TIME, TimerMode::Repeating)),
        AnimationFrameCount(player_layout_len),
        Velocity::new(),
        Player,
    ));

    let brick_sheet_handle = asset_server.load("bricks.png");
    let brick_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 4, 1, None, None);
    let brick_layout_len = brick_layout.len();
    let brick_layout_handle = texture_atlases.add(brick_layout);

    let mut i = 0;
    let mut t = Vec3::new(
        -WIN_W / 2. + (TILE_SIZE as f32) / 2.,
        -WIN_H / 2. + (TILE_SIZE as f32) / 2.,
        1.,
    );
    while i * TILE_SIZE < (LEVEL_LEN as u32) {
        commands.spawn((
            Sprite::from_atlas_image(
                brick_sheet_handle.clone(),
                TextureAtlas {
                    layout: brick_layout_handle.clone(),
                    index: (i as usize) % brick_layout_len,
                },
            ),
            Transform {
                translation: t,
                ..default()
            },
            Brick,
        ));

        i += 1;
        t += Vec3::new(TILE_SIZE as f32, 0., 0.);
    }
}

fn move_player(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    player: Single<(&mut Transform, &mut Velocity), (With<Player>, Without<Background>)>,
) {
    let (mut transform, mut velocity) = player.into_inner();

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

    let min = Vec3::new(
        -WIN_W / 2. + (TILE_SIZE as f32) / 2.,
        -WIN_H / 2. + (TILE_SIZE as f32) * 1.5,
        900.,
    );
    let max = Vec3::new(
        LEVEL_LEN - (WIN_W / 2. + (TILE_SIZE as f32) / 2.),
        WIN_H / 2. - (TILE_SIZE as f32) / 2.,
        900.,
    );

    transform.translation = (transform.translation + change.extend(0.)).clamp(min, max);
}

fn animate_player(
    time: Res<Time>,
    player: Single<
        (
            &Velocity,
            &mut Sprite,
            &mut AnimationTimer,
            &AnimationFrameCount,
        ),
        With<Player>,
    >,
) {
    let (v, mut sprite, mut timer, frame_count) = player.into_inner();
    if v.velocity.cmpne(Vec2::ZERO).any() {
        timer.tick(time.delta());

        // Why commented? run and find out
        // if timer.just_finished() {
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = (atlas.index + 1) % **frame_count;
        }
        // }
    }
}

fn move_camera(
    player: Single<&Transform, With<Player>>,
    mut camera: Single<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    camera.translation.x = player.translation.x.clamp(0., LEVEL_LEN - WIN_W);
}
