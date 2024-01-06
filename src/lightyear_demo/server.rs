use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::time::Duration;
use lightyear::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::server::*;
use crate::lightyear_demo::SERVER_PORT;
use super::shared::*;


// define a bevy plugin

pub struct DemoClientPlugin;

impl Plugin for DemoClientPlugin {
    fn build(&self, app: &mut App) {
        let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, SERVER_PORT);
        // You can add a link conditioner to simulate network conditions
        let link_conditioner = LinkConditionerConfig {
            incoming_latency: Duration::from_millis(100),
            incoming_jitter: Duration::from_millis(0),
            incoming_loss: 0.00,
        };
        let io_config = IoConfig::from_transport(TransportConfig::UdpSocket(SocketAddr::V4(addr)))
            .with_conditioner(link_conditioner);

        app
            .add_plugins(MinimalPlugins)
            .add_plugins(
                ServerPlugin::new(PluginConfig::new(
                    ServerConfig::default(),
                    Io::from_config(&io_config),
                    protocol()
                )
            ));
    }
}

/*
pub fn shared_config() -> SharedConfig {
    SharedConfig {
        // how often will the server send packets to the client (you can use this to reduce bandwidth used)
        server_send_interval: Duration::from_millis(100),
        // configuration for the FixedUpdate schedule
        tick: TickConfig {
            tick_duration: Duration::from_secs_f64(1.0 / 64.0),
        },
        log: LogConfig {
            level: Level::INFO,
            filter: "wgpu=error,wgpu_hal=error,naga=warn,bevy_app=info,bevy_render=warn"
                .to_string(),
        },
        ..Default::default()
    }
}
*/


/*
pub struct DefenderGamePlugin;

impl Plugin for DefenderGamePlugin{
    fn build(&self, app: &mut App) {
        app
            .add_event::<ProjectileCollisionEvent>()
            .insert_resource(UserInput::default())
            .insert_resource(DefenderGameConfig::default())

            .add_systems(Startup, init)
            .add_systems(Update, (
                // input:
                (
                    take_user_input_system,
                    update_player_tower_input_system
                ).chain(),

                (
                    // game logic, player:
                    player_tower_system,

                    // game logic, projectile:
                    projectile_movement_system,
                    projectile_damage_system,
                    projectile_collision_system,

                    // game logic, enemy:
                    enemy_death_system,
                    enemy_spawner_system,

                    // game logic
                    life_span_system,
                ),

                // display:
                (
                    draw_player_towers,
                    draw_projectiles,
                    draw_enemies
                )
            ).chain());
    }
}*/