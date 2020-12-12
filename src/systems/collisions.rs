use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};
use std::f32::consts::PI;

use crate::boids::{Boid, ARENA_HEIGHT, ARENA_WIDTH};

pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (ReadStorage<'a, Boid>, WriteStorage<'a, Transform>);

    fn run(&mut self, (boids, mut locals): Self::SystemData) {
        for (boid, local) in (&boids, &mut locals).join() {
            /*
            if local.translation().y >= ARENA_HEIGHT {
                local.set_translation_y(0.0);
            }
            if local.translation().y < 0.0 {
                local.set_translation_y(ARENA_HEIGHT);
            }
            if local.translation().x >= ARENA_WIDTH {
                local.set_translation_x(0.0);
            }
            if local.translation().x < 0.0 {
                local.set_translation_x(ARENA_WIDTH);
            }
            */

            //println!("{:?}", local.rotation().angle());

            // Get boid coordinates
            let boid_y = local.translation().y;
            let boid_x = local.translation().x;
            let boid_angle = local.rotation().angle();

            // Compute distance between boid and nearest wall
            let y_dist = boid_y.min(ARENA_HEIGHT - boid_y);
            let x_dist = boid_x.min(ARENA_WIDTH - boid_x);
            let min_dist = y_dist.min(x_dist);
            let mut angle = 0.25 / min_dist;

            // if angle in radians is between 0 and pi/4, pi/4 and pi/2, that handles left and
            // right
            // just need to figure out up and down

            // If the boid is about to hit a wall, turn
            if boid_y + boid.visual_distance >= ARENA_HEIGHT || boid_y - boid.visual_distance < 0.0
            {
                // The angle at which the boid should turn should be inversely
                // proportional to the distance it is to its collision

                local.rotate_2d(angle);
            }

            // If the boid is about to hit a wall, turn
            if boid_x + boid.visual_distance >= ARENA_WIDTH || boid_x - boid.visual_distance < 0.0 {
                local.rotate_2d(angle);
            }
        }
    }
}
