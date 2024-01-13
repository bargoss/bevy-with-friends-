//! Specify how a Client sends/receives messages with a Server
use std::time::Duration;

use anyhow::Result;
use bevy::ecs::component::Tick as BevyTick;
use bevy::prelude::World;
use serde::Serialize;
use tracing::{debug, trace, trace_span};

use crate::_reexport::{EntityUpdatesChannel, PingChannel};
use crate::channel::senders::ChannelSend;
use crate::client::sync::SyncConfig;
use crate::connection::events::ConnectionEvents;
use crate::connection::message::{ClientMessage, ServerMessage};
use crate::inputs::native::input_buffer::InputBuffer;
use crate::packet::message_manager::MessageManager;
use crate::packet::packet_manager::Payload;
use crate::prelude::{ChannelKind, MapEntities, NetworkTarget};
use crate::protocol::channel::ChannelRegistry;
use crate::protocol::Protocol;
use crate::serialize::reader::ReadBuffer;
use crate::shared::ping::manager::{PingConfig, PingManager};
use crate::shared::ping::message::SyncMessage;
use crate::shared::replication::receive::ReplicationReceiver;
use crate::shared::replication::send::ReplicationSender;
use crate::shared::replication::ReplicationMessage;
use crate::shared::replication::ReplicationMessageData;
use crate::shared::tick_manager::Tick;
use crate::shared::tick_manager::TickManager;
use crate::shared::time_manager::TimeManager;

use super::sync::SyncManager;

/// Wrapper that handles the connection with the server
pub struct Connection<P: Protocol> {
    pub message_manager: MessageManager,
    pub(crate) replication_sender: ReplicationSender<P>,
    pub(crate) replication_receiver: ReplicationReceiver<P>,
    pub(crate) events: ConnectionEvents<P>,

    pub(crate) ping_manager: PingManager,
    pub(crate) input_buffer: InputBuffer<P::Input>,
    pub(crate) sync_manager: SyncManager,
    // TODO: maybe don't do any replication until connection is synced?
}

impl<P: Protocol> Connection<P> {
    pub fn new(
        channel_registry: &ChannelRegistry,
        sync_config: SyncConfig,
        ping_config: &PingConfig,
        input_delay_ticks: u16,
    ) -> Self {
        // create the message manager and the channels
        let mut message_manager = MessageManager::new(channel_registry);
        // get the acks-tracker for entity updates
        let update_acks_tracker = message_manager
            .channels
            .get_mut(&ChannelKind::of::<EntityUpdatesChannel>())
            .unwrap()
            .sender
            .subscribe_acks();
        let replication_sender = ReplicationSender::new(update_acks_tracker);
        let replication_receiver = ReplicationReceiver::new();
        Self {
            message_manager,
            replication_sender,
            replication_receiver,
            ping_manager: PingManager::new(ping_config),
            input_buffer: InputBuffer::default(),
            sync_manager: SyncManager::new(sync_config, input_delay_ticks),
            events: ConnectionEvents::default(),
        }
    }

    pub(crate) fn clear(&mut self) {
        self.events.clear();
    }

    /// Add an input for the given tick
    pub fn add_input(&mut self, input: P::Input, tick: Tick) {
        self.input_buffer.set(tick, Some(input));
    }

    pub fn update(&mut self, time_manager: &TimeManager, tick_manager: &TickManager) {
        self.message_manager
            .update(time_manager, &self.ping_manager, tick_manager);
        self.ping_manager.update(time_manager);

        // we update the sync manager in POST_UPDATE
        // self.sync_manager.update(time_manager);
    }

    pub(crate) fn buffer_message(
        &mut self,
        message: P::Message,
        channel: ChannelKind,
        target: NetworkTarget,
    ) -> Result<()> {
        // TODO: i know channel names never change so i should be able to get them as static
        // TODO: just have a channel registry enum as well?
        let channel_name = self
            .message_manager
            .channel_registry
            .name(&channel)
            .unwrap_or("unknown")
            .to_string();
        let message = ClientMessage::<P>::Message(message, target);
        message.emit_send_logs(&channel_name);
        self.message_manager.buffer_send(message, channel)?;
        Ok(())
    }

    pub fn buffer_replication_messages(&mut self, tick: Tick, bevy_tick: BevyTick) -> Result<()> {
        // NOTE: this doesn't work too well because then duplicate actions/updates are accumulated before the connection is synced
        // if !self.sync_manager.is_synced() {
        //
        //
        //     // // clear the duplicate component checker
        //     // self.replication_sender.pending_unique_components.clear();
        //     return Ok(());
        // }
        self.replication_sender
            .finalize(tick)
            .into_iter()
            .try_for_each(|(channel, group_id, message_data)| {
                let should_track_ack = matches!(message_data, ReplicationMessageData::Updates(_));
                let channel_name = self
                    .message_manager
                    .channel_registry
                    .name(&channel)
                    .unwrap_or("unknown")
                    .to_string();
                let message = ClientMessage::<P>::Replication(ReplicationMessage {
                    group_id,
                    data: message_data,
                });
                trace!("Sending replication message: {:?}", message);
                message.emit_send_logs(&channel_name);
                let message_id = self
                    .message_manager
                    .buffer_send(message, channel)?
                    .expect("The EntityUpdatesChannel should always return a message_id");

                // keep track of the group associated with the message, so we can handle receiving an ACK for that message_id later
                if should_track_ack {
                    self.replication_sender
                        .updates_message_id_to_group_id
                        .insert(message_id, (group_id, bevy_tick));
                }
                Ok(())
            })
    }

