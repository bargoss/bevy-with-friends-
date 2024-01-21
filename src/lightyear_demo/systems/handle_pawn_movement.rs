use bevy::prelude::{Or, Query, Transform, With};
use lightyear::client::prediction::Predicted;
use crate::lightyear_demo::components_old::{Pawn, PawnInput};
use crate::lightyear_demo::shared_old::Replicate;

pub fn handle_pawn_movement(
    mut pawn_query: Query<(&Pawn, &PawnInput, &mut Transform), Or<(With<Predicted>, With<Replicate>)>>,
){
    let speed = 0.25;
    pawn_query.for_each_mut(|(_, pawn_input, mut replicated_position)|{
        replicated_position.translation += pawn_input.movement_direction * speed;
    });
}