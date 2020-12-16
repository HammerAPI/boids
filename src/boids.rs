use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::{Component, DenseVecStorage, World};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture,
};
use rand::{thread_rng, Rng};
use std::f32::consts::PI;
use std::f32::NAN;

// Tie the "game data" to a struct
pub struct Boids;

// Default number of boids
const DEFAULT_POPULATION: usize = 25;

// Constants for the arena size
pub const ARENA_HEIGHT: f32 = 200.0;
pub const ARENA_WIDTH: f32 = 200.0;

// Constants for the boid's features
pub const BOID_VELOCITY: f32 = 0.75;
pub const BOID_SIGHT: f32 = 15.0;
const BOID_WIDTH: f32 = 7.0;
const BOID_HEIGHT: f32 = 10.0;

// Holds transformation information to be applied to a boid
pub struct TransformInfo {
    pub angles: Vec<f32>,
    pub velocities: Vec<f32>,
    pub new_y: f32,
    pub new_x: f32,
}

impl Component for TransformInfo {
    type Storage = DenseVecStorage<Self>;
}

pub struct Boid {
    pub id: usize,
    pub width: f32,
    pub height: f32,
}

impl Component for Boid {
    type Storage = DenseVecStorage<Self>;
}

fn initialize_boids(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, num_boids: usize) {
    let mut rng = thread_rng();

    for i in 0..num_boids {
        // Create the translation
        let mut transform = Transform::default();

        // Generate random spawn coordinates
        let spawn_x = rng.gen_range(BOID_WIDTH as f32, ARENA_WIDTH - BOID_WIDTH as f32);
        let spawn_y = rng.gen_range(BOID_HEIGHT as f32, ARENA_HEIGHT - BOID_HEIGHT as f32);

        // Spawn with a random orientation
        transform.set_rotation_2d(rng.gen_range(-PI, PI));

        // Set the coordinates
        transform.set_translation_xyz(spawn_x, spawn_y, 0.0);

        // Pick a random color for the boid
        let color: usize = rng.gen_range(0, 2);
        let sprite_render = SpriteRender::new(sprite_sheet_handle.clone(), color);

        // Create the boid
        world
            .create_entity()
            .with(sprite_render)
            .with(Boid {
                id: i,
                width: BOID_WIDTH,
                height: BOID_HEIGHT,
            })
            .with(TransformInfo {
                angles: vec![],
                velocities: vec![],
                new_y: NAN,
                new_x: NAN,
            })
            .with(transform)
            .build();
    }
}

impl SimpleState for Boids {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // Obtain the world to generate boids in
        let world = data.world;

        // Load the spritesheet for the graphics
        let sprite_sheet_handle = load_sprite_sheet(world);

        // If the user supplies a number of boids to spawn via command-line args, use it
        // Otherwise, spawn in a default number of boids
        let args: Vec<String> = std::env::args().collect();
        let num_boids = if args.len() > 1 {
            args[1].parse::<usize>().unwrap_or(DEFAULT_POPULATION)
        } else {
            DEFAULT_POPULATION
        };

        // Initialize all boids
        initialize_boids(world, sprite_sheet_handle, num_boids);

        // Set up world camera
        initialize_camera(world);
    }
}

fn initialize_camera(world: &mut World) {
    // Set up camera in a way that our screen covers the whole arena and (0, 0) is in the bottom
    // left
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load the spite sheet necessary to render the graphics
    // The texture is plain pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/arrow_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/arrow_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
