use bevy::prelude::Component;
use bevy::utils::HashMap;
use lightyear::prelude::Message;
use serde::{Deserialize, Serialize};

#[derive(Default,Component, Message, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ShipState {
    pub block_placements : Vec<(ShipCell, BlockState)>,
}

#[derive(Default,Component)]
pub struct Ship{
    block_placements : Vec<(ShipCell, BlockState)>,
    block_map : HashMap<ShipCell, BlockState>,
}


pub struct ShipCell(i8,i8);
pub struct BlockState {
    pub taken_damage: u8,

}

pub enum BlockType{
    Hull,
    Gyro,
    Engine,
    ShieldGenerator,
    FixedWeapon,
    MobileWeapon,
}