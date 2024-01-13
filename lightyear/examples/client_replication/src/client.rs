use crate::protocol::Direction;
use crate::protocol::*;
use crate::shared::{color_from_id, shared_config, shared_movement_behaviour};
use crate::{Transports, KEY, PROTOCOL_ID};
use bevy::prelude::*;
use lightyear::_reexport::ShouldBePredicted;
use lightyear::prelude::client::*;
use lightyear::prelude::*;
use std::net::{Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::time::Duration;

#[derive(Resource, Clone, Copy)]
pub struct MyClientPlugin {
    pub(crate) client_id: ClientId,
    pub(crate) client_port: u16,
    pub(crate) server_addr: Ipv4Addr,
    pub(crate) server_port: u16,
    pub(crate) transport: Transports,
}

impl Plugin for MyClientPlugin {
    fn build(&self, app: &mut App) {
        let server_addr = SocketAddr::new(self.server_addr.into(), self.server_port);
        let auth = Authentication::Manual {
            server_addr,
            client_id: self.client_id,
            private_key: KEY,
            protocol_id: PROTOCOL_ID,
        };
        let client_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), self.client_port);
        let link_conditioner = LinkConditionerConfig {
            incoming_latency: Duration::from_millis(0),
            incoming_jitter: Duration::from_millis(0),
            incoming_loss: 0.0,
        };
        let transport = match self.transport {
            Transports::Udp => TransportConfig::UdpSocket(client_addr),
            Transports::Webtransport => TransportConfig::WebTransportClient {
                client_addr,
                server_addr,
            },
        };
        let io =
            Io::from_config(IoConfig::from_transport(transport).with_conditioner(link_conditioner));
        let config = ClientConfig {
            shared: shared_config().clone(),
            input: InputConfig::default(),
            netcode: Default::default(),
            ping: PingConfig::default(),
            sync: SyncConfig::default(),
            prediction: PredictionConfig::default(),
            // we are sending updates every frame (60fps), let's add a delay of 6 network-ticks
            interpolation: InterpolationConfig::default()
                .with_delay(InterpolationDelay::default().with_send_interval_ratio(2.0)),
        };
        let plugin_config = PluginConfig::new(config, io, protocol(), auth);
        app.add_plugins(ClientPlugin::new(plugin_config));
        app.add_plugins(crate::shared::SharedPlugin);
        app.insert_resource(self.clone());
        app.add_systems(Startup, init);
        app.add_systems(
            FixedUpdate,
            buffer_input.in_set(InputSystemSet::BufferInputs),
        );
        // all actions related-system that can be rolled back should be in FixedUpdateSet::Main
        app.add_systems(
            FixedUpdate,
            (player_movement, delete_player).in_set(FixedUpdateSet::Main),
        );
        app.add_systems(
            Update,
            (
                cursor_movement,
                receive_message,
                send_message,
                spawn_player,
                handle_predicted_spawn,
                handle_interpolated_spawn,
            ),
        );
    }
}

// Startup system for the client
pub(crate) fn init(
    mut commands: Commands,
    mut client: ResMut<Client>,
    plugin: Res<MyClientPlugin>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(TextBundle::from_section(
        format!("Client {}", plugin.client_id),
        TextStyle {
            font_size: 30.0,
            color: Color::WHITE,
            ..default()
        },
    ));
    // spawn a local cursor which will be replicated to other clients, but remain client-authoritative.
    commands.spawn(CursorBundle::new(
        plugin.client_id,
        Vec2::ZERO,
        color_from_id(plugin.client_id),
    ));
    client.connect();
    // client.set_base_relative_speed(0.001);
}

// System that reads from peripherals and adds inputs to the buffer
pub(crate) fn buffer_input(mut client: ResMut<Client>, keypress: Res<Input<KeyCode>>) {
    let mut direction = Direction {
        up: false,
        down: false,
        left: false,
        right: false,
    };
    if keypress.pressed(KeyCode::W) || keypress.pressed(KeyCode::Up) {
        direction.up = true;
    }
    if keypress.pressed(KeyCode::S) || keypress.pressed(KeyCode::Down) {
        direction.down = true;
    }
    if keypress.pressed(KeyCode::A) || keypress.pressed(KeyCode::Left) {
        direction.left = true;
    }
    if keypress.pressed(KeyCode::D) || keypress.pressed(KeyCode::Right) {
        direction.right = true;
    }
    if !direction.is_none() {
        return client.add_input(Inputs::Direction(direction));
    }
    if keypress.pressed(KeyCode::K) {
        // currently, directions is an enum and we can only add one input per tick
        return client.add_input(Inputs::Delete);
    }
    if keypress.pressed(KeyCode::Space) {
        return client.add_input(Inputs::Spawn);
    }
    return client.add_input(Inputs::None);
}

