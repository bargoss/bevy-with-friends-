use bevy::prelude::{EventReader, Query, With};
use lightyear::client::prediction::Predicted;
use lightyear::shared::events::InputEvent;
use crate::lightyear_demo::components_old::{Pawn, PawnInput};
use crate::lightyear_demo::shared_old::Inputs;

pub fn handle_pawn_input_client(
    mut pawn_query: Query<(&Pawn, &mut PawnInput), With<Predicted>>,
    mut input_reader: EventReader<InputEvent<Inputs>>,
){
    for input in input_reader.read() {
        if let Some(input) = input.input() {
            for mut pawn in pawn_query.iter_mut() {
                match input {
                    Inputs::None => {}
                    Inputs::PawnInputData(pawn_input_data) => {
                        pawn.1.movement_direction = pawn_input_data.direction_input.to_vec3();
                        pawn.1.attack = pawn_input_data.attack;
                    }
                }
            }
        }
    }
}