use bevy::prelude::{Local, Query, Transform};

pub fn cause_mis_predictions(mut transforms: Query<&mut Transform>, mut counter: Local<i32>) {
    for mut transform in &mut transforms.iter_mut() {
        transform.translation += 0.01 * ((*counter % 10) as f32 - 4.5);
    }
    *counter += 1;
}