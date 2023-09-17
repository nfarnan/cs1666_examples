use bevy::{prelude::*, window::PresentMode};
use rand::Rng;
use std::collections::HashMap;

const TITLE: &str = "bv07 Tiling";
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

struct SheetData {
    len: usize,
    handle: Handle<TextureAtlas>,
}

#[derive(PartialEq, Eq, Hash)]
enum SheetTypes {
    Bird,
    Brick,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let sheets_data: HashMap<_, _> = [SheetTypes::Bird, SheetTypes::Brick]
        .into_iter()
        .map(|s| {
            let (fname, cols, rows) = match s {
                SheetTypes::Bird => ("birds.png", 2, 2),
                SheetTypes::Brick => ("bricks.png", 4, 1),
            };
            let handle = asset_server.load(fname);
            let atlas =
                TextureAtlas::from_grid(handle, Vec2::splat(TILE_SIZE), cols, rows, None, None);
            (
                s,
                SheetData {
                    len: atlas.textures.len(),
                    handle: texture_atlases.add(atlas),
                },
            )
        })
        .collect();

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
        spawn_tile(&mut commands, &sheets_data[&SheetTypes::Bird], t, i, Bird);
    }

    let mut i = 0;
    let mut t = Vec3::new(-x_bound, -y_bound, 0.);
    while (i as f32) * TILE_SIZE < WIN_W {
        spawn_tile(&mut commands, &sheets_data[&SheetTypes::Brick], t, i, Brick);

        i += 1;
        t += Vec3::new(TILE_SIZE, 0., 0.);
    }
}

fn spawn_tile<T>(
    commands: &mut Commands,
    data: &SheetData,
    translation: Vec3,
    index: usize,
    component: T,
) where
    T: Component,
{
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: data.handle.clone(),
            transform: Transform {
                translation,
                ..default()
            },
            sprite: TextureAtlasSprite {
                index: index % data.len,
                ..default()
            },
            ..default()
        })
        .insert(component);
}
