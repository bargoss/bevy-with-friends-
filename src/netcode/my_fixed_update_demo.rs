use bevy::MinimalPlugins;
use bevy::prelude::*;



pub fn demo(){
    App::new()
        .add_plugins(MinimalPlugins)
        .edit_schedule(Main, |schedule| {
            schedule.
        })
        .run();
}