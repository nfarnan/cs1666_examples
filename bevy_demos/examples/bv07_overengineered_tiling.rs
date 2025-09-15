use bevy::{prelude::*, window::PresentMode};
use std::collections::HashMap;

const TITLE: &str = "bv07 Tiling";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

const TILE_SIZE: u32 = 100;
const NUM_BIRDS: usize = 8;

#[derive(Component)]
struct Bird;

#[derive(Component)]
struct Brick;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::Srgba(Srgba::gray(0.25))))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

struct LayoutData {
    texture: Handle<Image>,
    len: usize,
    layout: Handle<TextureAtlasLayout>,
}

#[derive(PartialEq, Eq, Hash)]
enum SheetTypes {
    Bird,
    Brick,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let sheets_data: HashMap<_, _> = [SheetTypes::Bird, SheetTypes::Brick]
        .into_iter()
        .map(|sheet_type| {
            let (fname, cols, rows) = match sheet_type {
                SheetTypes::Bird => ("birds.png", 2, 2),
                SheetTypes::Brick => ("bricks.png", 4, 1),
            };
            let sheet_handle = asset_server.load(fname);
            let atlas =
                TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), cols, rows, None, None);
            (
                sheet_type,
                LayoutData {
                    texture: sheet_handle,
                    len: atlas.textures.len(),
                    layout: texture_atlases.add(atlas),
                },
            )
        })
        .collect();

    commands.spawn(Camera2d);

    let x_bound = WIN_W / 2. - (TILE_SIZE as f32) / 2.;
    let y_bound = WIN_H / 2. - (TILE_SIZE as f32) / 2.;

    for i in 0..NUM_BIRDS {
        let t = Vec3::new(
            rand::random_range(-x_bound..x_bound),
            rand::random_range(-y_bound..y_bound),
            900.,
        );
        spawn_tile(&mut commands, &sheets_data[&SheetTypes::Bird], t, i, Bird);
    }

    let mut i = 0;
    let mut t = Vec3::new(-x_bound, -y_bound, 0.);
    while ((i * TILE_SIZE) as f32) < WIN_W {
        spawn_tile(
            &mut commands,
            &sheets_data[&SheetTypes::Brick],
            t,
            i as usize,
            Brick,
        );

        i += 1;
        t += Vec3::new(TILE_SIZE as f32, 0., 0.);
    }
}

fn spawn_tile<C>(
    commands: &mut Commands,
    data: &LayoutData,
    translation: Vec3,
    index: usize,
    component: C,
) where
    C: Component,
{
    commands.spawn((
        Sprite::from_atlas_image(
            data.texture.clone(),
            TextureAtlas {
                layout: data.layout.clone(),
                index: index % data.len,
            },
        ),
        Transform {
            translation,
            ..default()
        },
        component,
    ));
}
