extern crate amethyst;

use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::{application_dir, application_root_dir},
    Error,
};

fn main() -> Result<(), amethyst::Error> {
    amethyst::start_logger(Default::default());

    struct Pong;
    impl SimpleState for Pong {}

    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");
    let display_config_path = app_root.join("config").join("display.ron");

    let game_data = GameDataBuilder::default().with_bundle(
        RenderingBundle::<DefaultBackend>::new()
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?
                    .with_clear([0.0, 0.0, 0.0, 1.0]),
            )
            .with_plugin(RenderFlat2D::default()),
    )?;

    let mut game = Application::new(assets_dir, Pong, game_data)?;

    game.run();
    Ok(())
}
