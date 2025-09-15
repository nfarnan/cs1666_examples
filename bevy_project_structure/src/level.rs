use bevy::prelude::*;

use crate::{
    GameState, LEVEL_LEN, TILE_SIZE, WIN_H, WIN_W,
    loading::{LoadingAssets, despawn_with},
};

#[derive(Component)]
pub struct Brick;

#[derive(Component)]
pub struct Background;

#[derive(Resource)]
pub struct BackgroundImage(Handle<Image>);
#[derive(Resource)]
pub struct BrickSheet(Handle<Image>, Handle<TextureAtlasLayout>);

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_level)
            .add_systems(OnEnter(GameState::Playing), setup_level)
            .add_systems(
                OnExit(GameState::Playing),
                (despawn_with::<Brick>, despawn_with::<Background>),
            );
    }
}

fn load_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut loading_assets: ResMut<LoadingAssets>,
) {
    let bg_texture_handle = asset_server.load("small_bg.png");

    loading_assets.0.push(bg_texture_handle.clone().untyped());
    commands.insert_resource(BackgroundImage(bg_texture_handle));

    let brick_sheet_handle: Handle<Image> = asset_server.load("bricks.png");
    loading_assets.0.push(brick_sheet_handle.clone().untyped());

    let brick_layout =
        TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE as u32), 4, 1, None, None);
    let brick_layout_handle = texture_atlases.add(brick_layout);

    commands.insert_resource(BrickSheet(brick_sheet_handle, brick_layout_handle));
}

fn setup_level(
    mut commands: Commands,
    texture_atlases: Res<Assets<TextureAtlasLayout>>,
    background_image: Res<BackgroundImage>,
    brick_sheet: Res<BrickSheet>,
) {
    let mut x_offset = 0.;
    while x_offset < LEVEL_LEN {
        commands.spawn((
            Sprite::from_image(background_image.0.clone()),
            Transform::from_xyz(x_offset, 0., 0.),
            Background,
        ));

        x_offset += WIN_W;
    }

    let brick_layout = texture_atlases.get(&brick_sheet.1);
    let brick_layout_len = brick_layout.unwrap().len();
    let mut i = 0;
    let mut t = Vec3::new(
        -WIN_W / 2. + TILE_SIZE / 2.,
        -WIN_H / 2. + TILE_SIZE / 2.,
        1.,
    );
    while (i as f32) * TILE_SIZE < LEVEL_LEN {
        commands
            .spawn((
                Sprite::from_atlas_image(
                    brick_sheet.0.clone(),
                    TextureAtlas {
                        layout: brick_sheet.1.clone(),
                        index: i % brick_layout_len,
                    },
                ),
                Transform {
                    translation: t,
                    ..default()
                },
                Brick,
            ))
            .insert(Brick);

        i += 1;
        t += Vec3::new(TILE_SIZE, 0., 0.);
    }
}
