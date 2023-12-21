use bevy::app::App;
use bevy::prelude::{Plugin, Startup, Update};
use bevy::prelude::IntoSystemConfigs;
use crate::defender_game::resources::UserInput;

pub struct SpaceGamePlugin;

impl Plugin for SpaceGamePlugin{
    fn build(&self, app: &mut App) {
        app
            .insert_resource(UserInput::default())
        ;
            //.add_systems(Startup, init)
            //.add_systems(Update, .chain());
    }
}