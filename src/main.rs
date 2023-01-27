use crabbit::{components::Stage, graphics::Sprite, runner::Runner, Game};
use std::io;
use termion::{color, raw::IntoRawMode};

fn main() {
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let (cols, rows) = termion::terminal_size().unwrap();

    let sky = Sprite::from_file("./sprites/sky");
    let mountains = Sprite::from_file("./sprites/mountains");
    let grass = Sprite::from_file("./sprites/grass");
    let ground = Sprite::from_file("./sprites/ground");

    let mut stage = Stage::new(cols);

    stage.add_layer(sky, color::White, 40, false, 1);
    stage.add_layer(mountains, color::LightBlack, 4, false, 1);
    stage.add_layer(grass, color::Green, 0, true, 2);
    stage.add_layer(ground, color::LightWhite, 4, false, 2);

    if rows > stage.size.height {
        stage.add_padding(rows - stage.size.height);
    }

    let player = Sprite::new(vec!['O'], 1);
    let game = Game::new(player, color::Yellow, stage);

    Runner::new(
        game,
        "Welcome to Crabbit! Press any key to continue.",
        color::Blue,
    )
    .run(&mut stdout);
}
