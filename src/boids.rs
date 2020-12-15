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

pub struct Boids;

pub const ARENA_HEIGHT: f32 = 200.0;
pub const ARENA_WIDTH: f32 = 200.0;

pub const BOID_FOV: f32 = 0.25 * PI;
pub const BOID_VELOCITY: f32 = 0.75;
pub const BOID_SIGHT: f32 = 20.0;
pub const BOID_WIDTH: f32 = 7.0;
pub const BOID_HEIGHT: f32 = 10.0;

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
    pub pos: (f32, f32),
    pub velocity: f32,
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

        /*
        let spawn_x = ARENA_WIDTH / 2.0;
        let spawn_y = ARENA_HEIGHT / 2.0;
        */
        /*
        let spawn_x;
        let spawn_y;
        if i % 4 == 0 {
            // Top right
            spawn_x = (ARENA_WIDTH * 0.9) - 40.0;
            spawn_y = (ARENA_HEIGHT * 0.9) - 5.0;
            transform.set_rotation_2d(PI);
        } else if i % 4 == 1 {
            // Bottom right
            spawn_x = ARENA_WIDTH * 0.9;
            spawn_y = (ARENA_HEIGHT * 0.1) + 5.0;
            transform.set_rotation_2d(PI);
        } else if i % 4 == 2 {
            // Top left
            spawn_x = (ARENA_WIDTH * 0.1) + 40.0;
            spawn_y = (ARENA_HEIGHT * 0.9) + 5.0;
            transform.set_rotation_2d(0.0);
        } else {
            // Bottom left
            spawn_x = ARENA_WIDTH * 0.1;
            spawn_y = (ARENA_HEIGHT * 0.1) - 5.0;
            transform.set_rotation_2d(0.0);
        }
        */
        let spawn_x = rng.gen_range(BOID_WIDTH as f32, ARENA_WIDTH - BOID_WIDTH as f32);
        let spawn_y = rng.gen_range(BOID_HEIGHT as f32, ARENA_HEIGHT - BOID_HEIGHT as f32);
        transform.set_rotation_2d(rng.gen_range(-PI, PI));
        transform.set_translation_xyz(spawn_x, spawn_y, 0.0);

        // Gold boid
        //let color: usize = rng.gen_range(0, 2);
        let color = 1;
        let sprite_render = SpriteRender::new(sprite_sheet_handle.clone(), color);

        world
            .create_entity()
            .with(sprite_render)
            .with(Boid {
                id: i,
                pos: (spawn_x, spawn_y),
                velocity: BOID_VELOCITY,
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
        let world = data.world;

        // Load the spritesheet for the graphics
        let sprite_sheet_handle = load_sprite_sheet(world);

        world.register::<Boid>();

        let args: Vec<String> = std::env::args().collect();
        let num_boids = if args.len() > 1 {
            args[1].parse::<usize>().unwrap_or(1)
        } else {
            1
        };
        initialize_boids(world, sprite_sheet_handle, num_boids);
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
