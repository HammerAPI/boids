use amethyst::core::transform::TransformBundle;
use amethyst::prelude::*;
use amethyst::renderer::plugins::{RenderFlat2D, RenderToWindow};
use amethyst::renderer::types::DefaultBackend;
use amethyst::renderer::RenderingBundle;
use amethyst::utils::application_root_dir;

mod boids;
mod systems;
use crate::boids::Boids;

fn main() -> amethyst::Result<()> {
    // Start the logger
    amethyst::start_logger(Default::default());

    // Setup directories
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let assets_dir = app_root.join("assets");

    // Construct the game data
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?.with_clear([
                        245.0 / 255.0, // Red
                        245.0 / 255.0, // Green
                        220.0 / 255.0, // Blue
                        1.0,           // Alpha
                    ]), // Construct a beige window
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with(systems::CollisionSystem, "collision_system", &[])
        .with(
            systems::MovementSystem,
            "movement_system",
            &["collision_system"],
        ); // The movement system depends on the collision system

    // Setup the game
    let mut game = Application::new(assets_dir, Boids, game_data)?;

    // Self explanatory
    game.run();

    Ok(())
}
