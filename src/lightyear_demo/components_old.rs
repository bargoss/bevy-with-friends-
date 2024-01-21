use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, AddAssign, Mul};
use bevy::asset::AssetContainer;
use bevy::prelude::*;
use bevy::reflect::{ReflectMut, ReflectOwned, ReflectRef, TypeInfo};
use bevy_inspector_egui::InspectorOptions;
use derive_more::{Add, Mul};
use lightyear::client::components::Confirmed;
use lightyear::client::prediction::{Rollback, RollbackState};
use lightyear::prelude::*;
use lightyear::prelude::client::{Interpolated, LerpFn, Predicted};
use serde::{Deserialize, Serialize};

use crate::lightyear_demo::shared_old::*;

#[derive(Default,Component, Message, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Pawn{
    pub last_attack_time : Tick,
    pub moving : bool
}

impl Add for Pawn{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self{
            last_attack_time: rhs.last_attack_time,
            moving : false
        }
    }
}

impl Mul<f32> for Pawn{
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self{
            last_attack_time: self.last_attack_time,
            moving : self.moving
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
    replicate: Replicate,
    transform : Transform,
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
            replicate: Replicate{
                prediction_target: NetworkTarget::Only(vec![owner_client_id]),
                interpolation_target: NetworkTarget::AllExcept(vec![owner_client_id]),
                replication_group : ReplicationGroup::Group(owner_client_id),
                ..Default::default()
            },
            transform: Transform{
                translation : position,
                ..Default::default()
            },
            circle_view: CircleView{
                radius,
                color
            },
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
    transform: Transform,
    circle_view: CircleView,
    //replicate: Replicate,
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
            transform: Transform{
                translation : position,
                ..Default::default()
            },
            circle_view: CircleView{
                radius: 0.25,
                color: Color::RED,
            },
            //replicate: Replicate{
            //    prediction_target: NetworkTarget::Only(vec![owner_client_id]),
            //    replication_group : ReplicationGroup::Group(owner_client_id),
            //    interpolation_target: NetworkTarget::AllExcept(vec![owner_client_id]),
            //    ..Default::default()
            //},
        }
    }
}