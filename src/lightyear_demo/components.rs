use bevy::prelude::*;
use lightyear::prelude::*;
use crate::lightyear_demo::shared::*;

#[derive(Component, Default, Clone)]
pub struct Pawn{
    pub last_attack_time : f32,
}

#[derive(Component, Clone)]
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