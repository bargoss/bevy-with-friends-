//! Handles client-side prediction
use std::fmt::Debug;

use bevy::prelude::*;
use tracing::{error, info};

pub use despawn::{PredictionCommandsExt, PredictionDespawnMarker};
pub use plugin::add_prediction_systems;
pub use predicted_history::{ComponentState, PredictionHistory};

use crate::client::components::{ComponentSyncMode, Confirmed};
use crate::client::events::ComponentInsertEvent;
use crate::client::prediction::resource::PredictionManager;
use crate::client::resource::Client;
use crate::protocol::Protocol;
use crate::shared::replication::components::{PrePredicted, Replicate, ShouldBePredicted};
use crate::shared::tick_manager::Tick;

pub(crate) mod correction;
mod despawn;
pub mod plugin;
pub mod predicted_history;
mod resource;
pub(crate) mod rollback;

/// Marks an entity that is being predicted by the client
#[derive(Component, Debug)]
pub struct Predicted {
    // This is an option because we could spawn pre-predicted entities on the client that exist before we receive
    // the corresponding confirmed entity
    pub confirmed_entity: Option<Entity>,
}

#[derive(Resource)]
pub struct Rollback {
    pub state: RollbackState,
    // pub rollback_groups: EntityHashMap<ReplicationGroupId, RollbackState>,
}

/// Resource that will track whether we should do rollback or not
/// (We have this as a resource because if any predicted entity needs to be rolled-back; we should roll back all predicted entities)
#[derive(Debug)]
pub enum RollbackState {
    Default,
    ShouldRollback {
        // tick we are setting (to record history)k
        current_tick: Tick,
    },
}

/// For pre-spawned entities, we want to stop replicating as soon as the initial spawn message has been sent to the
/// server. Otherwise any predicted action we would do affect the server entity, even though we want the server to
/// have authority on the entity.
/// Therefore we will remove the `Replicate` component right after the first time we've sent a replicating message to the
/// server
pub(crate) fn clean_prespawned_entity<P: Protocol>(
    client: Res<Client<P>>,
    mut commands: Commands,
    pre_predicted_entities: Query<Entity, With<ShouldBePredicted>>,
) {
    for entity in pre_predicted_entities.iter() {
        debug!(?entity, "removing replicate from pre-spawned entity");
        commands
            .entity(entity)
            .remove::<Replicate<P>>()
            // don't remove should-be-predicted, so that we can know which entities were pre-predicted
            .remove::<ShouldBePredicted>()
            .insert((
                Predicted {
                    confirmed_entity: None,
                },
                // TODO: add this if we want to send inputs for pre-predicted entities before we receive the confirmed entity
                PrePredicted,
            ));
    }
}

