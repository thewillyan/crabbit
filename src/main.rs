use std::io;
use termion::{color, raw::IntoRawMode};
use termjumper::{runner::Runner, stage::Stage, Game, sprite::Sprite};

fn main() {
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let (cols, rows) = termion::terminal_size().unwrap();

    let sky = vec![
        ' ', '.', ' ', ' ', ' ', ' ', '*',
        ' ', ' ', '.', '+', ' ', ' ', ' '
    ];
    let sky = Sprite::new(sky, 7);

    let mountains = vec![
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', '/', '\\', ' ', ' ', ' ', ' ',
        ' ', ' ', '/', ' ', ' ', '\\', '/', '\\', ' ',
        ' ', '/', ' ', ' ', ' ', '/', ' ', ' ', '\\'
    ];
    let mountains = Sprite::new(mountains, 9);

    let ground = vec![
        '-', '^', '-',
        ' ', '.', ' ',
        '.', ' ', '.',
    ];
    let ground = Sprite::new(ground, 3);

    let mut stage = Stage::new(cols);

    stage.add_layer(sky, color::Fg(color::White), 40, false, 1);
    stage.add_layer(mountains, color::Fg(color::LightBlack), 4, false, 1);
    stage.add_layer(ground, color::Fg(color::Green), 0, true, 2);

    if rows > stage.size.height {
        stage.add_padding(rows - stage.size.height);
    }

    let player = Sprite::new(vec!['O'], 1);

    let game = Game::new(player, color::Fg(color::Yellow), stage);

    Runner::run(game, &mut stdout);
}
