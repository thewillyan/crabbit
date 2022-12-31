use std::io;

use termion::{raw::IntoRawMode, color};
use termjumper::{stage::Stage, Game, runner::Runner};

fn main() {
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let (cols, rows) = termion::terminal_size().unwrap();

    let sky = vec![
        vec![' '; 40],
        vec![' ', '.', ' ', ' ', ' ', ' ', '*'],
        vec![' ', ' ', '.', '+']
    ];

    let mountains = vec![
        vec![' '; 13],
        vec![' ', ' ', ' ', '/', '\\'],
        vec![' ', ' ', '/', ' ', ' ', '\\', '/', '\\'],
        vec![' ', '/', ' ', ' ', ' ', '/', ' ', ' ', '\\'],

    ];

    let ground = vec![
        vec!['-', '^', '-'],
        vec![' ', '.', ' '],
        vec!['.', ' ', '.'],
    ];


    let mut stage = Stage::new(cols);
    stage.add_layer(sky, color::White.fg_str(), false, 1);
    stage.add_layer(mountains, color::LightBlack.fg_str(),  false, 1);
    stage.add_layer(ground, color::Green.fg_str(), true, 2);
    if rows > stage.size.height {
        stage.add_padding(rows - stage.size.height);
    }

    let player = vec![vec!['O']];
    let game = Game::new(player, color::Yellow.fg_str(), stage);

    let mut runner = Runner::new(game);
    runner.run(&mut stdout);
}
