use bevy::{prelude::*, window::PresentMode};

mod level;
mod loading;
mod music;
mod player;
mod win;

const TITLE: &str = "Better Bevy Project Setup";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

const PLAYER_SPEED: f32 = 500.;
const ACCEL_RATE: f32 = 5000.;
const ANIM_TIME: f32 = 0.2;

const TILE_SIZE: f32 = 100.;

const LEVEL_LEN: f32 = 5000.;

const PROGRESS_LENGTH: f32 = 120.;
const PROGRESS_HEIGHT: f32 = 20.;
const PROGRESS_FRAME: f32 = 5.;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    Loading,
    Playing,
    Win,
}

fn main() {
    App::new()
        // Setup Bevy and game window
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from(TITLE),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::Srgba(Srgba::gray(0.25))))
        // Set initial state
        .init_state::<GameState>()
        // Add general systems
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(GameState::Loading), log_state_change)
        .add_systems(OnEnter(GameState::Playing), log_state_change)
        .add_systems(OnEnter(GameState::Win), log_state_change)
        // Add all subsystems
        .add_plugins((
            loading::LoadingPlugin,
            music::BackgroundMusicPlugin,
            player::PlayerPlugin,
            level::LevelPlugin,
            win::WinPlugin,
        ))
        // Run the game
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn log_state_change(state: Res<State<GameState>>) {
    info!("Just moved to {:?}!", state.get());
}
