mod utils;
use utils::*;

mod defender_game;
mod lightyear_demo;

use bevy::{DefaultPlugins, MinimalPlugins};
use bevy::prelude::*;

use bevy_vector_shapes::prelude::*;
use bevy::prelude::IntoSystemConfigs;
use crate::lightyear_demo::server::DemoServerPlugin;
use crate::lightyear_demo::client::DemoClientPlugin;

const GRID_LEN: f32 = 5.0;

fn run_client() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DemoClientPlugin)
        .run();
}
fn run_server() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(DemoServerPlugin)
        .run();
}

fn main() {
    // create a new thread
    let server_thread = std::thread::spawn(move || -> anyhow::Result<()> {
        run_server();
        Ok(())
    });

    run_client();
}
