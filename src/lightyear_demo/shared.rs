use std::time::Duration;
use bevy::log::Level;
use bevy::prelude::{default, Bundle, Color, Component, Deref, DerefMut, Entity, Vec2, Vec3, Plugin, App, FixedUpdate, IntoSystemConfigs};
use bevy::utils::EntityHashSet;
use derive_more::{Add, Mul};
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};
use crate::lightyear_demo::components::*;
use crate::lightyear_demo::systems::{create_replicated_transforms, pull_replicated_positions, push_replicated_positions};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Direction {
    pub(crate) up: bool,
    pub(crate) down: bool,
    pub(crate) left: bool,
    pub(crate) right: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum Inputs {
    Move(Direction),
    Direction(Direction),
    Delete,
    Spawn,
    // NOTE: we NEED to provide a None input so that the server can distinguish between lost input packets and 'None' inputs
    None,
}
impl UserInput for Inputs {}


#[derive(Message, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Message1(pub usize);

#[message_protocol(protocol = "MyProtocol")]
pub enum Messages {
    Message1(Message1),
}



#[derive(Component, Message, Serialize, Deserialize, Clone, Debug, PartialEq, Deref, DerefMut, Add, Mul)]
pub struct PlayerId(ClientId);

impl PlayerId {
    pub fn new(client_id: ClientId) -> Self {
        Self(client_id)
    }
}

#[derive(Component, Message, Serialize, Deserialize, Clone, Debug, PartialEq, Deref, DerefMut, Add, Mul)]
pub struct PlayerPosition(Vec2);

#[derive(Component, Message, Serialize, Deserialize, Clone, Debug, PartialEq, Deref, DerefMut, Add, Mul)]
pub struct ReplicatedPosition(pub Vec3);

#[derive(Component, Message, Deserialize, Serialize, Clone, Debug, PartialEq, Deref, DerefMut, Add, Mul)]
pub struct PlayerColor(pub(crate) Color);

#[component_protocol(protocol = "MyProtocol")]
pub enum Components {
    #[sync(once)]
    PlayerId(PlayerId),
    #[sync(full)]
    PlayerPosition(PlayerPosition),
    #[sync(once)]
    Pawn(Pawn),
    #[sync(once)]
    CircleView(CircleView),
    #[sync(once)]
    PlayerColor(PlayerColor),
    #[sync(full)]
    ReplicatedPosition(ReplicatedPosition),
}


#[derive(Channel)]
pub struct Channel1;


protocolize! {
    Self = MyProtocol,
    Message = Messages,
    Component = Components,
    Input = Inputs,
}

pub(crate) fn protocol() -> MyProtocol {
    let mut protocol = MyProtocol::default();
    protocol.add_channel::<Channel1>(ChannelSettings {
        mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
        direction: ChannelDirection::Bidirectional,
    });
    protocol
}



pub fn shared_config() -> SharedConfig {
    SharedConfig {
        enable_replication: true,
        client_send_interval: Duration::default(),
        server_send_interval: Duration::from_millis(40),
        // server_send_interval: Duration::from_millis(100),
        tick: TickConfig {
            tick_duration: Duration::from_secs_f64(1.0 / 64.0),
        },
        log: LogConfig {
            level: Level::INFO,
            filter: "wgpu=error,wgpu_hal=error,naga=warn,bevy_app=info,bevy_render=warn,quinn=warn"
                .to_string(),
        },
    }
}

// create a

pub struct SharedPlugin;
impl Plugin for SharedPlugin {
    /*
    TickUpdate,    /// Main loop (with physics, game logic) during FixedUpda
    Main,
    MainFlush,
    */
    fn build(&self, app: &mut App) {
        //app.add_systems(FixedUpdate, replicated_position_transform_sync.in_set(FixedUpdateSet::Main));
        app.add_systems(FixedUpdate, (create_replicated_transforms,pull_replicated_positions).chain().in_set(FixedUpdateSet::TickUpdate));
        app.add_systems(FixedUpdate, push_replicated_positions.in_set(FixedUpdateSet::MainFlush));
    }
}