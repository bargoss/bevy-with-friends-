use bevy::prelude::*;

use crate::defender_game::components::*;

// projectile collision event
#[derive(Event)]
pub struct ProjectileCollisionEvent{
    pub projectile: Projectile,
    pub projectile_position: Vec2,
    pub collided_entity: Entity,
}

