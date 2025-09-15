use bevy::{prelude::*, window::PresentMode};
use std::convert::From;

const TITLE: &str = "bv08 Simple BG Scroll";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

const PLAYER_SPEED: f32 = 500.;
const ACCEL_RATE: f32 = 5000.;

const TILE_SIZE: u32 = 100;

const SCROLL_SPEED: f32 = 120.;

enum PlayerType {
    Bird,
    Plane,
    UFO,
    Helicopter,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Background;

#[derive(Component, Deref, DerefMut)]
struct Velocity {
    velocity: Vec2,
}

impl Velocity {
    fn new() -> Self {
        Self {
            velocity: Vec2::ZERO,
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
        .add_systems(Update, scroll_bg)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    for start_x in [0., WIN_W] {
        commands.spawn((
            Sprite::from_image(asset_server.load("small_bg.png")),
            Transform::from_xyz(start_x, 0., -1.),
            Velocity::from(Vec2::new(SCROLL_SPEED, 0.)),
            Background,
        ));
    }

    let bird_sheet_handle = asset_server.load("birds.png");
    let bird_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 2, 2, None, None);
    let bird_layout_handle = texture_atlases.add(bird_layout);
    commands.spawn((
        Sprite::from_atlas_image(
            bird_sheet_handle,
            TextureAtlas {
                layout: bird_layout_handle,
                index: PlayerType::Plane as usize,
            },
        ),
        Transform {
            translation: Vec3::new(0., 0., 900.),
            ..default()
        },
        Velocity::new(),
        Player,
    ));
}

fn scroll_bg(
    time: Res<Time>,
    mut bg: Query<(&mut Transform, &Velocity), (With<Background>, Without<Player>)>,
) {
    let deltat = time.delta_secs();
    for (mut bt, bv) in bg.iter_mut() {
        bt.translation -= bv.velocity.extend(0.) * deltat;
        if bt.translation.x < -WIN_W {
            bt.translation += Vec3::new(WIN_W * 2., 0., 0.);
        }
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

    let max = Vec2::new(
        WIN_W / 2. - (TILE_SIZE as f32) / 2.,
        WIN_H / 2. - (TILE_SIZE as f32) / 2.,
    );
    let min = max.clone() * -1.;

    transform.translation =
        (transform.translation + change.extend(0.)).clamp(min.extend(900.), max.extend(900.));
}
