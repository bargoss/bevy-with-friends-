//! Handles interpolation of entities between server updates
use std::ops::{Add, Mul};

use bevy::prelude::{Added, Commands, Component, Entity, Query, Res, ResMut};
use tracing::debug;

pub use interpolate::InterpolateStatus;
pub use interpolation_history::ConfirmedHistory;
pub use plugin::{add_interpolation_systems, add_prepare_interpolation_systems};

use crate::client::components::{Confirmed, LerpFn, SyncComponent};
use crate::client::interpolation::resource::InterpolationManager;
use crate::client::resource::Client;
use crate::protocol::Protocol;
use crate::shared::replication::components::ShouldBeInterpolated;

mod despawn;
mod interpolate;
pub mod interpolation_history;
pub mod plugin;
mod resource;

pub struct LinearInterpolator;
impl<C> LerpFn<C> for LinearInterpolator
where
    C: Mul<f32, Output = C> + Add<C, Output = C>,
{
    fn lerp(start: C, other: C, t: f32) -> C {
        start * (1.0 - t) + other * t
    }
}

/// Use this if you don't want to use an interpolation function for this component.
/// (For example if you are running your own interpolation logic)
pub struct NullInterpolator;
impl<C> LerpFn<C> for NullInterpolator {
    fn lerp(start: C, _other: C, _t: f32) -> C {
        start
    }
}

/// Marks an entity that is being interpolated by the client
#[derive(Component, Debug)]
pub struct Interpolated {
    // TODO: maybe here add an interpolation function?
    pub confirmed_entity: Entity,
    // TODO: add config about despawn behaviour here:
    //  - despawn immediately all components
    //  - leave the entity alive until the confirmed entity catches up to it and then it gets removed.
    //    - or do this only for certain components (audio, animation, particles..) -> mode on PredictedComponent
}

pub fn spawn_interpolated_entity<P: Protocol>(
    client: Res<Client<P>>,
    mut manager: ResMut<InterpolationManager>,
    mut commands: Commands,
    mut confirmed_entities: Query<(Entity, Option<&mut Confirmed>), Added<ShouldBeInterpolated>>,
) {
    for (confirmed_entity, confirmed) in confirmed_entities.iter_mut() {
        // spawn a new interpolated entity
        let interpolated_entity_mut = commands.spawn(Interpolated { confirmed_entity });
        let interpolated = interpolated_entity_mut.id();

        // update the entity mapping
        manager
            .interpolated_entity_map
            .confirmed_to_interpolated
            .insert(confirmed_entity, interpolated);

        // add Confirmed to the confirmed entity
        // safety: we know the entity exists
        let mut confirmed_entity_mut = commands.get_entity(confirmed_entity).unwrap();
        if let Some(mut confirmed) = confirmed {
            confirmed.interpolated = Some(interpolated);
        } else {
            // get the confirmed tick for the entity
            // if we don't have it, something has gone very wrong
            let confirmed_tick = client
                .replication_receiver()
                .get_confirmed_tick(confirmed_entity)
                .unwrap();
            confirmed_entity_mut.insert(Confirmed {
                interpolated: Some(interpolated),
                predicted: None,
                tick: confirmed_tick,
            });
        }
        debug!(
            "Spawn interpolated entity {:?} for confirmed: {:?}",
            interpolated, confirmed_entity
        );
        #[cfg(feature = "metrics")]
        {
            metrics::increment_counter!("spawn_interpolated_entity");
        }
    }
}
