use bevy::math::Vec3;
use bevy::prelude::{Commands, Entity, Query, Res, Transform, Without};
use lightyear::client::components::Confirmed;
use lightyear::client::interpolation::Interpolated;
use lightyear::prelude::TickManager;
use crate::lightyear_demo::components_old::{Projectile, SimpleVelocity};

pub fn handle_projectile(
    mut projectile_query: Query<(Entity,&mut Projectile, &SimpleVelocity, &mut Transform),(Without<Confirmed>, Without<Interpolated>)>,
    mut commands: Commands,
    tick_manager: Res<TickManager>
){
    projectile_query.for_each_mut(|(entity, mut projectile, velocity, mut replicated_position)|{
        replicated_position.translation += 0.15 * Vec3::new(0.0, -1.0, 0.0);


        if tick_manager.tick().0 - projectile.start_tick.0 > 100 {
            commands.entity(entity).despawn();
        }
    });
}