use std::marker::PhantomData;

use bevy::prelude::{
    apply_deferred, App, FixedUpdate, IntoSystemConfigs, IntoSystemSetConfigs, Plugin, PostUpdate,
    PreUpdate, Res, SystemSet,
};
use bevy::transform::TransformSystem;

use crate::_reexport::FromType;
use crate::client::components::{SyncComponent, SyncMetadata};
use crate::client::prediction::correction::{
    get_visually_corrected_state, restore_corrected_state,
};
use crate::client::prediction::despawn::{
    despawn_confirmed, remove_component_for_despawn_predicted, remove_despawn_marker,
    restore_components_if_despawn_rolled_back,
};
use crate::client::prediction::predicted_history::update_prediction_history;
use crate::client::prediction::resource::PredictionManager;
use crate::client::resource::Client;
use crate::prelude::ReplicationSet;
use crate::protocol::component::ComponentProtocol;
use crate::protocol::Protocol;
use crate::shared::sets::{FixedUpdateSet, MainSet};

use super::predicted_history::{add_component_history, apply_confirmed_update};
use super::rollback::{check_rollback, increment_rollback_tick, prepare_rollback, run_rollback};
use super::{
    clean_prespawned_entity, handle_pre_prediction, spawn_predicted_entity, ComponentSyncMode,
    Rollback, RollbackState,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct PredictionConfig {
    /// If true, we completely disable the prediction plugin
    pub disable: bool,
    /// If true, we always rollback whenever we receive a server update, instead of checking
    /// ff the confirmed state matches the predicted state history
    pub always_rollback: bool,
    /// The amount of ticks that the player's inputs will be delayed by.
    /// This can be useful to mitigate the amount of client-prediction
    /// This setting is global instead of per Actionlike because it affects how ahead the client will be
    /// compared to the server
    pub input_delay_ticks: u16,
    /// The number of correction ticks will be a multiplier of the number of ticks between
    /// the client and the server correction
    /// (i.e. if the client is 10 ticks head and correction_ticks is 1.0, then the correction will be done over 10 ticks)
    // Number of ticks it will take to visually update the Predicted state to the new Corrected state
    pub correction_ticks_factor: f32,
}

impl PredictionConfig {
    pub fn disable(mut self, disable: bool) -> Self {
        self.disable = disable;
        self
    }

    pub fn always_rollback(mut self, always_rollback: bool) -> Self {
        self.always_rollback = always_rollback;
        self
    }

    /// Update the amount of input delay (number of ticks)
    pub fn with_input_delay_ticks(mut self, tick: u16) -> Self {
        self.input_delay_ticks = tick;
        self
    }

    /// Update the amount of input delay (number of ticks)
    pub fn with_correction_ticks_factor(mut self, factor: f32) -> Self {
        self.correction_ticks_factor = factor;
        self
    }
}

pub struct PredictionPlugin<P: Protocol> {
    config: PredictionConfig,
    // rollback_tick_stragegy:
    // - either we consider that the server sent the entire world state at last_received_server_tick
    // - or not, and we have to check the oldest tick across all components that don't match
    _marker: PhantomData<P>,
}

impl<P: Protocol> PredictionPlugin<P> {
    pub(crate) fn new(config: PredictionConfig) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }
}

