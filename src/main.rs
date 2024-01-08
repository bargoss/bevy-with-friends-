use bevy::{DefaultPlugins, MinimalPlugins};
use bevy::app::Plugins;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_vector_shapes::prelude::*;

use crate::lightyear_demo::client::DemoClientPlugin;
use crate::lightyear_demo::server::DemoServerPlugin;
use crate::lightyear_demo::shared::FixedUpdateMainSet;
use crate::lightyear_demo::systems::*;

mod utils;
mod defender_game;
mod lightyear_demo;

const GRID_LEN: f32 = 5.0;

fn init_test_stuff(){
    let mut app = App::new();
    app.add_systems(Update, draw_circle_view);
    app.run();
}
fn run_client(headless : bool) {
    let mut app_builder = App::new();
    app_builder.add_plugins(DemoClientPlugin { headless });

    if headless {
        app_builder
            .add_plugins(MinimalPlugins);
    } else {
        app_builder
            .add_plugins(DefaultPlugins)
            //.add_systems(FixedUpdate, draw_circle_view.after(FixedUpdateMainSet::Push))
            .add_systems(Update, draw_circle_view)
            .add_plugins(WorldInspectorPlugin::new())
            .add_plugins(ShapePlugin::default())
        ;
    }

    app_builder.run();

    //App::new()
    //    .add_plugins(base_plugins)
    //    .add_plugins(WorldInspectorPlugin::new())
    //    .add_plugins(ShapePlugin::default())
    //
    //    .add_plugins(DemoClientPlugin)
    //
    //    .add_systems(Update, draw_circle_view)
    //
    //    .run();
}
fn run_server(headless : bool) {
    //App::new()
    //    .add_plugins(DefaultPlugins)
    //    .add_plugins(DemoServerPlugin)
    //    .run();

    let mut app_builder = App::new();
    app_builder.add_plugins(DemoServerPlugin);

    if headless {
        app_builder
            .add_plugins(MinimalPlugins);
    } else {
        app_builder
            .add_plugins(DefaultPlugins)
            //.add_systems(Update, draw_circle_view)
            .add_plugins(WorldInspectorPlugin::new())
            //.add_plugins(ShapePlugin::default())
        ;
    }

    app_builder.run();
}

fn main() {
    // create a new thread
    let thread0 = std::thread::spawn(move || -> anyhow::Result<()> {
        run_server(true);
        //run_client(true);
        Ok(())
    });



    //run_server(false);
    run_client(false);
}
