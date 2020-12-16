use amethyst::{
    core::transform::Transform,
    ecs::{Join, System, WriteStorage},
};
use std::f32::NAN;

use crate::boids::{Boid, TransformInfo, BOID_VELOCITY};

pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteStorage<'a, Boid>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, TransformInfo>,
    );

    fn run(&mut self, (mut boids, mut transforms, mut trans_info): Self::SystemData) {
        for (_boid, trans, info) in (&mut boids, &mut transforms, &mut trans_info).join() {
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
            }

            if info.velocities.is_empty() {
                // Move the boid according to its velocity
                trans.move_right(BOID_VELOCITY);
            } else {
                // If any velocities have been added, use them
                let velocities: Vec<f32> = info.velocities.drain(..).collect();
                for velocity in velocities {
                    trans.move_right(velocity);
                }
            }
        }
    }
}
