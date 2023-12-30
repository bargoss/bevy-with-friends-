use bevy::prelude::*;
use bevy_replicon::renet::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize)]
struct Player(ClientId);


#[derive(Component, Deserialize, Serialize, Deref, DerefMut)]
struct PlayerPosition(Vec2);

#[derive(Component, Deserialize, Serialize)]
struct PlayerColor(Color);

/// A movement event for the controlled box.
#[derive(Debug, Default, Deserialize, Event, Serialize)]
struct MoveDirection(Vec2);