use bevy::prelude::*;
use std::convert::From;

use crate::{
    ACCEL_RATE, ANIM_TIME, GameState, LEVEL_LEN, PLAYER_SPEED, TILE_SIZE, WIN_H, WIN_W,
    level::Background,
    loading::{LoadingAssets, despawn_with},
    win::Win,
};

#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationFrameCount(usize);

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

#[derive(Resource)]
pub struct PlayerSheet(Handle<Image>, Handle<TextureAtlasLayout>);

impl Velocity {
    fn new() -> Self {
        Self(Vec2::ZERO)
    }
}

impl From<Vec2> for Velocity {
    fn from(velocity: Vec2) -> Self {
        Self(velocity)
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_player_sheet)
            .add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, move_player.run_if(in_state(GameState::Playing)))
            .add_systems(
                Update,
                (animate_player, move_camera)
                    .after(move_player)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), despawn_with::<Player>);
    }
}

fn load_player_sheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut loading_assets: ResMut<LoadingAssets>,
) {
    let player_sheet_handle = asset_server.load("walking.png");
    loading_assets.push(player_sheet_handle.clone().untyped());

    let player_layout =
        TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE as u32), 4, 1, None, None);
    let player_layout_handle = texture_atlases.add(player_layout);

    commands.insert_resource(PlayerSheet(player_sheet_handle, player_layout_handle));
}

fn spawn_player(
    mut commands: Commands,
    texture_atlases: Res<Assets<TextureAtlasLayout>>,
    player_sheet: Res<PlayerSheet>,
) {
    let player_layout = texture_atlases.get(&player_sheet.1);
    let player_layout_len = player_layout.unwrap().len();

    commands.spawn((
        Sprite::from_atlas_image(
            player_sheet.0.clone(),
            TextureAtlas {
                layout: player_sheet.1.clone(),
                index: 0,
            },
        ),
        Transform::from_xyz(0., -(WIN_H / 2.) + (TILE_SIZE * 1.5), 900.),
        AnimationTimer(Timer::from_seconds(ANIM_TIME, TimerMode::Repeating)),
        AnimationFrameCount(player_layout_len),
        Velocity::new(),
        Player,
    ));
}

fn move_player(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    player: Single<(&mut Transform, &mut Velocity), (With<Player>, Without<Background>)>,
    mut win_event: EventWriter<Win>,
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

    if transform.translation.x > LEVEL_LEN - (WIN_W / 2. + TILE_SIZE) {
        // Close enough to end of level, move to WinScreen
        win_event.send(Win);
    }
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
    if v.cmpne(Vec2::ZERO).any() {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = (atlas.index + 1) % **frame_count;
            }
        }
    }
}

fn move_camera(
    player: Single<&Transform, With<Player>>,
    mut camera: Single<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    camera.translation.x = player.translation.x.clamp(0., LEVEL_LEN - WIN_W);
}
