use bevy::input::Input;
use bevy::prelude::{KeyCode, Res};
use crate::lightyear_demo::shared_old::{ClientMut, DirectionInput, Inputs, PawnInputData};

pub fn buffer_input(mut client: ClientMut, keypress: Res<Input<KeyCode>>) {
    let mut input = PawnInputData {
        direction_input: DirectionInput::None,
        attack: false,
    };
    if keypress.pressed(KeyCode::W) || keypress.pressed(KeyCode::Up) {
        input.direction_input = DirectionInput::Up;
    }
    if keypress.pressed(KeyCode::S) || keypress.pressed(KeyCode::Down) {
        input.direction_input = DirectionInput::Down;
    }
    if keypress.pressed(KeyCode::A) || keypress.pressed(KeyCode::Left) {
        input.direction_input = DirectionInput::Left;
    }
    if keypress.pressed(KeyCode::D) || keypress.pressed(KeyCode::Right) {
        input.direction_input = DirectionInput::Right;
    }
    if keypress.pressed(KeyCode::Space) {
        input.attack = true;
    }

    // always remember to send an input message
    client.add_input(Inputs::PawnInputData(input));
}