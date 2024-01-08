use bevy::prelude::*;
use derive_more::{Add, Mul};
use lightyear::_reexport::{InterpolatedComponent, LinearInterpolation};
use lightyear::client::interpolation::InterpFn;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

use crate::lightyear_demo::shared::*;

//#[derive(Component, Default, Clone)]
#[derive(Default,Component, Message, Deserialize, Serialize, Clone, Debug, PartialEq, Deref, DerefMut)]
pub struct Pawn{
    pub last_attack_time : Tick,
}

// Assuming `InterpFn` needs to be implemented for Pawn
impl InterpFn<Pawn> for LinearInterpolation {
    fn lerp(start: Pawn, other: Pawn, t: f32) -> Pawn {
        other
    }
}

//impl InterpolatedComponent<Pawn> for Pawn{
//    type Fn = LinearInterpolation;
//    fn lerp(start: Pawn, other: Pawn, t: f32) -> Pawn {
//        other
//    }
//}

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
    owner_id: PlayerId,
    projectile: Projectile,
    simple_velocity: SimpleVelocity,
    replicated_position : ReplicatedPosition,
    transform_bundle: TransformBundle,
    circle_view: CircleView,
    replicate: Replicate,
}

impl ProjectileBundle{
    pub fn new(
        owner_client_id: ClientId,
        start_tick : Tick,
        position: Vec3,
        velocity: Vec3,
    ) -> Self{
        Self{
            owner_id: PlayerId::new(owner_client_id),
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
                interpolation_target: NetworkTarget::AllExcept(vec![owner_client_id]),
                ..Default::default()
            },
        }
    }
}