use bevy::prelude::*;

// bevy component
#[derive(Component, Default, Clone)]
pub struct Enemy{
    pub last_attack_time : f32,
}

#[derive(Component)]
pub struct Health{
    pub hit_points : f32,
    pub max_hit_points : f32,
}

#[derive(Component)]
pub struct LifeSpan{
    pub start_time : f32,
    pub duration : f32,
    pub started : bool,
}
impl Default for LifeSpan{
    fn default() -> Self{
        LifeSpan{
            start_time : 0.0,
            duration : 0.0,
            started : false,
        }
    }
}


impl Health{
    pub fn new(hit_points: f32) -> Self{
        Health{
            max_hit_points : hit_points,
            hit_points,
        }
    }
}

#[derive(Component, Clone)]
pub struct Projectile{
    pub damage: f32,
    pub velocity: Vec2,
    pub max_range: f32,
    pub distance_travelled: f32,
}

#[derive(Component)]
pub struct EnemySpawner {
    pub last_spawn_time: f32,
}

#[derive(Component)]
pub struct PlayerTower{
    pub aim_direction: Vec2,
    pub shoot_input: bool,
    pub last_shot_time: f32,
}



impl PlayerTower {
    pub fn aim_direction_3d(&self) -> Vec3{
        Vec3::new(self.aim_direction.x, self.aim_direction.y, 0.0)
    }
}

//#[derive(Bundle)]
//pub struct PlayerTowerBundle{
//    pub player_tower: PlayerTower,
//    pub health: Health,
//    pub transform: Transform,
//}