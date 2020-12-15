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
            }

            if info.velocities.is_empty() {
                // Move the boid according to its velocity
                trans.move_right(boid.velocity);
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

/*
use amethyst::{
    core::transform::Transform,
    ecs::{Join, System, WriteStorage},
};
//use std::f32::NAN;

use crate::boids::{Boid, BOID_SIGHT};

pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (WriteStorage<'a, Boid>, WriteStorage<'a, Transform>);

    fn run(&mut self, (mut boids, mut transforms): Self::SystemData) {
        for (boid, trans) in (&mut boids, &mut transforms).join() {
            //trans.rotate_2d(angle);
            //trans.move_up(velocity);
        }

        /*
        // Terrible way of computing the number of boids
        let num_boids = (&boids, &transforms)
            .join()
            .max_by_key(|(b, _)| b.id)
            .unwrap()
            .0
            .id
            + 1;

        for (boid, _trans) in (&boids, &transforms).join() {
            // Calculates the center of all boids
            let cohesion = |b: &Boid| {
                let mut center = (0.0, 0.0);
                for (other_boid, _other_trans) in (&boids, &transforms).join() {
                    if other_boid.id != b.id {
                        center = vec_add(center, (other_boid.x, other_boid.y));
                    }
                }

                center = vec_div(center, num_boids - 1);

                center
            };

            // Move boids slightly away from each other
            let separation = |b: &Boid| {
                let mut space = (0.0, 0.0);

                for (other_boid, _other_trans) in (&boids, &transforms).join() {
                    if other_boid.id != b.id {
                        let d = dist(b.x, b.y, other_boid.x, other_boid.y);
                        if d < BOID_SIGHT {
                            space =
                                vec_sub(space, vec_sub((b.x, b.y), (other_boid.y, other_boid.y)));
                        }
                    }
                }

                space
            };

            // Align boids with neighbors (velocity matching)
            let alignment = |b: &Boid| {
                let mut velocity = (0.0, 0.0);

                for (other_boid, _other_trans) in (&boids, &transforms).join() {
                    if other_boid.id != b.id {
                        velocity = vec_add(velocity, (other_boid.vx, other_boid.vy));
                    }
                }

                velocity = vec_div(velocity, num_boids - 1);

                vec_div(vec_sub(velocity, (b.vx, b.vy)), 8)
            };

            let center = cohesion(boid);
            let personal_space = separation(boid);
            let velocity = alignment(boid);
        }

        for (boid, trans) in (&mut boids, &mut transforms).join() {
            //let angle = cohesion_angle;
            //let velocity = cohesion_velocity;
            //trans.rotate_2d(angle);
            //trans.move_up(velocity);
        }
        */
    }
}

// Adds two tuples
fn vec_add(v1: (f32, f32), v2: (f32, f32)) -> (f32, f32) {
    (v1.0 + v2.0, v1.1 + v2.1)
}

// Subtract two tuples
fn vec_sub(v1: (f32, f32), v2: (f32, f32)) -> (f32, f32) {
    (v1.0 - v2.0, v1.1 - v2.1)
}

// Divides a tuple by a scalar
fn vec_div(v: (f32, f32), a: usize) -> (f32, f32) {
    (v.0 / a as f32, v.1 / a as f32)
}

// Compute euclidean distance between two points
fn dist(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}
*/
