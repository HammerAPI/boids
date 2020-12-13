use amethyst::{
    core::transform::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::boids::Boid;

pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (WriteStorage<'a, Boid>, WriteStorage<'a, Transform>);

    fn run(&mut self, (mut boids, mut locals): Self::SystemData) {
        // Move every boid at a constant speed
        for (boid, local) in (&mut boids, &mut locals).join() {
            local.move_up(boid.velocity);
            boid.ray.origin = boid.ray.at_distance(boid.velocity);
        }
    }
}
