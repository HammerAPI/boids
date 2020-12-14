/*
//use amethyst::core::math::Point3;
use amethyst::input::{InputHandler, StringBindings};
use amethyst::{
    core::Transform,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
};
//use std::f32::consts::PI;

use crate::boids::{Boid, TransformInfo, ARENA_HEIGHT, ARENA_WIDTH};

pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        ReadStorage<'a, Boid>,
        ReadStorage<'a, Transform>,
        WriteStorage<'a, TransformInfo>,
        Read<'a, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (boids, transforms, mut transform_info, _input): Self::SystemData) {
        for (boid, trans, mut info) in (&boids, &transforms, &mut transform_info).join() {
            // Get boid coordinates
            let boid_x = trans.translation().x;
            let boid_y = trans.translation().y;
            // Get the boid's angle
            //let angle = trans.euler_angles().2;

            // Get the point the boid is looking at
            // TODO: Add check so that any entity that crosses this line will cause movement
            // let ahead = look_ahead(boid_x, boid_y, angle, boid.visual_distance);

            // Distance between boid and lookahead point
            //let dist = dist(boid_x, boid_y, ahead.x, ahead.y);
            //println!("{:?}", dist);

            // If the boid is about to go out of bounds, turn around
            /*
            if out_of_bounds(ahead.x, ahead.y) {
                //info.angles.push(0.2);
            }
            */

            /*
            println!(
                "({:2.2}, {:2.2}) -> {:?})",
                boid_x,
                boid_y,
                boid.ray.at_distance(boid.visual_distance)
            );

            info.angles.push(0.015);
            */

            for (_other_boid, other_trans) in (&boids, &transforms).join() {
                if other_trans != trans {
                    if collision(&trans, &other_trans, boid.visual_distance) {
                        let dist1 = dist(
                            boid_x,
                            boid_y,
                            other_trans.translation().x,
                            other_trans.translation().y,
                        );

                        let dist2 = dist(
                            boid_x + 1.0,
                            boid_y + 1.0,
                            other_trans.translation().x,
                            other_trans.translation().y,
                        );
                        if dist1 > dist2 {
                            info.angles.push(1.0 / dist1);
                        } else {
                            info.angles.push(-1.0 / dist1);
                        }
                    }
                    /*
                    if dist <= boid.visual_distance {
                        info.angles.push(1.0 / dist);
                    }
                    */

                    /*
                    if out_of_bounds(x, y) {
                        info.angles.push(0.02);
                    }
                    */
                }
            }

            /*
            let angle = trans.euler_angles().2;
            let angle = if angle < 0.0 {
                -2.0 * angle
            } else {
                2.0 * angle
            };
            */

            //println!("{:0.3} {:0.2} {:0.2}", angle, boid_x, boid_y);
            //let (x, y) = look_ahead(boid_x, boid_y, angle, boid.visual_distance);

            /*
            println!(
                "{:1.3} ({:2.2}, {:2.2}) -> ({:2.2}, {:2.2})",
                angle, boid_x, boid_y, x, y
            );
            */

            //info.angles.push(-0.015);

            if boid_y - boid.height >= ARENA_HEIGHT {
                //trans.set_translation_y(0.0 - boid.height);
                info.new_y = 0.0 - boid.height;
            }
            if boid_y + boid.height < 0.0 {
                //trans.set_translation_y(ARENA_HEIGHT + boid.height);
                info.new_y = ARENA_HEIGHT + boid.height;
            }
            if boid_x - boid.width >= ARENA_WIDTH {
                //trans.set_translation_x(0.0 - boid.width);
                info.new_x = 0.0 - boid.width;
            }
            if boid_x + boid.width < 0.0 {
                //trans.set_translation_x(ARENA_WIDTH + boid.width);
                info.new_x = ARENA_WIDTH + boid.width;
            }

            /*
            let mouse_coords = input.mouse_position();
            let boid_y = trans.translation().y;
            let boid_x = trans.translation().x;
            if let Some(coords) = mouse_coords {
                //println!("{}", collide_with_mouse(&boid, &trans, coords.0, coords.1));

                /*
                println!(
                    "({:.2}, {:.2}) ({:.3}, {:.3})",
                    boid_x, boid_y, coords.0, coords.1
                );
                */

                if collide_with_mouse(&boid, &trans, coords.0, coords.1) {
                    // Compute distance between mouse and boid
                    let distance =
                        ((coords.0 - boid_x).powi(2) + (coords.1 - boid_y).powi(2)).sqrt();

                    // Angle is inversely proportional to distance
                    let angle = 0.1 / distance;

                    trans.rotate_2d(angle);
                }
            }
            */
        }
    }
}

/*
// Computes the coordinates of the point ahead of the boid
fn look_ahead(x: f32, y: f32, angle: f32, dist: f32) -> Point3<f32> {
    let new_x = x + angle.cos() * dist;
    let new_y = y + -(angle.sin() * dist);

    Point3::new(new_x, new_y, 0.0)
}
*/

// Determines if two boids will collide
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
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

/*
fn out_of_bounds(x: f32, y: f32) -> bool {
    x >= ARENA_WIDTH || x < 0.0 || y >= ARENA_HEIGHT || y < 0.0
}
*/
/*

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
*/
