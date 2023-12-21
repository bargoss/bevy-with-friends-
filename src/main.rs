mod utils;
use utils::*;

mod defender_game;
mod space_game;

use bevy::DefaultPlugins;
use bevy::prelude::{App, Assets, Camera, Camera3dBundle, Color, Commands, default, EventReader, GamepadAxis, GlobalTransform, info, Mesh, MouseButton, PbrBundle, Res, ResMut, Resource, shape, StandardMaterial, Startup, Transform, Update, Vec2, Vec3, Visibility, Window};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_vector_shapes::prelude::{LinePainter, ShapePlugin};
use bevy_vector_shapes::prelude::ShapePainter;
use bevy_vector_shapes::shapes::DiscPainter;
use bevy::ecs::system::Query;
use bevy::input::ButtonState::Pressed;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::IntoSystemConfigs;
use bevy_framepace::FramepacePlugin;
use bevy_rapier2d::prelude::{NoUserData, RapierDebugRenderPlugin, RapierPhysicsPlugin};

const GRID_LEN: f32 = 5.0;

fn main() {
    //run_defender_game();
    run_space_game();
}

fn run_space_game() {

}

fn run_defender_game() {
    let limiter = bevy_framepace::Limiter::from_framerate(6000.0);
    let framepace_settings = bevy_framepace::FramepaceSettings {
        limiter
    };

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(ShapePlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(FramepacePlugin)
        .insert_resource(framepace_settings)
        .add_plugins(defender_game::plugin::DefenderGamePlugin)
        .run();
}
