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

#[derive(Component)]
pub struct Vision(u32);

#[derive(Component, Default)]
pub struct Metabolism {
    sugar: u32,
    spice: u32,
}

pub struct MapLocation {
    sugar: u32,
}

#[derive(Resource)]
pub struct MapData {
    locations: Vec<Vec<MapLocation>>,
}

fn update_agents(
    dims: Res<MapDimensions>,
    mut map: ResMut<MapData>,
    mut agents: Query<(&Name, &mut Location, &Vision, &Metabolism), With<Agent>>,
) {
    let mut locmap = std::collections::HashMap::new();
    // for each agent
    for (name, mut loc, vis, met) in agents.iter_mut() {
        let (x, y) = (loc.x, loc.y);
        locmap.insert((x, y), &name.0);

        let y_min = if y > vis.0 { y - vis.0 } else { 0 };
        let y_max = if y > dims.height - vis.0 {
            dims.height
        } else {
            y + vis.0
        };
        let x_min = if x > vis.0 { x - vis.0 } else { 0 };
        let x_max = if x > dims.width - vis.0 {
            dims.width
        } else {
            x + vis.0
        };

        let mut max_sugar = 0;
        let mut next_x = 0;
        let mut next_y = 0;

        // check horizontally
        for nx in x_min..x_max {
            if !locmap.contains_key(&(nx, y)) {
                let pot_sugar = map.locations[nx as usize][y as usize].sugar;
                if pot_sugar > max_sugar {
                    max_sugar = pot_sugar;
                    (next_x, next_y) = (nx, y);
                }
            }
        }
        // check horizontally
        for ny in y_min..y_max {
            if !locmap.contains_key(&(x, ny)) {
                let pot_sugar = map.locations[x as usize][ny as usize].sugar;
                if pot_sugar > max_sugar {
                    max_sugar = pot_sugar;
                    (next_x, next_y) = (x, ny);
                }
            }
        }

        loc.x = next_x;
        loc.y = next_y;
        map.locations[loc.x as usize][loc.y as usize].sugar -= met.sugar;
    }
    //   find the best location
    //   move there
    //   eat
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

fn print_world(
    dims: Res<MapDimensions>,
    map: Res<MapData>,
    agents: Query<(&Name, &Location), With<Agent>>,
) {
    let mut locs = std::collections::HashSet::new();
    for (_name, loc) in &agents {
        let (x, y) = (loc.x, loc.y);
        locs.insert((x, y));
    }
    for row in (0..dims.height).rev() {
        for col in 0..(dims.width - 1) {
            if locs.contains(&(col, row)) {
                print!("Ag-");
            } else {
                print!("{:0>2}-", map.locations[col as usize][row as usize].sugar);
            }
        }
        if locs.contains(&((dims.width - 1), row)) {
            print!("Ag");
        } else {
            print!(
                "{:0>2}",
                map.locations[(dims.width - 1) as usize][row as usize].sugar
            );
        }
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
    //print_sugar(&map, &dims);
}

fn add_agents(mut commands: Commands) {
    commands.spawn((
        Agent,
        Name("Smith".to_string()),
        Location::default(),
        Vision(4),
        Metabolism { sugar: 5, spice: 3 },
    ));
    commands.spawn((
        Agent,
        Name("Jones".to_string()),
        Location { x: 9, y: 9 },
        Vision(2),
        Metabolism { sugar: 7, spice: 4 },
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
        .add_systems(PreUpdate, print_world)
        .add_systems(Update, agent_details)
        .add_systems(Update, hello_world)
        .add_systems(Update, update_agents)
        .add_systems(PostUpdate, print_world)
        .run();
}
