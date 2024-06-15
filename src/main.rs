use bevy::prelude::*;

#[derive(Component)]
pub struct Agent;

#[derive(Component)]
pub struct Name(String);

fn hello_world() {
    println!("Hello, world! (from a system)");
}

fn agent_names(query: Query<&Name, With<Agent>>) {
    for agent in &query {
        println!("Agent name: {}", agent.0);
    }
}

fn add_agents(mut commands: Commands) {
    commands.spawn((Agent, Name("Smith".to_string())));
}

fn main() {
    println!("Hello, world!");
    App::new()
        .add_systems(Startup, add_agents)
        .add_systems(Update, agent_names)
        .add_systems(Update, hello_world)
        .run();
}
