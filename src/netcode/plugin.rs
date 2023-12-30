use std::net::IpAddr;
use bevy::app::*;
use bevy::DefaultPlugins;
use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::*;
use bevy_replicon::prelude::*;
use bevy_replicon::{ReplicationPlugins, server};
use crate::netcode::resources::NetworkConfiguration;

pub struct NetCodePlugin {
    pub network_configuration : NetworkConfiguration
}

fn increment_replicon_tick(mut replicon_tick: ResMut<RepliconTick>) {
    replicon_tick.increment();
}


#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FixedPredictedSchedule;


impl Plugin for NetCodePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(self.network_configuration)

            //.add_plugins(RepliconCorePlugin)
            //.add_plugins(ParentSyncPlugin)
            //.add_plugins(ClientPlugin)
            //.add_plugins(ServerPlugin{
            //    tick_policy: TickPolicy::Manual,
            //    ..Default::default()
            //})
            //.add_systems(FixedUpdate, increment_replicon_tick.run_if(resource_exists::<RenetServer>()))
            .add_plugins(ReplicationPlugins.build().set(ServerPlugin{
                tick_policy: TickPolicy::Manual,
                ..Default::default()
            }))
            .add_systems(FixedUpdate, increment_replicon_tick.run_if(resource_exists::<RenetServer>()))




            //.add_event::<ProjectileCollisionEvent>()
            //.insert_resource(UserInput::default())
            //.insert_resource(DefenderGameConfig::default())

            //.add_systems(Startup, crate::defender_game::systems::init)
            //.add_systems(Update, (
            //).chain());
        ;
    }
}

// a simple test to see if the plugin can run
#[cfg(test)]
mod tests {
    use bevy::MinimalPlugins;
    use super::*;

    #[test]
    fn test_plugin() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(NetCodePlugin {
            network_configuration: NetworkConfiguration::Server {
                port: 1234,
            }
        });

        // println!("app: {:?}", app);
        app.update();
        app.update();
        app.update();
        app.update();

        println!("app: {:?}", app);



    }
}