use std::io;
use termion::{color, raw::IntoRawMode};
use crabbit::{components::Stage, graphics::Sprite, runner::Runner, Game};

fn main() {
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let (cols, rows) = termion::terminal_size().unwrap();

    let sky = Sprite::from_file("./sprites/sky");
    let mountains = Sprite::from_file("./sprites/mountains");
    let grass = Sprite::from_file("./sprites/grass");
    let ground = Sprite::from_file("./sprites/ground");

    let mut stage = Stage::new(cols);

    stage.add_layer(sky, color::Fg(color::White), 40, false, 1);
    stage.add_layer(mountains, color::Fg(color::LightBlack), 4, false, 1);
    stage.add_layer(grass, color::Fg(color::Green), 0, true, 2);
    stage.add_layer(ground, color::Fg(color::LightWhite), 4, false, 2);

    if rows > stage.size.height {
        stage.add_padding(rows - stage.size.height);
    }

    let player = Sprite::new(vec!['O'], 1);

    let game = Game::new(player, color::Fg(color::Yellow), stage);

    Runner::run(game, &mut stdout);
}
