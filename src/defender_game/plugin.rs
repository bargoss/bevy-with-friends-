use bevy::app::App;
use bevy::prelude::{Plugin, Startup, Update};
use bevy_framepace::FramepacePlugin;
use crate::defender_game::events::*;
use crate::defender_game::resources::*;
use crate::defender_game::systems::*;
use bevy::prelude::IntoSystemConfigs;

pub struct DefenderGamePlugin;

impl Plugin for DefenderGamePlugin{
    fn build(&self, app: &mut App) {
        app
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