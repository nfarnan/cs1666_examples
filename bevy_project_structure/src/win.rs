use bevy::prelude::*;

use crate::{GameState, loading::LoadingAssets};

#[derive(Event, Default)]
pub struct Win;

#[derive(Component)]
pub struct WinScreen;

#[derive(Resource)]
pub struct WinScreenImage(Handle<Image>);

pub struct WinPlugin;
impl Plugin for WinPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_win)
            .add_systems(OnEnter(GameState::Win), setup_win)
            .add_systems(Update, win_event_listener)
            .add_event::<Win>();
    }
}

fn load_win(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading_assets: ResMut<LoadingAssets>,
) {
    let win_texture_handle = asset_server.load("win.png");

    loading_assets.0.push(win_texture_handle.clone().untyped());
    commands.insert_resource(WinScreenImage(win_texture_handle));
}

fn setup_win(
    mut commands: Commands,
    winscreen_image: Res<WinScreenImage>,
    mut camera: Single<&mut Transform, With<Camera>>,
) {
    commands.spawn((
        Sprite::from_image(winscreen_image.0.clone()),
        Transform::from_xyz(0., 0., 0.),
        WinScreen,
    ));

    camera.translation.x = 0.;
}

fn win_event_listener(
    mut win_event: EventReader<Win>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !win_event.is_empty() {
        next_state.set(GameState::Win);
        win_event.clear();
    }
}
