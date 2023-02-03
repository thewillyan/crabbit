use std::io;
use termion::{color, raw::IntoRawMode};

use crabbit::{
    components::{
        enemies::{Enemies, Walls},
        hud::{Hud, Splash},
        player::Player,
        stage::{Layer, Stage},
    },
    game::Game,
    graphics::{object::Sprite, Pos},
};

fn main() {
    // get terminal output
    let mut stdout = io::stdout()
        .into_raw_mode()
        .expect("Failed to get a new raw stdout handler");
    let (cols, rows) = termion::terminal_size().expect("Failed to get terminal size.");

    // load stage layers
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

    // creates stage
    let mut stage = Stage::new(cols, rows);
    stage.push_layer(ground, color::LightWhite);
    stage.push_layer(grass, color::Green);
    stage.set_floor();
    stage.push_layer(mountains, color::LightBlack);
    stage.push_layer(sky, color::White);

    // creates player
    let player = Player::new('O', color::Yellow, stage.floor);

    // set enemies
    let mut enemies = Enemies::new();
    let walls_spawn = Pos {
        col: cols,
        row: stage.floor,
    };
    enemies.add_enemy(Walls::new('|', walls_spawn, 2));

    // setup HUD
    let splash_screen = Splash::new(
        &stage.size,
        "Welcome to Crabbit! Press any key to continue.",
        color::Blue,
        "Game Paused",
        color::Magenta,
    );
    let hud = Hud::new(splash_screen);

    Game::new(player, stage, enemies, hud).run(&mut stdout);
}
