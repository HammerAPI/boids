//use amethyst::core::math::base::Vector3;
use amethyst::core::math::Point3;
use amethyst::{
    core::transform::Transform,
    ecs::{Join, System, WriteStorage},
};
use std::f32::NAN;

use crate::boids::{Boid, TransformInfo};

pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteStorage<'a, Boid>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, TransformInfo>,
    );

    fn run(&mut self, (mut boids, mut transforms, mut trans_info): Self::SystemData) {
        for (boid, trans, info) in (&mut boids, &mut transforms, &mut trans_info).join() {
            // If the boid needs to be moved to the other side of the arena, do that
            if info.new_y.is_finite() {
                trans.set_translation_y(info.new_y);
                info.new_y = NAN;
            }
            if info.new_x.is_finite() {
                trans.set_translation_x(info.new_x);
                info.new_x = NAN;
            }

            // Rotate boid, given the list of angles assigned to it
            let angles: Vec<f32> = info.angles.drain(..).collect();
            for angle in angles {
                trans.rotate_2d(angle);
                // Move ray direction
                //boid.ray.direction = Vector3::new(0.0, trans.euler_angles().2, 0.0);
            }

            // Move ray position
            boid.ray.origin = Point3::new(trans.translation().x, trans.translation().y, 0.0);

            if info.velocities.is_empty() {
                // Move the boid according to its velocity
                trans.move_up(boid.velocity);
            } else {
                // If any velocities have been added, use them
                let velocities: Vec<f32> = info.velocities.drain(..).collect();
                for velocity in velocities {
                    trans.move_up(velocity);
                }
            }
        }
    }
}
