use bevy::{prelude::*, window::PresentMode};

const TITLE: &str = "bv13 Circ Collision";
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
struct Dot;

#[derive(Component)]
struct Circle {
    radius: f32,
}

impl Circle {
    fn new(radius: f32) -> Self {
        Self { radius }
    }
}

fn my_circ_collision(a_translation: Vec3, a_rad: f32, b_translation: Vec3, b_rad: f32) -> bool {
    let radsum_sq = (a_rad + b_rad).powi(2);
    let dist_sq = a_translation
        .truncate()
        .distance_squared(b_translation.truncate());

    dist_sq < radsum_sq
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("red_circle.png"),
            transform: Transform {
                translation: Vec3::new(-WIN_W / 4., 0., 0.),
                ..default()
            },
            ..default()
        })
        .insert(Velocity::new())
        .insert(Circle::new(PLAYER_SIZE / 2.))
        .insert(Player);

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("blue_circle.png"),
            transform: Transform {
                translation: Vec3::new(WIN_W / 4., 0., 0.),
                ..default()
            },
            ..default()
        })
        .insert(Circle::new(PLAYER_SIZE / 2.))
        .insert(Dot);
}

fn move_player(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut player: Query<(&mut Transform, &Circle, &mut Velocity), (With<Player>, Without<Dot>)>,
    dot: Query<(&Transform, &Circle), (With<Dot>, Without<Player>)>,
) {
    let (mut pt, pc, mut pv) = player.single_mut();
    let (dt, dc) = dot.single();

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
    if !my_circ_collision(new_pos, pc.radius, dt.translation, dc.radius)
        && new_pos.x >= -(WIN_W / 2.) + PLAYER_SIZE / 2.
        && new_pos.x <= WIN_W / 2. - PLAYER_SIZE / 2.
    {
        pt.translation = new_pos;
    }

    let new_pos = pt.translation + Vec3::new(0., change.y, 0.);
    if !my_circ_collision(new_pos, pc.radius, dt.translation, dc.radius)
        && new_pos.y >= -(WIN_H / 2.) + PLAYER_SIZE / 2.
        && new_pos.y <= WIN_H / 2. - PLAYER_SIZE / 2.
    {
        pt.translation = new_pos;
    }
}
