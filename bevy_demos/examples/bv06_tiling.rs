use bevy::{prelude::*, window::PresentMode};
use rand::Rng;

const TITLE: &str = "bv06 Tiling";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

const TILE_SIZE: f32 = 100.;
const NUM_BIRDS: usize = 8;

#[derive(Component)]
struct Bird;

#[derive(Component)]
struct Brick;

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
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let bird_handle = asset_server.load("birds.png");
    let bird_atlas = TextureAtlas::from_grid(bird_handle, Vec2::splat(TILE_SIZE), 2, 2, None, None);
    let bird_atlas_len = bird_atlas.textures.len();
    let bird_atlas_handle = texture_atlases.add(bird_atlas);

    let brick_handle = asset_server.load("bricks.png");
    let brick_atlas =
        TextureAtlas::from_grid(brick_handle, Vec2::splat(TILE_SIZE), 4, 1, None, None);
    let brick_atlas_len = brick_atlas.textures.len();
    let brick_atlas_handle = texture_atlases.add(brick_atlas);

    commands.spawn(Camera2dBundle::default());

    let mut rng = rand::thread_rng();
    let x_bound = WIN_W / 2. - TILE_SIZE / 2.;
    let y_bound = WIN_H / 2. - TILE_SIZE / 2.;

    for i in 0..NUM_BIRDS {
        let t = Vec3::new(
            rng.gen_range(-x_bound..x_bound),
            rng.gen_range(-y_bound..y_bound),
            900.,
        );
        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: bird_atlas_handle.clone(),
                transform: Transform {
                    translation: t,
                    ..default()
                },
                sprite: TextureAtlasSprite {
                    index: i % bird_atlas_len,
                    ..default()
                },
                ..default()
            })
            .insert(Bird);
    }

    let mut i = 0;
    let mut t = Vec3::new(-x_bound, -y_bound, 0.);
    while (i as f32) * TILE_SIZE < WIN_W {
        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: brick_atlas_handle.clone(),
                transform: Transform {
                    translation: t,
                    ..default()
                },
                sprite: TextureAtlasSprite {
                    index: i % brick_atlas_len,
                    ..default()
                },
                ..default()
            })
            .insert(Brick);

        i += 1;
        t += Vec3::new(TILE_SIZE, 0., 0.);
    }
}
