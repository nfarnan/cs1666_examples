use bevy::{prelude::*, window::PresentMode};

#[derive(Component, Deref, DerefMut)]
struct PopupTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Hello World!".into(),
                resolution: (640., 480.).into(),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, show_popup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("hello_world_win.png"),
        ..default()
    });
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("tuxdoge.png"),
            transform: Transform::from_xyz(0., 0., -1.),
            ..default()
        })
        .insert(PopupTimer(Timer::from_seconds(10., TimerMode::Once)));

    info!("Hello world!");
}

fn show_popup(time: Res<Time>, mut popup: Query<(&mut PopupTimer, &mut Transform)>) {
    for (mut timer, mut transform) in popup.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            transform.translation.z = 2.;
            info!("Actually is Linux!");
        }
    }
}
