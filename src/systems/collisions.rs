use amethyst::core::math::geometry::Point3;
use amethyst::input::{InputHandler, StringBindings};
use amethyst::{
    core::Transform,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
};
//use std::f32::consts::PI;

use crate::boids::{Boid, ARENA_HEIGHT, ARENA_WIDTH};

pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        ReadStorage<'a, Boid>,
        WriteStorage<'a, Transform>,
        Read<'a, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (boids, mut locals, _input): Self::SystemData) {
        let other_locals: Vec<_> = (&boids, &locals).join().collect();
        for (boid, local) in (&boids, &mut locals).join() {
            let boid_x = local.translation().x;
            let boid_y = local.translation().y;

            for (other_boid, other_local) in &other_locals {
                //
            }

            if boid_y - boid.height >= ARENA_HEIGHT {
                local.set_translation_y(0.0 - boid.height);
            }
            if boid_y + boid.height < 0.0 {
                local.set_translation_y(ARENA_HEIGHT + boid.height);
            }
            if boid_x - boid.width >= ARENA_WIDTH {
                local.set_translation_x(0.0 - boid.width);
            }
            if boid_x + boid.width < 0.0 {
                local.set_translation_x(ARENA_WIDTH + boid.width);
            }

            /*
            let mouse_coords = input.mouse_position();
            let boid_y = local.translation().y;
            let boid_x = local.translation().x;
            if let Some(coords) = mouse_coords {
                //println!("{}", collide_with_mouse(&boid, &local, coords.0, coords.1));

                /*
                println!(
                    "({:.2}, {:.2}) ({:.3}, {:.3})",
                    boid_x, boid_y, coords.0, coords.1
                );
                */

                if collide_with_mouse(&boid, &local, coords.0, coords.1) {
                    // Compute distance between mouse and boid
                    let distance =
                        ((coords.0 - boid_x).powi(2) + (coords.1 - boid_y).powi(2)).sqrt();

                    // Angle is inversely proportional to distance
                    let angle = 0.1 / distance;

                    local.rotate_2d(angle);
                }
            }
            */
        }
    }
}

// Determins if two boids will collide
fn collision(boid: &Transform, other: &Transform, fov_dist: f32) -> bool {
    dist(
        boid.translation().x,
        boid.translation().y,
        other.translation().x,
        other.translation().y,
    ) <= fov_dist
}

// Compute euclidean distance between two points
fn dist(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
}

/*
fn out_of_bounds(x: f32, y: f32) -> bool {
    x >= ARENA_WIDTH || x < 0.0 || y >= ARENA_HEIGHT || y < 0.0
}

fn collide_with_mouse(boid: &Boid, transform: &Transform, mouse_x: f32, mouse_y: f32) -> bool {
    let boid_y = transform.translation().y;
    let boid_x = transform.translation().x;

    // Construct edges of boid rectangle
    let left = boid_x - (boid.width / 2.0);
    let bottom = boid_y - (boid.height / 2.0);
    let right = boid_x + boid.width + (boid.width / 2.0);
    let top = boid_y + boid.height + (boid.height / 2.0);

    mouse_x >= left && mouse_x <= right && mouse_y >= bottom && mouse_y <= top
}
*/
