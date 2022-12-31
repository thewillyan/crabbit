use std::io;

use termion::raw::IntoRawMode;
use termjumper::{stage::Stage, Game, Runner};

fn main() {
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let (cols, rows) = termion::terminal_size().unwrap();

    let ground = vec![
        vec!['-', '^', '-'],
        vec![' ', '.', ' '],
        vec!['.', ' ', '.'],
    ];

    let sky = vec![
        vec![' '; 15],
        vec!['+', '.', ' ', '/', '\\'],
        vec![' ', ' ', '/', ' ', ' ', '\\', '/', '\\'],
        vec![' ', '/', ' ', ' ', ' ', '/', ' ', ' ', '\\'],

    ];

    let mut stage = Stage::new(cols as usize);
    stage.add_layer(sky, false, 1);
    stage.add_layer(ground, true, 2);
    stage.add_padding(rows - stage.size.1 as u16);

    let player = vec![vec!['O']];
    let game = Game::new(player, stage);

    let mut runner = Runner::new(game);
    runner.run(&mut stdout);
}
