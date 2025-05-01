use bevy::{
    color::palettes::basic::BLUE, color::palettes::basic::GRAY, color::palettes::basic::GREEN,
    color::palettes::basic::PURPLE, color::palettes::basic::RED, prelude::*,
};
use rand::Rng;
use std::f32;

const TILE_COLOR: Srgba = GREEN;
const TILE_SIZE: Vec2 = Vec2::new(30., 30.);
const TILE_GAP: f32 = 2.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component, Default)]
struct Tile {
    sugar: u32,
}

#[derive(Component, Default)]
struct Coord {
    x: u32,
    y: u32,
}

impl Coord {
    fn tile_position(&self) -> Vec2 {
        Vec2::new(
            self.x as f32 * (TILE_GAP + TILE_SIZE.x),
            self.y as f32 * (TILE_GAP + TILE_SIZE.y),
        )
    }
}

#[derive(Component)]
struct Sugar;

#[derive(Component)]
struct Agent;

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
    let _side_margin = 10.0;
    let _top_margin = 10.0;
    let start_corner = Vec2::new(
        // this should become a Resource
        cols as f32 * (TILE_SIZE.x + TILE_GAP) / 2.,
        rows as f32 * (TILE_SIZE.y + TILE_GAP) / 2.,
    );

    let agents = rows + cols;

    for row in 0..rows {
        for column in 0..cols {
            let coord = Coord { x: column, y: row };
            let tile_position = coord.tile_position() - start_corner;
            let color = if row == 2 && column == 3 {
                PURPLE
            } else {
                TILE_COLOR
            };
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(materials.add(Color::from(color))),
                Transform::default()
                    .with_scale(TILE_SIZE.extend(0.))
                    .with_translation(tile_position.extend(0.)),
                Tile::default(),
                coord,
            ));
            /*
                .with_child((
                    Mesh2d(meshes.add(Rectangle::default())),
                    MeshMaterial2d(materials.add(Color::from(RED))),
                    //MeshMaterial2d(materials.add(Color::from(GRAY))),
                    Transform::default()
                        .with_scale(Vec3::new(
                            //0.1 * TILE_SIZE.x,
                            0.1,
                            //TILE_SIZE.y - 2. * TILE_GAP,
                            (TILE_SIZE.y - 2. * TILE_GAP) / TILE_SIZE.y,
                            1.,
                        ))
                        .with_translation(Vec3::new(-0.4, 0., 0.)),
                    //.with_translation(Vec3::new(-0.5 * TILE_SIZE.x, -0.5 * TILE_SIZE.y, 1.)),
                    Sugar,
                ));
            */
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(materials.add(Color::from(GRAY))),
                Transform::default()
                    .with_scale(Vec3::new(
                        0.2 * TILE_SIZE.x,
                        TILE_SIZE.y - 2. * TILE_GAP,
                        //(TILE_SIZE.y - 2. * TILE_GAP) / TILE_SIZE.y,
                        1.,
                    ))
                    .with_translation(Vec3::new(
                        tile_position.x + (0.4 * TILE_SIZE.x),
                        tile_position.y, // + (-0.5 * TILE_SIZE.y),
                        10.,
                    )),
                Sugar,
            ));
        }
    }

    for _agent in 0..agents {
        let x = rand::thread_rng().gen_range(0..cols);
        let y = rand::thread_rng().gen_range(0..rows);
        let coord = Coord { x, y };
        let tile_position = coord.tile_position() - start_corner;
        commands.spawn((
            Mesh2d(meshes.add(Circle::default())),
            MeshMaterial2d(materials.add(Color::from(BLUE))),
            Transform::default()
                .with_scale(Vec3::splat(0.7 * f32::min(TILE_SIZE.x, TILE_SIZE.y)))
                .with_translation(Vec3::new(tile_position.x, tile_position.y, 100.)),
            Agent,
            coord,
        ));
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