/// Spawn a predicted entity for each confirmed entity that has the `ShouldBePredicted` component added
/// The `Confirmed` entity could already exist because we share the Confirmed component for prediction and interpolation.
// TODO: (although normally an entity shouldn't be both predicted and interpolated, so should we
//  instead panic if we find an entity that is both predicted and interpolated?)
pub(crate) fn spawn_predicted_entity<P: Protocol>(
    client: Res<Client<P>>,
    mut manager: ResMut<PredictionManager>,
    mut commands: Commands,
    // get the list of entities who get ShouldBePredicted replicated from server
    mut should_be_predicted_added: EventReader<ComponentInsertEvent<ShouldBePredicted>>,
    mut confirmed_entities: Query<(Entity, Option<&mut Confirmed>, Ref<ShouldBePredicted>)>,
    mut predicted_entities: Query<&mut Predicted>,
) {
    for message in should_be_predicted_added.read() {
        let entity = message.entity();

        if let Ok((confirmed_entity, confirmed, should_be_predicted)) =
            confirmed_entities.get_mut(entity)
        {
            let mut predicted_entity = None;

            // check if we are in a pre-prediction scenario
            let mut should_spawn_predicted = true;
            if let Some(client_entity) = should_be_predicted.client_entity {
                if commands.get_entity(client_entity).is_none() {
                    error!(
                    "The pre-predicted entity has been deleted before we could receive the server's confirmation of it.\
                    This is probably because `EntityCommands::despawn()` has been called.\
                    On `Predicted` entities, you should call `EntityCommands::prediction_despawn()` instead."
                );
                    continue;
                }
                let client_id = should_be_predicted.client_id.unwrap();
                if client_id != client.id() {
                    debug!(
                        local_client = ?client_id,
                        should_be_predicted_client = ?client.id(),
                        "Received ShouldBePredicted component from server for an entity that is pre-predicted by another client: {:?}!", entity);
                } else {
                    // we have a pre-spawned predicted entity! instead of spawning a new predicted entity, we will
                    // just re-use the existing one!
                    should_spawn_predicted = false;
                    predicted_entity = Some(client_entity);
                    debug!(
                        "Re-use pre-spawned predicted entity {:?} for confirmed: {:?}",
                        predicted_entity, confirmed_entity
                    );
                    if let Ok(mut predicted) = predicted_entities.get_mut(client_entity) {
                        predicted.confirmed_entity = Some(confirmed_entity);
                    }

                    #[cfg(feature = "metrics")]
                    {
                        metrics::increment_counter!("prespawn_predicted_entity");
                    }
                }
            }

            if should_spawn_predicted {
                // we need to spawn a predicted entity for this confirmed entity
                let predicted_entity_mut = commands.spawn(Predicted {
                    confirmed_entity: Some(confirmed_entity),
                });
                predicted_entity = Some(predicted_entity_mut.id());
                debug!(
                    "Spawn predicted entity {:?} for confirmed: {:?}",
                    predicted_entity, confirmed_entity
                );
                #[cfg(feature = "metrics")]
                {
                    metrics::increment_counter!("spawn_predicted_entity");
                }
            }

            // update the predicted entity mapping
            let predicted_entity = predicted_entity.unwrap();
            manager
                .predicted_entity_map
                .confirmed_to_predicted
                .insert(confirmed_entity, predicted_entity);

            // add Confirmed to the confirmed entity
            // safety: we know the entity exists
            let mut confirmed_entity_mut = commands.entity(confirmed_entity);
            confirmed_entity_mut.remove::<ShouldBePredicted>();

            if let Some(mut confirmed) = confirmed {
                confirmed.predicted = Some(predicted_entity);
            } else {
                // get the confirmed tick for the entity
                // if we don't have it, something has gone very wrong
                let confirmed_tick = client
                    .replication_receiver()
                    .get_confirmed_tick(confirmed_entity)
                    .unwrap();
                confirmed_entity_mut.insert(Confirmed {
                    predicted: Some(predicted_entity),
                    interpolated: None,
                    tick: confirmed_tick,
                });
            }
        } else {
            error!(
                "Received ShouldBePredicted component from server for an entity that does not exist: {:?}!", entity
            );
        }
    }
}

// TODO: should we run this only when Added<ShouldBePredicted>?
/// If a client adds `ShouldBePredicted` to an entity to perform pre-Prediction.
/// We automatically add the extra needed information to the component.
/// - client_entity: is needed to know which entity to use as the predicted entity
/// - client_id: is needed in case the pre-predicted entity is predicted by other players upon replication
pub(crate) fn handle_pre_prediction<P: Protocol>(
    client: Res<Client<P>>,
    mut query: Query<(Entity, &mut ShouldBePredicted), Without<Confirmed>>,
) {
    for (entity, mut should_be_predicted) in query.iter_mut() {
        assert!(client.is_connected());
        debug!(
            client_id = ?client.id(),
            entity = ?entity,
            "adding pre-prediction info!");
        // TODO: actually we don't need to add the client_entity to the message.
        //  on the server, for pre-predictions, we can just use the entity that was sent in the message to set the value of ClientEntity.
        should_be_predicted.client_entity = Some(entity);
        should_be_predicted.client_id = Some(client.id());
    }
}
