use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, AddAssign, Mul};
use bevy::asset::AssetContainer;
use bevy::prelude::*;
use bevy::reflect::{ReflectMut, ReflectOwned, ReflectRef, TypeInfo};
use bevy_inspector_egui::InspectorOptions;
use derive_more::{Add, Mul};
use lightyear::_reexport::{ShouldBePredicted};
use lightyear::client::components::Confirmed;
use lightyear::client::prediction::{Rollback, RollbackState};
use lightyear::prelude::*;
use lightyear::prelude::client::{Interpolated, LerpFn, Predicted};
use serde::{Deserialize, Serialize};

use crate::lightyear_demo::shared::*;

#[derive(Default,Component, Message, Deserialize, Serialize, Clone, Debug, PartialEq, Deref, DerefMut)]
pub struct Pawn{
    pub last_attack_time : Tick,
}

impl Add for Pawn{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self{
            last_attack_time: rhs.last_attack_time,
        }
    }
}

impl Mul<f32> for Pawn{
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self{
            last_attack_time: self.last_attack_time,
        }
    }
}




#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}
#[derive(Default,Component, Message, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PawnInput{
    pub movement_direction : Vec3,
    pub attack : bool,
}

#[derive(Component, Message, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CircleView{
    pub radius : f32,
    pub color : Color,
}
impl Default for CircleView{
    fn default() -> Self{
        CircleView{
            radius : 1.0,
            color : Color::WHITE,
        }
    }
}


#[derive(Bundle)]
pub struct PawnBundle{
    pawn: Pawn,
    pawn_input: PawnInput,
    player_id: PlayerId,
    replicated_position : ReplicatedPosition,
    replicate: Replicate,
    transform_bundle: TransformBundle,
    circle_view: CircleView,
}
impl PawnBundle{
    pub fn new(
        position: Vec3,
        radius : f32,
        color : Color,
        owner_client_id: ClientId,
    ) -> Self{
        Self{
            pawn: Pawn::default(),
            pawn_input: PawnInput::default(),
            player_id: PlayerId::new(owner_client_id),
            replicated_position : ReplicatedPosition(position),
            replicate: Replicate{
                prediction_target: NetworkTarget::Only(vec![owner_client_id]),
                interpolation_target: NetworkTarget::AllExcept(vec![owner_client_id]),
                replication_group : ReplicationGroup::Group(owner_client_id + 2^16),
                ..Default::default()
            },
            transform_bundle: TransformBundle{
                local: Transform::from_translation(position),
                ..Default::default()
            },
            circle_view: CircleView{
                radius,
                color
            }
        }
    }
}

#[derive(Component, Message, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Projectile{
    pub start_tick : Tick,
}

#[derive(Default,Component, Message, Deserialize, Serialize, Clone, Debug, PartialEq, Deref, DerefMut, Add, Mul)]
pub struct SimpleVelocity{
    pub value : Vec3,
}

#[derive(Bundle)]
pub struct ProjectileBundle{
    player_id: PlayerId,
    projectile: Projectile,
    simple_velocity: SimpleVelocity,
    replicated_position : ReplicatedPosition,
    transform_bundle: TransformBundle,
    circle_view: CircleView,
    replicate: Replicate,
    spawn_hash: SpawnHash,
}

impl ProjectileBundle{
    pub fn new(
        owner_client_id: ClientId,
        start_tick : Tick,
        position: Vec3,
        velocity: Vec3
    ) -> Self{
        Self{
            player_id: PlayerId::new(owner_client_id),
            projectile: Projectile{
                start_tick
            },
            simple_velocity: SimpleVelocity{
                value: velocity,
            },
            replicated_position : ReplicatedPosition(position),
            transform_bundle: TransformBundle{
                local: Transform::from_translation(position),
                ..Default::default()
            },
            circle_view: CircleView{
                radius: 0.25,
                color: Color::RED,
            },
            replicate: Replicate{
                prediction_target: NetworkTarget::Only(vec![owner_client_id]),
                replication_group : ReplicationGroup::Group(owner_client_id + 2^16),
                interpolation_target: NetworkTarget::AllExcept(vec![owner_client_id]),
                ..Default::default()
            },
            spawn_hash: SpawnHash{
                hash: 0,
                spawned_tick: start_tick,
            }
            //should_be_predicted: ShouldBePredicted{client_entity: None},
        }
    }
}

// where T is Component
#[derive(Resource)]
pub struct PredictedSpawnIndexing where
{
    pub value : HashMap<SpawnHash, Entity>,
}
pub trait IndexComponent where Self: Component + PartialEq{
    fn get_value(&self) -> u32;
}


// in client, in "push" system_set, after global time update
pub fn destroy_old_predicted_spawns(
    mut commands: Commands,
    predicted_local_spawn : Query<(Entity, &SpawnHash), Without<Replicate>>,
    global_time: Res<GlobalTime>,
){
    for (entity, spawn_hash) in predicted_local_spawn.iter() {
        let age_in_ticks = global_time.simulation_tick.0 - spawn_hash.spawned_tick.0;
        if age_in_ticks > 200{
            commands.entity(entity).despawn_recursive();
        }
    }
}
pub fn destroy_all_predicted_spawns_on_rollback(
    mut commands: Commands,
    predicted_local_spawn : Query<(Entity, &SpawnHash), (Without<Confirmed>,Without<Predicted>)>,
    rollback: Res<Rollback>
){
    match rollback.state {
        RollbackState::Default => {}
        RollbackState::ShouldRollback { .. } => {
            for (entity, spawn_hash) in predicted_local_spawn.iter() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

pub fn destroy_reconciled_predicted_spawns(
    mut commands: Commands,
    local : Query<(Entity, &SpawnHash), With<Simulated>>,
    reconciled : Query<(Entity, &SpawnHash), With<Confirmed>>,
){
    let reconciled : HashSet<SpawnHash> = reconciled.iter().map(|(_, hash)| hash.clone()).collect();

    for (entity, spawn_hash) in local.iter() {
        if reconciled.contains(spawn_hash){
            commands.entity(entity).despawn_recursive();
            log::info!("--------destroying reconciled predicted spawn");
        }
    }
}

pub fn destroy_illegal_replicated_components_on_client(
    query : Query<(Entity, &SpawnHash), With<Replicate>>,
    mut commands: Commands,
    global_time: Res<GlobalTime>
){
    // remove all "Replicated" components that are not in the predicted spawn index
    for (entity, spawn_hash) in query.iter() {
        log::info!("destroying illegal replicated component");
        commands.entity(entity).remove::<Replicate>();
    }
}

// destroy_old_predicted_spawns, destroy_reconciled_predicted_spawns
#[derive(Default,Component, Message, Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SpawnHash{
    pub hash: u32,
    pub spawned_tick: Tick,
}

pub fn see_spawn_hash(
    mut commands: Commands,
    mut query : Query<(Entity, &SpawnHash), Changed<SpawnHash>>,
){
    for (entity, spawn_hash) in query.iter_mut() {
        //log::info!("see spawn hash");
        commands.entity(entity).insert(SeeSpawnHash{
            hash: spawn_hash.hash,
            spawned_tick: *spawn_hash.spawned_tick,
        });
    }
}

#[derive(Default, Component, Debug,Reflect, InspectorOptions)]
pub struct SeeSpawnHash{
    pub hash: u32,
    pub spawned_tick: u16,
}

// impl IndexComponent with this:
//fn get_value(&self) -> u32
//impl IndexComponent for SpawnHash{
//    fn get_value(&self) -> u32 {
//        // hash the hash with the tick
//        self.hash ^ self.spawned_tick.0 as u32
//    }
//}