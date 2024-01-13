use std::time::Duration;

use bevy::input::Input;
use bevy::log::Level;
use bevy::prelude::{App, Bundle, Color, Commands, Component, Deref, DerefMut, Entity, FixedUpdate, IntoSystemConfigs, KeyCode, Or, Plugin, Query, Res, ResMut, Resource, SystemSet, Update, Vec2, Vec3, With, Without};
use bevy::prelude::IntoSystemSetConfigs;
use derive_more::{Add, Mul};
use lightyear::prelude::*;
use lightyear::prelude::client::{Client, Confirmed, Predicted};
use serde::{Deserialize, Serialize};

use crate::lightyear_demo::components::*;
use crate::lightyear_demo::systems::*;

//use crate::lightyear_demo::systems::pawn_movement;
//use crate::lightyear_demo::systems;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct PawnInputData {
    pub(crate) up: bool,
    pub(crate) down: bool,
    pub(crate) left: bool,
    pub(crate) right: bool,
    pub(crate) attack: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum Inputs {
    Move(PawnInputData),
    //Direction(Direction),
    //Delete,
    //Spawn,
    //// NOTE: we NEED to provide a None input so that the server can distinguish between lost input packets and 'None' inputs
    //None,
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
    pub fn get_client_id(&self) -> ClientId {
        self.0
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
    #[sync(full)]
    Pawn(Pawn),
    #[sync(once)]
    PawnInput(PawnInput),
    #[sync(once)]
    CircleView(CircleView),
    #[sync(once)]
    PlayerColor(PlayerColor),
    #[sync(full)]
    ReplicatedPosition(ReplicatedPosition),
    #[sync(once)]
    Projectile(Projectile),
    #[sync(full)]
    SimpleVelocity(SimpleVelocity),
    #[sync(once)]
    SpawnHash(SpawnHash),
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
    /*
    TickUpdate,    /// Main loop (with physics, game logic) during FixedUpda
    Main,
    MainFlush,
    */
    fn build(&self, app: &mut App) {
        app.insert_resource(GlobalTime{simulation_tick:Tick(0)});

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

        // todo better merge these two systems or run the commands in between them somehow
        //app.add_systems(FixedUpdate, (create_replicated_transforms,pull_replicated_positions).chain()
        //    .in_set(FixedUpdateMainSet::Pull));

        app.add_systems(FixedUpdate, handle_pawn_movement.in_set(FixedUpdateMainSet::Update));
        //app.add_systems(FixedUpdate, handle_pawn_shooting.in_set(FixedUpdateMainSet::Update)); //.after SimulatedTag
        //app.add_systems(Update, handle_pawn_shooting);
        app.add_systems(FixedUpdate, handle_projectile.in_set(FixedUpdateMainSet::Update));



        //app.add_systems(FixedUpdate, push_replicated_positions.in_set(FixedUpdateMainSet::Push));
    }
}


/*
    TickUpdate,    /// Main loop (with physics, game logic) during FixedUpda
    Main,
    MainFlush,
*/

#[derive(Component)]
pub struct Simulated;
pub fn handle_simulated_tag_client(
    to_tag: Query<Entity,(With<Predicted>,Without<Simulated>, Without<SpawnHash>)>,
    to_un_tag: Query<(Entity, &Simulated),(Without<Predicted>, Without<SpawnHash>)>,
    mut commands: Commands,
){
    to_tag.for_each(|entity|{
        commands.entity(entity).insert(Simulated);
    });

    to_un_tag.for_each(|(entity, _)|{
        commands.entity(entity).remove::<Simulated>();
    });
}
pub fn handle_simulated_tag_for_predicted_spawns_client(
    to_tag: Query<Entity,   (Without<Simulated>,With<SpawnHash>,Without<Predicted>,Without<Confirmed>, Without<Replicate>)>,
    to_un_tag: Query<Entity,(With<SpawnHash>,With<Predicted>,   With<Simulated>,    Without<Replicate>)>,
    mut commands: Commands,
) {
    to_tag.for_each(|entity| {
        commands.entity(entity).insert(Simulated);
    });

    //to_un_tag.for_each(|entity| {
    //    commands.entity(entity).remove::<Simulated>();
    //});
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
    let mut input = PawnInputData {
        up: false,
        down: false,
        left: false,
        right: false,
        attack: false,
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
    if keypress.pressed(KeyCode::Space) {
        input.attack = true;
    }

    client.add_input(Inputs::Move(input))

    //// always remember to send an input message
    //return client.add_input(Inputs::None);
}


// T is #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct InputComponent<T>(pub T);

//type MyInputComponent = InputComponent<Direction>;

/*
pub(crate) fn movement(
    mut position_query: Query<&mut PlayerPosition>,
    mut input_reader: EventReader<InputEvent<Inputs>>,
    global: Res<Global>,
    server: Res<Server<MyProtocol>>,
) {
    for input in input_reader.read() {
        let client_id = input.context();
        if let Some(input) = input.input() {
            if let Some(player_entity) = global.client_id_to_entity_id.get(client_id) {
                if let Ok(position) = position_query.get_mut(*player_entity) {
                    shared_movement_behaviour(position, input);
                }
            }
        }
    }
}
*/
//app.add_systems(FixedUpdate, movement.in_set(FixedUpdateSet::Main));



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


#[derive(Resource, Default)]
pub struct GlobalTime{
    pub simulation_tick: Tick,
}