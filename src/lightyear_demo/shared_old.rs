use std::time::Duration;

use bevy::input::Input;
use bevy::log::Level;
use bevy::prelude::{App, Bundle, Color, Commands, Component, default, Deref, DerefMut, Entity, FixedUpdate, IntoSystemConfigs, KeyCode, Or, Plugin, Query, Reflect, Res, ResMut, Resource, SystemSet, Transform, Update, Vec2, Vec3, With, Without};
use bevy::prelude::IntoSystemSetConfigs;
use derive_more::{Add, Mul};
use lightyear::prelude::*;
use lightyear::prelude::client::{Confirmed, Interpolated, Predicted};
use serde::{Deserialize, Serialize};

use crate::lightyear_demo::components_old::*;
use crate::lightyear_demo::systems::*;

use lightyear::utils::bevy::TransformLinearInterpolation;


#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum Inputs {
    #[default]
    None,
    PawnInputData(PawnInputData),
}
#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct PawnInputData{
    pub direction_input : DirectionInput,
    pub attack : bool,
}

impl UserAction for Inputs {}

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
    pub fn get_client_id(&self) -> ClientId {
        self.0
    }
}

#[derive(Component, Message, Serialize, Deserialize, Clone, Debug, PartialEq, Deref, DerefMut, Add, Mul)]
pub struct PlayerPosition(Vec2);

#[derive(Component, Message, Deserialize, Serialize, Clone, Debug, PartialEq, Deref, DerefMut, Add, Mul)]
pub struct PlayerColor(pub(crate) Color);

#[component_protocol(protocol = "MyProtocol")]
pub enum Components {
    #[sync(once)]
    PlayerId(PlayerId),
    #[sync(full)]
    PlayerPosition(PlayerPosition),
    #[sync(full)]
    Pawn(Pawn),
    #[sync(once)]
    PawnInput(PawnInput),
    #[sync(once)]
    CircleView(CircleView),
    #[sync(once)]
    PlayerColor(PlayerColor),
    #[sync(once)]
    Projectile(Projectile),
    #[sync(full)]
    SimpleVelocity(SimpleVelocity),
    #[sync(full, lerp = "TransformLinearInterpolation")]
    Transform(Transform),
}

//Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, Hash, Reflect
//Message, Deserialize, Serialize, Clone, Debug, PartialEq
//union:
#[derive(Default, Component, Message, Deserialize, Serialize, Eq, Hash, Copy, Clone, Debug, PartialEq, Reflect)]
pub enum DirectionInput{
    #[default]
    None,
    Up,
    Down,
    Left,
    Right,
}


impl DirectionInput {
    pub fn new() -> Self {
        Self::None
    }
    pub fn to_vec3(&self) -> Vec3 {
        match self {
            DirectionInput::Up => Vec3::new(0.0, 1.0, 0.0),
            DirectionInput::Down => Vec3::new(0.0, -1.0, 0.0),
            DirectionInput::Left => Vec3::new(-1.0, 0.0, 0.0),
            DirectionInput::Right => Vec3::new(1.0, 0.0, 0.0),
            DirectionInput::None => Vec3::new(0.0, 0.0, 0.0),
        }
    }
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

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum FixedUpdateMainSet {
    Pull,
    AfterPull,
    Update,
    Push,
}

pub struct SharedPlugin;
impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            FixedUpdate,
            (
                FixedUpdateMainSet::Pull,
                FixedUpdateMainSet::Update,
                FixedUpdateMainSet::Push
            )
            .chain()
            .in_set(FixedUpdateSet::Main)
        );


        app.add_systems(FixedUpdate, handle_simulated_tag.in_set(FixedUpdateMainSet::Pull));

        app.add_systems(FixedUpdate, handle_pawn_movement.in_set(FixedUpdateMainSet::Update));
        app.add_systems(FixedUpdate, handle_pawn_shooting.in_set(FixedUpdateMainSet::Update)); //.after SimulatedTag
        app.add_systems(FixedUpdate, handle_projectile.in_set(FixedUpdateMainSet::Update));
    }
}

pub fn handle_simulated_tag(
    to_tag: Query<Entity,(Without<Simulated>, Or<(With<Predicted>, With<Replicate>)>)>,
    to_un_tag: Query<Entity,(With<Simulated>, Without<Predicted>, Without<Replicate>)>,
    mut commands: Commands,
)
{
    to_tag.for_each(|entity| {
        commands.entity(entity).insert(Simulated);
    });

    to_un_tag.for_each(|entity| {
        commands.entity(entity).remove::<Simulated>();
    });
}

#[derive(Component)]
pub struct Simulated;
