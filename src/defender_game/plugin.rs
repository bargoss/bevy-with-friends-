use bevy::app::App;
use bevy::prelude::{Plugin, Startup, Update};
use crate::defender_game::events::*;
use crate::defender_game::systems::*;

pub struct DefenderGamePlugin;

impl Plugin for DefenderGamePlugin{
    fn build(&self, app: &mut App) {
        app
            .add_event::<ProjectileCollisionEvent>()
            .add_systems(Startup, init)
            .add_systems(Update, handle_projectile_movement)
            .add_systems(Update, projectile_collision_events)
            .add_systems(Update, handle_projectile_enemy_collisions)
            .add_systems(Update, draw_player_tower);
    }
}