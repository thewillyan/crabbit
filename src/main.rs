use std::io;
use termion::{color, raw::IntoRawMode};

use crabbit::{
    components::{
        player::Player,
        stage::{Layer, Stage},
    },
    graphics::Sprite,
    Game, Runner,
};

fn main() {
    let mut stdout = io::stdout()
        .into_raw_mode()
        .expect("Failed to get a new raw stdout handler");
    let (cols, rows) = termion::terminal_size().expect("Failed to get terminal size.");

    let ground = Layer::new(cols, Sprite::from_file("./sprites/ground"))
        .gap(4)
        .shift(2)
        .build();
    let grass = Layer::new(cols, Sprite::from_file("./sprites/grass"))
        .shift(2)
        .build();
    let mountains = Layer::new(cols, Sprite::from_file("./sprites/mountains"))
        .gap(4)
        .build();
    let sky = Layer::new(cols, Sprite::from_file("./sprites/sky"))
        .gap(40)
        .build();

    let mut stage = Stage::new(cols, rows);

    stage.push_layer(ground, color::LightWhite);
    stage.push_layer(grass, color::Green);
    stage.set_floor();
    stage.push_layer(mountains, color::LightBlack);
    stage.push_layer(sky, color::White);

    let player = Player::new('O', color::Yellow, stage.floor);
    let game = Game::new(player, stage);

    Runner::new(
        game,
        "Welcome to Crabbit! Press any key to continue.",
        color::Blue,
    )
    .run(&mut stdout);
}
