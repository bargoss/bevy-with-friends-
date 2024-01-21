use bevy::prelude::{EventReader, Query, Res};
use lightyear::netcode::ClientId;
use lightyear::shared::events::InputEvent;
use crate::lightyear_demo::components_old::{Pawn, PawnInput};
use crate::lightyear_demo::server::Global;
use crate::lightyear_demo::shared_old::Inputs;

pub fn handle_pawn_input_server(
    mut pawn_query: Query<(&Pawn, &mut PawnInput)>,
    mut input_reader: EventReader<InputEvent<Inputs, ClientId>>,
    global: Res<Global>,
){
    for input in input_reader.read() {
        let client_id = input.context();
        if let Some(input) = input.input() {
            if let Some(player_entity) = global.client_id_to_entity_id.get(client_id) {
                if let Ok(mut data) = pawn_query.get_mut(*player_entity) {
                    if let Inputs::PawnInputData(pawn_input_data) = input {
                        data.1.movement_direction = pawn_input_data.direction_input.to_vec3();
                        data.1.attack = pawn_input_data.attack;
                    }
                }
            }
        }
    }
}