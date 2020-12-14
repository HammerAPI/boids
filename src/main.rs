use amethyst::core::transform::TransformBundle;
use amethyst::input::{InputBundle, StringBindings};
use amethyst::prelude::*;
use amethyst::renderer::plugins::{RenderFlat2D, RenderToWindow};
use amethyst::renderer::types::DefaultBackend;
use amethyst::renderer::RenderingBundle;
use amethyst::utils::application_root_dir;

mod boids;
mod systems;
use crate::boids::Boids;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let assets_dir = app_root.join("assets");
    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.00196, 0.23726, 0.21765, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        //.with_bundle(input_bundle)?
        .with(systems::MovementSystem, "movement_system", &[]);
    //.with(systems::CollisionSystem, "collisionSystem", &[]);

    let mut game = Application::new(assets_dir, Boids, game_data)?;

    game.run();

    Ok(())
}
