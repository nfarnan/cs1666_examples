use bevy::{prelude::*, window::PresentMode};

const TITLE: &str = "bv04 Basic Motion";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;
const PLAYER_SZ: f32 = 32.;

#[derive(Component)]
struct Player;

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
                color: Color::SEA_GREEN,
                custom_size: Some(Vec2::splat(PLAYER_SZ)),
                ..default()
            },
            ..default()
        })
        .insert(Player);
}

fn move_player(input: Res<Input<KeyCode>>, mut player: Query<&mut Transform, With<Player>>) {
    let mut player_transform = player.single_mut();

    let mut x_vel = 0.;
    let mut y_vel = 0.;

    if input.pressed(KeyCode::A) {
        x_vel -= 1.;
    }

    if input.pressed(KeyCode::D) {
        x_vel += 1.;
    }

    if input.pressed(KeyCode::W) {
        y_vel += 1.;
    }

    if input.pressed(KeyCode::S) {
        y_vel -= 1.;
    }

    player_transform.translation.x += x_vel;
    player_transform.translation.y += y_vel;
}

/* TODO:
 * Can we slowly ramp up to speed limit instead of max accel?
 * What about different refresh rates?
 * How do we stay inside the window?
 * How do we avoid speeding up along the diagonal?
 */
