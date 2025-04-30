use bevy::{
    color::palettes::basic::GREEN, color::palettes::basic::PURPLE, color::palettes::basic::RED,
    prelude::*,
};

const TILE_COLOR: Srgba = GREEN;
const TILE_SIZE: Vec2 = Vec2::new(30., 30.);
const TILE_GAP: f32 = 2.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct Tile;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    // determine the size of the window
    // determine the size of a tile
    let rows = 20;
    let cols = 30;
    let side_margin = 10.0;
    let top_margin = 10.0;
    let start_corner = Vec2::new(
        cols as f32 * (TILE_SIZE.x + TILE_GAP) / 2.,
        rows as f32 * (TILE_SIZE.y + TILE_GAP) / 2.,
    );

    for row in 0..rows {
        for column in 0..cols {
            let tile_position = Vec2::new(
                side_margin + column as f32 * (TILE_GAP + TILE_SIZE.x),
                top_margin + row as f32 * (TILE_GAP + TILE_SIZE.y),
            ) - start_corner;
            let color = if row == 2 && column == 3 {
                PURPLE
            } else {
                TILE_COLOR
            };
            let _parent = commands.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(materials.add(Color::from(color))),
                Transform::default()
                    .with_scale(TILE_SIZE.extend(0.))
                    .with_translation(tile_position.extend(0.)),
                Tile,
            ));
        }
    }
    /*
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::from(PURPLE))),
        Transform::default().with_scale(Vec3::splat(128.)),
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::from(GREEN))),
        Transform::default()
            .with_scale(Vec3::splat(32.))
            .with_translation(Vec3::new(0., 0., 32.)),
    ));
    //println!("{:?}", Transform::default());
    */
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(10., 10.))),
        //Mesh2d(meshes.add(Rectangle::new(64., 128.))),
        //Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::from(RED))),
        Transform::default().with_translation(Vec3::new(0., 0., 32.)),
        //Transform::default(),
        //Transform::default().with_translation(Vec3::splat(32.)),
        //Transform::from_xyz(128., 128., 128.),
    ));
}
