use amethyst::core::math::Matrix3x1;
use amethyst::input::{InputHandler, StringBindings};
use amethyst::{
    core::Transform,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
};
use std::f32::consts::PI;

use crate::boids::{Boid, TransformInfo, ARENA_HEIGHT, ARENA_WIDTH, BOID_SIGHT};

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

            // For each boid, check every other boid on the map
            for (other_boid, other_trans) in (&boids, &transforms).join() {
                // Don't check yourself!
                if boid.id != other_boid.id {
                    // Angle to turn
                    let mut turn_angle = 0.0;

                    // Get boid's angle in radians
                    let boid_angle = rad_rotation(trans);

                    // Get the angle between the two boids
                    let angle_between = fix_angle(get_angle_between(trans, other_trans));

                    // Get the distance between the two boids
                    let distance = boid_dist(trans, other_trans);

                    // RULE 1: SEPARATION
                    // Keep a distance between every other boid
                    if in_fov(trans, other_trans, BOID_SIGHT, boid_angle, angle_between) {
                        let turn = if is_left(
                            trans.translation(),
                            boid_angle,
                            other_trans.translation(),
                        ) {
                            2.0 / distance
                        } else {
                            -2.0 / distance
                        };

                        // Calculate the amount needed to turn
                        // Amount needed should be inversely proportional to the distance between
                        // the boids
                        turn_angle += turn;
                    }

                    if nearby(trans, other_trans, BOID_SIGHT * 5.0) {
                        // RULE 2: ALIGNMENT
                        // Try and match angle/velocity of nearby boids
                        let other_boid_angle = rad_rotation(other_trans);
                        let diff = other_boid_angle - boid_angle;
                        let turn = diff / 500.0;

                        turn_angle += turn;

                        // RULE 3: COHESION
                        // Try and steer towards the center of mass of neighboring boids
                        // Or, in this case, steer slightly towards any nearby boids
                        let turn = if is_left(
                            trans.translation(),
                            boid_angle,
                            other_trans.translation(),
                        ) {
                            -0.25 / distance
                        } else {
                            0.25 / distance
                        };

                        turn_angle += turn;
                    }
                    info.angles.push(turn_angle);
                }
            }

            // Teleport boids if they leave the arena
            if boid_y - boid.height >= ARENA_HEIGHT {
                info.new_y = 0.0 - boid.height;
            }
            if boid_y + boid.height < 0.0 {
                info.new_y = ARENA_HEIGHT + boid.height;
            }
            if boid_x - boid.width >= ARENA_WIDTH {
                info.new_x = 0.0 - boid.width;
            }
            if boid_x + boid.width < 0.0 {
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

// Credit to James Pruitt, @Rapdorian (GitHub) for this function
// This function imagines a vector perpendicular to the initial vector facing "right" we then
// project the vector from p1 to p2 onto that imagined vector and compare that value
fn is_left(p1: &Matrix3x1<f32>, theta: f32, p2: &Matrix3x1<f32>) -> bool {
    let axis = theta - (PI / 2.0);
    let axis = Matrix3x1::new(axis.cos(), axis.sin(), 0.0);
    let v2 = p2 - p1;
    // project v2 onto our axis
    // the formula is `v2 dot axis / |axis|` but |axis| is always 1
    let proj = v2.dot(&axis);
    proj < 0.0
}

// Check if `other` is in the 180-degree FOV of `boid`
fn in_fov(boid: &Transform, other: &Transform, distance: f32, ba: f32, ab: f32) -> bool {
    if nearby(boid, other, distance) {
        // Adjust so that the boid is facing straight up, or PI / 2.0
        // So, ba = ba - ba + (PI/2.0)
        // And we need the same adjustment to the angle between both boids
        let ab = ab - ba + (PI / 2.0);

        // If the angle between is larger than a full rotation, subtract a full rotation
        let ab = if ab > 2.0 * PI { ab - 2.0 * PI } else { ab };
        // Same thing, but negative
        let ab = if ab < -2.0 * PI { ab + 2.0 * PI } else { ab };

        // `boid` is assumed to be facing up, so if the angle between the two boids is between 0
        // and pi, then `other` is in `boid`'s 180-degree FOV
        if ab > 0.0 && ab < PI {
            return true;
        }
    }
    false
}

// Gets the angle between two boids
fn get_angle_between(boid: &Transform, other: &Transform) -> f32 {
    angle_between(
        boid.translation().x,
        boid.translation().y,
        other.translation().x,
        other.translation().y,
    )
}

// Get the angle between two points
fn angle_between(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    (y2 - y1).atan2(x2 - x1)
}

// Gets the rotation [0, 2pi] of the transform
fn rad_rotation(trans: &Transform) -> f32 {
    // Get the `z` component, in radians
    let angle = trans.euler_angles().2;

    fix_angle(angle)
}

// Fix the angle
fn fix_angle(angle: f32) -> f32 {
    // If the angle is negative, meaning the entity is facing downwards, get its corresponding positive angle
    if angle < 0.0 {
        2.0 * PI + angle
    } else {
        angle
    }
}

/*
// Computes the coordinates of the point ahead of the boid
fn look_ahead(x: f32, y: f32, angle: f32, dist: f32) -> Point3<f32> {
    let new_x = x + angle.cos() * dist;
    let new_y = y + -(angle.sin() * dist);

    Point3::new(new_x, new_y, 0.0)
}

// Determines if a coordinate is out of bounds of the arena
fn out_of_bounds(x: f32, y: f32) -> bool {
    x >= ARENA_WIDTH || x < 0.0 || y >= ARENA_HEIGHT || y < 0.0
}
*/

// Determines if two boids are nearby, based on the distance provided
fn nearby(boid: &Transform, other: &Transform, distance: f32) -> bool {
    dist(
        boid.translation().x,
        boid.translation().y,
        other.translation().x,
        other.translation().y,
    ) <= distance
}

// Get the distance between two boids
fn boid_dist(boid: &Transform, other: &Transform) -> f32 {
    dist(
        boid.translation().x,
        boid.translation().y,
        other.translation().x,
        other.translation().y,
    )
}

// Compute euclidean distance between two points
fn dist(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

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
