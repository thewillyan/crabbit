use std::io;
use termion::{color, raw::IntoRawMode};
use termjumper::{runner::Runner, stage::Stage, Game};

fn main() {
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let (cols, rows) = termion::terminal_size().unwrap();

    let sky = vec![
        vec![' ', '.', ' ', ' ', ' ', ' ', '*'],
        vec![' ', ' ', '.', '+'],
    ];

    let mountains = vec![
        vec![' '],
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

    stage.add_layer(sky, color::Fg(color::White), 40, false, 1);
    stage.add_layer(mountains, color::Fg(color::LightBlack), 4, false, 1);
    stage.add_layer(ground, color::Fg(color::Green), 0, true, 2);

    if rows > stage.size.height {
        stage.add_padding(rows - stage.size.height);
    }

    let player = vec![vec!['O']];
    let game = Game::new(player, color::Fg(color::Yellow), stage);

    Runner::run(game, &mut stdout);
}
