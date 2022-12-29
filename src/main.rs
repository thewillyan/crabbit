use std::{
    io::{self, Write},
    thread,
    time::Duration
};

use termion::{
    raw::IntoRawMode,
    clear
};
use termjumper::stage::Stage;

fn main() {
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", clear::All).unwrap();
    stdout.flush().unwrap();

    let ground = vec![
        vec!['-', '^', '-'],
        vec![' ', '.', ' '],
        vec!['.', ' ', '.'],
    ];

    let sky = vec![
        vec![' ', '.', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        vec!['.', ' ', '/', '\\', ' ', ' ', ' ', ' ', '+'],
        vec![' ', '/', ' ', ' ', '\\', '/', '\\', ' ', ' '],
        vec!['/', ' ', ' ', ' ', '/', ' ', ' ', '\\', ' '],

    ];

    let mut stage = Stage::new(100);
    stage.add_layer(sky, false, 0);
    stage.add_layer(ground, true, 1);
    for i in 1..=30 {
        stage.render(&mut stdout);
        write!(stdout, "frame: {}", i).unwrap();
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(30));
        stage.shift();
    }
}
