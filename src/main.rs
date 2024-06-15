use bevy::prelude::*;

fn hello_world() {
    println!("Hello, world! (from a system)");
}

fn main() {
    println!("Hello, world!");
    App::new().add_systems(Update, hello_world).run();
}
