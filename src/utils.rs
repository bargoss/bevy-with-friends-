use bevy::prelude::{Ray, Vec3};

pub struct Plane {
    pub normal: Vec3,
    pub point: Vec3
}

pub fn ray_plane_intersection(ray: &Ray, plane: &Plane) -> Option<Vec3> {
    let dot = ray.direction.dot(plane.normal);

    if dot.abs() > f32::EPSILON {
        let t = (plane.point - ray.origin).dot(plane.normal) / dot;

        if t > 0.0 {
            let intersection_point = ray.origin + t * ray.direction;
            return Some(intersection_point);
        }
    }

    None
}