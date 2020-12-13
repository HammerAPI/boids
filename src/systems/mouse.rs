//use amethyst::core::{SystemDesc, Transform};
//use amethyst::derive::SystemDesc;
//use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::ecs::{Read, ReadStorage, System};
use amethyst::input::{InputHandler, StringBindings};

use crate::boids::Boid;

//#[derive(SystemDesc)]
pub struct MouseInputSystem;

impl<'a> System<'a> for MouseInputSystem {
    type SystemData = (
        ReadStorage<'a, Boid>,
        Read<'a, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (boids, input): Self::SystemData) {
        let coords = input.mouse_position();
        println!("{:?}", coords);
    }
}
