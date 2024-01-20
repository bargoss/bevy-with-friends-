use crate::protocol::*;
use bevy::prelude::*;

use bevy::render::RenderPlugin;
use leafwing_input_manager::action_state::ActionState;
use lightyear::prelude::client::Confirmed;
use lightyear::prelude::*;
use std::ops::Deref;
use std::time::Duration;
use tracing::Level;

pub fn shared_config() -> SharedConfig {
    SharedConfig {
        enable_replication: true,
        client_send_interval: Duration::default(),
        // server_send_interval: Duration::default(),
        server_send_interval: Duration::from_millis(40),
        tick: TickConfig {
            // right now, we NEED the tick_duration to be smaller than the send_interval
            // (otherwise we can send multiple packets for the same tick at different frames)
            tick_duration: Duration::from_secs_f64(1.0 / 64.0),
        },
        log: LogConfig {
            level: Level::INFO,
            filter: "wgpu=error,wgpu_hal=error,naga=warn,bevy_app=info,bevy_render=warn,quinn=warn"
                .to_string(),
        },
    }
}

pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        if app.is_plugin_added::<RenderPlugin>() {
            app.add_systems(Update, (draw_boxes, draw_circles));
        }
    }
}

// This system defines how we update the player's positions when we receive an input
pub(crate) fn shared_movement_behaviour(mut position: Mut<Position>, input: &ActionState<Inputs>) {
    const MOVE_SPEED: f32 = 10.0;
    if input.pressed(Inputs::Up) {
        position.y += MOVE_SPEED;
    }
    if input.pressed(Inputs::Down) {
        position.y -= MOVE_SPEED;
    }
    if input.pressed(Inputs::Left) {
        position.x -= MOVE_SPEED;
    }
    if input.pressed(Inputs::Right) {
        position.x += MOVE_SPEED;
    }
}

/// System that draws the boxed of the player positions.
/// The components should be replicated from the server to the client
/// This time we will only draw the predicted/interpolated entities
pub(crate) fn draw_boxes(
    mut gizmos: Gizmos,
    // players: Query<(&Position, &PlayerColor), Without<Confirmed>>,
    players: Query<(&Position, &PlayerColor)>,
) {
    for (position, color) in &players {
        gizmos.rect(
            Vec3::new(position.x, position.y, 0.0),
            Quat::IDENTITY,
            Vec2::ONE * 50.0,
            color.0,
        );
    }
}

/// System that draws circles
pub(crate) fn draw_circles(mut gizmos: Gizmos, circles: Query<&Position, With<Circle>>) {
    for position in &circles {
        gizmos.circle_2d(*position.deref(), 1.0, Color::GREEN);
    }
}
