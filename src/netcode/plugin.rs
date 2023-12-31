use bevy::app::App;
use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::*;

pub struct DefenderGamePlugin;

impl Plugin for DefenderGamePlugin{
    fn build(&self, app: &mut App) {
        app
            .add_schedule(Schedule::new(PredictedTick))
            .add_systems(PredictedTick, my_system);
    }
}

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct PredictedTick;

//app
//.add_schedule(MySchedule, Schedule::new())
//.add_system(MySchedule, my_system);

fn run_my_schedule(world: &mut World) {
    for i in 0..8 {
        world.run_schedule(PredictedTick);
    }
}
fn my_system() {
    println!("Hello from my_system!");
}

/*
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
*/