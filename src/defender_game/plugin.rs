use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::prelude::{Plugin, Startup, Update};
use bevy::prelude::IntoSystemConfigs;
use bevy_framepace::FramepacePlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier2d::prelude::RapierDebugRenderPlugin;
use bevy_vector_shapes::ShapePlugin;

use crate::defender_game;
use crate::defender_game::events::*;
use crate::defender_game::resources::*;
use crate::defender_game::systems::*;

pub struct DefenderGamePlugin;

impl Plugin for DefenderGamePlugin{
    fn build(&self, app: &mut App) {
        let limiter = bevy_framepace::Limiter::from_framerate(6000.0);
        let framepace_settings = bevy_framepace::FramepaceSettings{
            limiter
        };

        app
            .add_plugins(DefaultPlugins)
            .add_plugins(WorldInspectorPlugin::new())
            .add_plugins(ShapePlugin::default())
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_plugins(FramepacePlugin)
            .insert_resource(framepace_settings)
            .add_plugins(defender_game::plugin::DefenderGamePlugin)


            .add_event::<ProjectileCollisionEvent>()
            .insert_resource(UserInput::default())
            .insert_resource(DefenderGameConfig::default())

            .add_systems(Startup, init)
            .add_systems(Update, (
                // input:
                (
                    take_user_input_system,
                    update_player_tower_input_system
                ).chain(),

                (
                    // game logic, player:
                    player_tower_system,

                    // game logic, projectile:
                    projectile_movement_system,
                    projectile_damage_system,
                    projectile_collision_system,

                    // game logic, enemy:
                    enemy_death_system,
                    enemy_spawner_system,

                    // game logic
                    life_span_system,
                ),

                // display:
                (
                    draw_player_towers,
                    draw_projectiles,
                    draw_enemies
                )
            ).chain());
    }
}