    /// Send packets that are ready to be sent
    pub fn send_packets(
        &mut self,
        time_manager: &TimeManager,
        tick_manager: &TickManager,
    ) -> Result<Vec<Payload>> {
        // update the ping manager with the actual send time
        // TODO: issues here: we would like to send the ping/pong messages immediately, otherwise the recorded current time is incorrect
        //   - can give infinity priority to this channel?
        //   - can write directly to io otherwise?
        if time_manager.is_ready_to_send() {
            // maybe send pings
            // same thing, we want the correct send time for the ping
            // (and not have the delay between when we prepare the ping and when we send the packet)
            if let Some(ping) = self.ping_manager.maybe_prepare_ping(time_manager) {
                trace!("Sending ping {:?}", ping);
                let message = ClientMessage::<P>::Sync(SyncMessage::Ping(ping));
                let channel = ChannelKind::of::<PingChannel>();
                self.message_manager.buffer_send(message, channel)?;
            }

            // prepare the pong messages with the correct send time
            self.ping_manager
                .take_pending_pongs()
                .into_iter()
                .try_for_each(|mut pong| {
                    trace!("Sending pong {:?}", pong);
                    // update the send time of the pong
                    pong.pong_sent_time = time_manager.current_time();
                    let message = ClientMessage::<P>::Sync(SyncMessage::Pong(pong));
                    let channel = ChannelKind::of::<PingChannel>();
                    self.message_manager.buffer_send(message, channel)?;
                    Ok::<(), anyhow::Error>(())
                })?;
        }
        self.message_manager
            .send_packets(tick_manager.current_tick())
    }

    pub fn receive(
        &mut self,
        world: &mut World,
        time_manager: &TimeManager,
        tick_manager: &TickManager,
    ) -> ConnectionEvents<P> {
        let _span = trace_span!("receive").entered();
        for (channel_kind, messages) in self.message_manager.read_messages::<ServerMessage<P>>() {
            let channel_name = self
                .message_manager
                .channel_registry
                .name(&channel_kind)
                .unwrap_or("unknown");
            let _span_channel = trace_span!("channel", channel = channel_name).entered();

            if !messages.is_empty() {
                trace!(?channel_name, "Received messages");
                for (tick, message) in messages.into_iter() {
                    // TODO: we shouldn't map the entities here!
                    //  - we should: order the entities in a group by topological sort (use MapEntities to check dependencies between entities).
                    //  - apply map_entities when we're in the stage of applying to the world.
                    //    - because then we read the first entity in the group; spawn it, and the next component that refers to that entity can be mapped successfully!
                    // map entities from remote to local
                    // message.map_entities(&self.replication_manager.entity_map);

                    // other message-handling logic
                    match message {
                        ServerMessage::Message(mut message) => {
                            // map any entities inside the message
                            message.map_entities(Box::new(
                                &self.replication_receiver.remote_entity_map,
                            ));
                            // buffer the message
                            self.events.push_message(channel_kind, message);
                        }
                        ServerMessage::Replication(replication) => {
                            // buffer the replication message
                            self.replication_receiver.recv_message(replication, tick);
                        }
                        ServerMessage::Sync(ref sync) => {
                            match sync {
                                SyncMessage::Ping(ping) => {
                                    // prepare a pong in response (but do not send yet, because we need
                                    // to set the correct send time)
                                    self.ping_manager.buffer_pending_pong(ping, time_manager);
                                }
                                SyncMessage::Pong(pong) => {
                                    // process the pong
                                    self.ping_manager.process_pong(pong, time_manager);
                                }
                            }
                        }
                    }
                }
                // Check if we have any replication messages we can apply to the World (and emit events)
                if self.sync_manager.is_synced() {
                    for (group, replication_list) in self
                        .replication_receiver
                        .read_messages(tick_manager.current_tick())
                    {
                        trace!(?group, ?replication_list, "read replication messages");
                        replication_list
                            .into_iter()
                            .for_each(|(tick, replication)| {
                                // TODO: we could include the server tick when this replication_message was sent.
                                self.replication_receiver.apply_world(
                                    world,
                                    tick,
                                    replication,
                                    group,
                                    &mut self.events,
                                );
                            });
                    }
                }
            }
        }

        // TODO: do i really need this? I could just create events in this function directly?
        //  why do i need to make events a field of the connection?
        //  is it because of push_connection?
        std::mem::replace(&mut self.events, ConnectionEvents::new())
    }

    pub fn recv_packet(
        &mut self,
        reader: &mut impl ReadBuffer,
        tick_manager: &TickManager,
    ) -> Result<()> {
        self.replication_sender.recv_update_acks();

        let tick = self.message_manager.recv_packet(reader)?;
        debug!("Received server packet with tick: {:?}", tick);
        if self
            .sync_manager
            .latest_received_server_tick
            .map_or(true, |server_tick| tick >= server_tick)
        {
            trace!("new last recv server tick: {:?}", tick);
            self.sync_manager.latest_received_server_tick = Some(tick);
            // TODO: add 'received_new_server_tick' ?
            // we probably actually physically received the packet some time between our last `receive` and now.
            // Let's add delta / 2 as a compromise
            self.sync_manager.duration_since_latest_received_server_tick = Duration::default();
            // self.sync_manager.duration_since_latest_received_server_tick = time_manager.delta() / 2;
            self.sync_manager.update_server_time_estimate(
                tick_manager.config.tick_duration,
                self.ping_manager.rtt(),
            );
        }
        trace!(?tick, last_server_tick = ?self.sync_manager.latest_received_server_tick, "Recv server packet");
        Ok(())
    }
}
