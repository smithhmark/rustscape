use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Agent;

#[derive(Component)]
pub struct Name(String);

#[derive(Component, Default)]
pub struct Location {
    x: u32,
    y: u32,
}

pub struct MapLocation {
    sugar: u32,
}

#[derive(Resource)]
pub struct MapData {
    locations: Vec<Vec<MapLocation>>,
}

fn populate_map(map: &mut MapData, dims: &MapDimensions) {
    let mut rng = rand::thread_rng();

    let mut new_map = MapData {
        locations: Vec::new(),
    };
    for _col in 0..dims.width {
        let mut column = Vec::new();
        for _row in 0..dims.height {
            let sugar: u32 = rng.gen_range(0..20);
            column.push(MapLocation { sugar })
        }
        new_map.locations.push(column);
    }
    *map = new_map;
}

fn print_sugar(map: &MapData, dims: &MapDimensions) {
    for row in (0..dims.height).rev() {
        for col in 0..(dims.width - 1) {
            print!("{:0>2}-", map.locations[col as usize][row as usize].sugar);
        }
        print!(
            "{:0>2}",
            map.locations[(dims.width - 1) as usize][row as usize].sugar
        );
        println!();
    }
}

#[derive(Resource)]
pub struct MapDimensions {
    width: u32,
    height: u32,
}

fn hello_world() {
    println!("Hello, world! (from a system)");
}

fn agent_details(map: Res<MapDimensions>, query: Query<(&Name, &Location), With<Agent>>) {
    for (name, loc) in &query {
        if loc.x < map.width && loc.y < map.height {
            println!("Agent {} is inbounds at {},{}", name.0, loc.x, loc.y);
        } else {
            println!("Agent {} is out-of-bounds at {},{}", name.0, loc.x, loc.y);
        }
    }
}

fn build_map(dims: Res<MapDimensions>, mut map: ResMut<MapData>) {
    populate_map(&mut map, &dims);
    print_sugar(&map, &dims);
}

fn add_agents(mut commands: Commands) {
    commands.spawn((Agent, Name("Smith".to_string()), Location::default()));
    commands.spawn((
        Agent,
        Name("Jones".to_string()),
        Location { x: 100, y: 1000 },
    ));
}

fn main() {
    //println!("Hello, world!");

    App::new()
        .insert_resource(MapDimensions {
            width: 10,
            height: 10,
        })
        .insert_resource(MapData {
            locations: Vec::new(),
        })
        .add_systems(Startup, add_agents)
        .add_systems(Startup, build_map)
        .add_systems(Update, agent_details)
        .add_systems(Update, hello_world)
        .run();
}
