use bevy::prelude::*;

// bevy component
#[derive(Component, Default)]
pub struct Enemy{
    pub last_attack_time : f32,
}

#[derive(Component)]
pub struct Health{
    pub hit_points : f32,
    pub max_hit_points : f32,
}

impl Health{
    pub fn new(hit_points: f32) -> Self{
        Health{
            max_hit_points : hit_points,
            hit_points
        }
    }
}

#[derive(Component)]
pub struct Projectile{
    pub damage: f32,
    pub velocity: Vec2,
    pub max_range: f32,
    pub distance_travelled: f32,
}

#[derive(Component)]
pub struct PlayerTower{
    pub aim_direction: Vec2,
    pub last_shot_time: f32,
}

//#[derive(Bundle)]
//pub struct PlayerTowerBundle{
//    pub player_tower: PlayerTower,
//    pub health: Health,
//    pub transform: Transform,
//}