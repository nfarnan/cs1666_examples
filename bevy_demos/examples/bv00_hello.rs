use bevy::{prelude::*, window::PresentMode};

#[derive(Component, Deref, DerefMut)]
struct PopupTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Hello World!".into(),
                resolution: (640., 480.).into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, show_popup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(Sprite::from_image(asset_server.load("hello_world_win.png")));
    commands.spawn((
        Sprite::from_image(asset_server.load("tuxdoge.png")),
        Transform {
            translation: Vec3::new(0., 0., -1.),
            ..default()
        },
        PopupTimer(Timer::from_seconds(10., TimerMode::Once)),
    ));

    info!("Hello world!");
}

fn show_popup(time: Res<Time>, mut popup: Query<(&mut PopupTimer, &mut Transform)>) {
    for (mut timer, mut transform) in popup.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            transform.translation.z = 2.;
            info!("Should be Linux!");
        }
    }
}
