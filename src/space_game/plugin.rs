use bevy::app::{App, Plugin, Startup, Update};
use bevy::DefaultPlugins;
use bevy::prelude::FixedUpdate;
use bevy_framepace::FramepacePlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier2d::prelude::RapierDebugRenderPlugin;
use bevy_vector_shapes::prelude::Line;
use bevy_vector_shapes::render::ShapeType3dPlugin;
use bevy_vector_shapes::ShapePlugin;
use crate::defender_game;
use crate::defender_game::events::ProjectileCollisionEvent;
use crate::defender_game::plugin::DefenderGamePlugin;
use crate::defender_game::resources::{DefenderGameConfig, UserInput};
use crate::space_game::config::SpaceGameConfig;


pub struct SharedPlugin;
impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        if !app.world.contains_resource::<ShapeType3dPlugin<Line>>() {
            app.add_plugins(ShapePlugin::default());
        }

        app
            .register_type::<SpaceGameConfig>()
            .insert_resource(UserInput::default())
            .insert_resource(DefenderGameConfig::default())
            .add_systems(FixedUpdate, (

                ).
            )


            ;
    }
}