// The client input only gets applied to predicted entities that we own
// This works because we only predict the user's controlled entity.
// If we were predicting more entities, we would have to only apply movement to the player owned one.
fn player_movement(
    mut position_query: Query<&mut PlayerPosition, With<Predicted>>,
    // InputEvent is a special case: we get an event for every fixed-update system run instead of every frame!
    mut input_reader: EventReader<InputEvent<Inputs>>,
) {
    if <Components as SyncMetadata<PlayerPosition>>::mode() != ComponentSyncMode::Full {
        return;
    }
    for input in input_reader.read() {
        if let Some(input) = input.input() {
            for position in position_query.iter_mut() {
                // NOTE: be careful to directly pass Mut<PlayerPosition>
                // getting a mutable reference triggers change detection, unless you use `as_deref_mut()`
                shared_movement_behaviour(position, input);
            }
        }
    }
}

/// Spawn a player when the space command is pressed
fn spawn_player(
    mut commands: Commands,
    mut input_reader: EventReader<InputEvent<Inputs>>,
    plugin: Res<MyClientPlugin>,
    players: Query<&PlayerId, With<PlayerPosition>>,
) {
    // do not spawn a new player if we already have one
    for player_id in players.iter() {
        if player_id.0 == plugin.client_id {
            return;
        }
    }
    for input in input_reader.read() {
        if let Some(input) = input.input() {
            match input {
                Inputs::Spawn => {
                    debug!("got spawn input");
                    commands.spawn((
                        PlayerBundle::new(
                            plugin.client_id,
                            Vec2::ZERO,
                            color_from_id(plugin.client_id),
                        ),
                        // IMPORTANT: this lets the server know that the entity is pre-predicted
                        // when the server replicates this entity; we will get a Confirmed entity which will use this entity
                        // as the Predicted version
                        ShouldBePredicted::default(),
                    ));
                }
                _ => {}
            }
        }
    }
}

/// Delete the predicted player when the space command is pressed
fn delete_player(
    mut commands: Commands,
    mut input_reader: EventReader<InputEvent<Inputs>>,
    plugin: Res<MyClientPlugin>,
    players: Query<
        (Entity, &PlayerId),
        (
            With<PlayerPosition>,
            Without<Confirmed>,
            Without<Interpolated>,
        ),
    >,
) {
    for input in input_reader.read() {
        if let Some(input) = input.input() {
            match input {
                Inputs::Delete => {
                    for (entity, player_id) in players.iter() {
                        if player_id.0 == plugin.client_id {
                            if let Some(mut entity_mut) = commands.get_entity(entity) {
                                // we need to use this special function to despawn prediction entity
                                // the reason is that we actually keep the entity around for a while,
                                // in case we need to re-store it for rollback
                                entity_mut.prediction_despawn::<MyProtocol>();
                                debug!("Despawning the predicted/pre-predicted player because we received player action!");
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

// Adjust the movement of the cursor entity based on the mouse position
fn cursor_movement(
    plugin: Res<MyClientPlugin>,
    window_query: Query<&Window>,
    mut cursor_query: Query<(&mut CursorPosition, &PlayerId)>,
) {
    for (mut cursor_position, player_id) in cursor_query.iter_mut() {
        if player_id.0 != plugin.client_id {
            return;
        }
        if let Ok(window) = window_query.get_single() {
            if let Some(mouse_position) = window_relative_mouse_position(window) {
                // only update the cursor if it's changed
                cursor_position.set_if_neq(CursorPosition(mouse_position));
            }
        }
    }
}

// Get the cursor position relative to the window
fn window_relative_mouse_position(window: &Window) -> Option<Vec2> {
    let Some(cursor_pos) = window.cursor_position() else {
        return None;
    };

    Some(Vec2::new(
        cursor_pos.x - (window.width() / 2.0),
        (cursor_pos.y - (window.height() / 2.0)) * -1.0,
    ))
}

// System to receive messages on the client
pub(crate) fn receive_message(mut reader: EventReader<MessageEvent<Message1>>) {
    for event in reader.read() {
        info!("Received message: {:?}", event.message());
    }
}

/// Send messages from server to clients
pub(crate) fn send_message(mut client: ResMut<Client>, input: Res<Input<KeyCode>>) {
    if input.pressed(KeyCode::M) {
        let message = Message1(5);
        info!("Send message: {:?}", message);
        // the message will be re-broadcasted by the server to all clients
        client
            .send_message_to_target::<Channel1, Message1>(Message1(5), NetworkTarget::All)
            .unwrap_or_else(|e| {
                error!("Failed to send message: {:?}", e);
            });
    }
}

// When the predicted copy of the client-owned entity is spawned, do stuff
// - assign it a different saturation
// - keep track of it in the Global resource
pub(crate) fn handle_predicted_spawn(mut predicted: Query<&mut PlayerColor, Added<Predicted>>) {
    for mut color in predicted.iter_mut() {
        color.0.set_s(0.4);
    }
}

// When the predicted copy of the client-owned entity is spawned, do stuff
// - assign it a different saturation
// - keep track of it in the Global resource
pub(crate) fn handle_interpolated_spawn(
    mut interpolated: Query<&mut PlayerColor, Added<Interpolated>>,
) {
    for mut color in interpolated.iter_mut() {
        color.0.set_s(0.1);
    }
}
