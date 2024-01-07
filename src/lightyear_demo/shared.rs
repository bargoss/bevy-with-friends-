use std::time::Duration;
use bevy::input::Input;
use bevy::log::Level;
use bevy::prelude::{default, Bundle, Color, Component, Deref, DerefMut, Entity, Vec2, Vec3, Plugin, App, FixedUpdate, IntoSystemConfigs, Commands, Transform, Without, Query, TransformBundle, ResMut, KeyCode, Res, With};
use bevy::utils::EntityHashSet;
use derive_more::{Add, Mul};
use lightyear::prelude::*;
use lightyear::prelude::client::{Client, InputSystemSet, Predicted};
use serde::{Deserialize, Serialize};
use crate::lightyear_demo::components::*;
//use crate::lightyear_demo::systems::pawn_movement;
//use crate::lightyear_demo::systems;

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

        //app.add_systems(FixedUpdate, pawn_movement.in_set(FixedUpdateSet::Main));

        app.add_systems(FixedUpdate, push_replicated_positions.in_set(FixedUpdateSet::MainFlush));
    }
}

/*
    TickUpdate,    /// Main loop (with physics, game logic) during FixedUpda
    Main,
    MainFlush,
*/

pub fn create_replicated_transforms(
    mut commands: Commands,
    // if it doesnt have transform
    query: Query<(Entity, &ReplicatedPosition), Without<Transform>>,
){
    query.for_each(|(entity, replicated_position)|{
        commands.entity(entity).insert(TransformBundle{
            local: Transform::from_translation(Vec3::new(replicated_position.0.x, replicated_position.0.y, replicated_position.0.z)),
            ..Default::default()
        });
    });
}
pub fn pull_replicated_positions(
    mut query: Query<(&ReplicatedPosition, &mut Transform)>,
){
    query.for_each_mut(|(replicated_position, mut transform)|{
        transform.translation.x = replicated_position.0.x;
        transform.translation.y = replicated_position.0.y;
        transform.translation.z = replicated_position.0.z;
    });
}
pub fn push_replicated_positions(
    mut query: Query<(&mut ReplicatedPosition, &Transform)>,
){
    query.for_each_mut(|(mut replicated_position, transform)|{
        replicated_position.0.x = transform.translation.x;
        replicated_position.0.y = transform.translation.y;
        replicated_position.0.z = transform.translation.z;
    });
}
#[derive(Component)]
pub struct Simulated;
pub fn handle_simulated_tag_client(
    //query: Query<(Entity), With<Predicted>, Without<Simulated>>
    to_tag: Query<(Entity, &Predicted),Without<Simulated>>,
    to_un_tag: Query<(Entity, &Simulated),Without<Predicted>>,
    mut commands: Commands,
){
    to_tag.for_each(|(entity, _)|{
        commands.entity(entity).insert(Simulated);
    });

    to_un_tag.for_each(|(entity, _)|{
        commands.entity(entity).remove::<Simulated>();
    });
}
pub fn handle_simulated_tag_server(
    to_tag: Query<(Entity, &Replicate),Without<Simulated>>,
    to_un_tag: Query<(Entity, &Simulated),Without<Replicate>>,
    mut commands: Commands,
){
    to_tag.for_each(|(entity, _)|{
        commands.entity(entity).insert(Simulated);
    });

    to_un_tag.for_each(|(entity, _)|{
        commands.entity(entity).remove::<Simulated>();
    });
}


pub(crate) fn buffer_input(mut client: ResMut<Client<MyProtocol>>, keypress: Res<Input<KeyCode>>) {
    let mut input = Direction {
        up: false,
        down: false,
        left: false,
        right: false,
    };
    if keypress.pressed(KeyCode::W) || keypress.pressed(KeyCode::Up) {
        input.up = true;
    }
    if keypress.pressed(KeyCode::S) || keypress.pressed(KeyCode::Down) {
        input.down = true;
    }
    if keypress.pressed(KeyCode::A) || keypress.pressed(KeyCode::Left) {
        input.left = true;
    }
    if keypress.pressed(KeyCode::D) || keypress.pressed(KeyCode::Right) {
        input.right = true;
    }

    client.add_input(Inputs::Direction(input))

    //// always remember to send an input message
    //return client.add_input(Inputs::None);
}


// T is #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct InputComponent<T>(pub T);

type MyInputComponent = InputComponent<Direction>;


//pub fn sync_input_component_server<T>(
//    mut query: Query<(Entity, &mut InputComponent<T>), With<Predicted>>,
//    mut input_reader: EventReader<InputEvent<Inputs>>,
//    global: Res<Global>,
//    server: Res<Server<MyProtocol>>,
//) {
//    for (entity, input, player_id) in query.iter_mut() {
//        client.add_input_with_context(*player_id, input.clone());
//    }
//}