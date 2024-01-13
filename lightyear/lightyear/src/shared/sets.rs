//! Bevy [`SystemSet`] that are shared between the server and client
use bevy::prelude::SystemSet;

/// System sets related to Replication
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum ReplicationSet {
    /// Gathers entity despawns and component removals
    /// Needs to run once per frame instead of once per send_interval
    /// because they rely on bevy events that are cleared every frame
    SendDespawnsAndRemovals,

    // TODO: is it useful to separate these two system sets?
    /// System Set to gather all the replication updates to send
    /// These systems only run once every send_interval
    SendEntityUpdates,
    SendComponentUpdates,

    // SystemSet that encompasses all replication systems
    All,
}

/// Main SystemSets used by lightyear to receive and send data
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum MainSet {
    /// Systems that receive data (buffer any data received from transport, and read
    /// data from the buffers)
    ///
    /// Runs in `PreUpdate`.
    Receive,
    ReceiveFlush,

    /// SystemSet that handles client-replication
    /// On server: You can use this SystemSet to add Replicate components to entities received from clients (to rebroadcast them to other clients)
    ClientReplication,
    ClientReplicationFlush,

    /// Runs once per frame, update sync (client only)
    Sync,
    /// Runs once per frame, clears events (server only)
    ClearEvents,

    /// Systems that send data (buffer any data to be sent, and send any buffered packets)
    ///
    /// Runs in `PostUpdate`.
    SendPackets,
    /// System to encompass all send-related systems. Runs only every send_interval
    Send,
}

/// SystemSet that run during the FixedUpdate schedule
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum FixedUpdateSet {
    /// System that runs at the very start of the FixedUpdate schedule to increment the ticks
    TickUpdate,
    /// Main loop (with physics, game logic) during FixedUpdate
    Main,
    MainFlush,
}