impl<P: Protocol> Default for PredictionPlugin<P> {
    fn default() -> Self {
        Self {
            config: PredictionConfig::default(),
            _marker: PhantomData,
        }
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum PredictionSet {
    // PreUpdate Sets
    /// Spawn predicted entities,
    /// We will also use this do despawn predicted entities when confirmed entities are despawned
    SpawnPrediction,
    SpawnPredictionFlush,
    /// Add component history for all predicted entities' predicted components
    SpawnHistory,
    SpawnHistoryFlush,
    RestoreVisualCorrection,
    /// Check if rollback is needed
    CheckRollback,
    /// Prepare rollback by snapping the current state to the confirmed state and clearing histories
    PrepareRollback,
    // we might need a flush because prepare-rollback might remove/add components when snapping the current state
    // to the confirmed state
    PrepareRollbackFlush,
    /// Perform rollback
    Rollback,
    // NOTE: no need to add RollbackFlush because running a schedule (which we do for rollback) will flush all commands at the end of each run

    // FixedUpdate Sets
    /// Increment the rollback tick after the main fixed-update physics loop has run
    IncrementRollbackTick,
    /// Set to deal with predicted/confirmed entities getting despawned
    /// In practice, the entities aren't despawned but all their components are removed
    EntityDespawn,
    /// Remove the marked components that indicates that components should be removekd
    EntityDespawnFlush,
    /// Update the client's predicted history; runs after each physics step in the FixedUpdate Schedule
    UpdateHistory,

    // PostUpdate Sets
    /// Visually interpolate the predicted components to the corrected state
    VisualCorrection,
}

/// Returns true if we are doing rollback
pub fn is_in_rollback(rollback: Res<Rollback>) -> bool {
    matches!(rollback.state, RollbackState::ShouldRollback { .. })
}

/// Returns true if the client is connected
pub fn is_connected<P: Protocol>(client: Res<Client<P>>) -> bool {
    client.is_connected()
}

pub fn add_prediction_systems<C: SyncComponent, P: Protocol>(app: &mut App)
where
    P::ComponentKinds: FromType<C>,
    P::Components: SyncMetadata<C>,
{
    // TODO: maybe create an overarching prediction set that contains all others?
    app.add_systems(
        PreUpdate,
        (
            // handle components being added
            add_component_history::<C, P>.in_set(PredictionSet::SpawnHistory),
        ),
    );
    match P::Components::mode() {
        ComponentSyncMode::Full => {
            app.add_systems(
                PreUpdate,
                // restore to the corrected state (as the visual state might be interpolating
                // between the predicted and corrected state)
                restore_corrected_state::<C>.in_set(PredictionSet::RestoreVisualCorrection),
            );
            app.add_systems(
                PreUpdate,
                (
                    // for SyncMode::Full, we need to check if we need to rollback.
                    check_rollback::<C, P>.in_set(PredictionSet::CheckRollback),
                    prepare_rollback::<C, P>.in_set(PredictionSet::PrepareRollback),
                ),
            );
            app.add_systems(
                FixedUpdate,
                // we need to run this during fixed update to know accurately the history for each tick
                update_prediction_history::<C, P>.in_set(PredictionSet::UpdateHistory),
            );
            app.add_systems(
                PostUpdate,
                get_visually_corrected_state::<C, P>.in_set(PredictionSet::VisualCorrection),
            );
        }
        ComponentSyncMode::Simple => {
            app.add_systems(
                PreUpdate,
                (
                    // for SyncMode::Simple, just copy the confirmed components
                    apply_confirmed_update::<C, P>.in_set(PredictionSet::CheckRollback),
                    // if we are rolling back (maybe because the predicted entity despawn is getting cancelled, restore components)
                    restore_components_if_despawn_rolled_back::<C>
                        // .before(run_rollback::<P>)
                        .in_set(PredictionSet::PrepareRollback),
                ),
            );
        }
        ComponentSyncMode::Once => {
            app.add_systems(
                PreUpdate,
                // if we are rolling back (maybe because the predicted entity despawn is getting cancelled, restore components)
                restore_components_if_despawn_rolled_back::<C>
                    // .before(run_rollback::<P>)
                    .in_set(PredictionSet::PrepareRollback),
            );
        }
        _ => {}
    };
    app.add_systems(
        FixedUpdate,
        remove_component_for_despawn_predicted::<C, P>.in_set(PredictionSet::EntityDespawn),
    );
}

impl<P: Protocol> Plugin for PredictionPlugin<P> {
    fn build(&self, app: &mut App) {
        if self.config.disable {
            return;
        }
        P::Components::add_prediction_systems(app);

        // RESOURCES
        app.init_resource::<PredictionManager>();
        app.insert_resource(Rollback {
            state: RollbackState::Default,
        });

        // PreUpdate systems:
        // 1. Receive confirmed entities, add Confirmed and Predicted components
        app.configure_sets(
            PreUpdate,
            (
                MainSet::ReceiveFlush,
                PredictionSet::SpawnPrediction,
                PredictionSet::SpawnPredictionFlush,
                PredictionSet::SpawnHistory,
                PredictionSet::SpawnHistoryFlush,
                PredictionSet::RestoreVisualCorrection,
                PredictionSet::CheckRollback,
                PredictionSet::PrepareRollback.run_if(is_in_rollback),
                PredictionSet::PrepareRollbackFlush.run_if(is_in_rollback),
                PredictionSet::Rollback.run_if(is_in_rollback),
            )
                .chain(),
        );
        app.add_systems(
            PreUpdate,
            (
                // TODO: we want to run this flushes only if something actually happened in the previous set!
                //  because running the flush-system is expensive (needs exclusive world access)
                //  check how I can do this in bevy
                apply_deferred.in_set(PredictionSet::SpawnPredictionFlush),
                apply_deferred.in_set(PredictionSet::SpawnHistoryFlush),
                apply_deferred.in_set(PredictionSet::PrepareRollbackFlush),
            ),
        );
        app.add_systems(
            FixedUpdate,
            (apply_deferred.in_set(PredictionSet::EntityDespawnFlush),),
        );

        // no need, since we spawn predicted entities/components in replication
        app.add_systems(
            PreUpdate,
            // NOTE: we put `despawn_confirmed` here because we only need to run it once per frame,
            //  not at every fixed-update tick, since it only depends on server messages
            (spawn_predicted_entity::<P>, despawn_confirmed).in_set(PredictionSet::SpawnPrediction),
        );
        app.add_systems(
            PostUpdate,
            (
                // fill in the client_entity and client_id for pre-predicted entities
                handle_pre_prediction::<P>.before(ReplicationSet::All),
                // clean-up the ShouldBePredicted components after we've sent them
                clean_prespawned_entity::<P>.after(ReplicationSet::All),
            )
                .run_if(is_connected::<P>),
        );
        // 2. (in prediction_systems) add ComponentHistory and a apply_deferred after
        // 3. (in prediction_systems) Check if we should do rollback, clear histories and snap prediction's history to server-state
        // 4. Potentially do rollback
        app.add_systems(
            PreUpdate,
            (run_rollback::<P>).in_set(PredictionSet::Rollback),
        );

        // FixedUpdate systems
        // 1. Update client tick (don't run in rollback)
        // 2. Run main physics/game fixed-update loop
        // 3. Increment rollback tick (only run in fallback)
        // 4. Update predicted history
        app.configure_sets(
            FixedUpdate,
            (
                FixedUpdateSet::Main,
                FixedUpdateSet::MainFlush,
                PredictionSet::EntityDespawn,
                PredictionSet::EntityDespawnFlush,
                PredictionSet::UpdateHistory,
                PredictionSet::IncrementRollbackTick.run_if(is_in_rollback),
            )
                .chain(),
        );
        app.add_systems(
            FixedUpdate,
            increment_rollback_tick.in_set(PredictionSet::IncrementRollbackTick),
        );
        app.add_systems(
            FixedUpdate,
            remove_despawn_marker.in_set(PredictionSet::EntityDespawnFlush),
        );

        // PostUpdate systems
        // 1. Visually interpolate the prediction to the corrected state
        app.configure_sets(
            PostUpdate,
            PredictionSet::VisualCorrection.before(TransformSystem::TransformPropagate),
        );
    }
}
