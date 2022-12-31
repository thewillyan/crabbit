use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use termion::{
    raw::IntoRawMode,
    clear,
    cursor
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
        vec![' '; 15],
        vec!['+', '.', ' ', '/', '\\'],
        vec![' ', ' ', '/', ' ', ' ', '\\', '/', '\\'],
        vec![' ', '/', ' ', ' ', ' ', '/', ' ', ' ', '\\'],

    ];

    let (cols, rows) = termion::terminal_size().unwrap();

    let mut stage = Stage::new(cols as usize);
    stage.add_layer(sky, false, 1);
    stage.add_layer(ground, true, 2);
    stage.add_padding(rows - stage.size.1 as u16);
    stage.fill_hitmap();

    for i in 1..=30 {
        stage.render(&mut stdout);
        write!(stdout, "{}frame: {}", cursor::Goto(1,1), i).unwrap();
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(100));
        stage.shift();
    }

    write!(stdout, "{}{}Hitmap: {:?}\r\n", clear::All, cursor::Goto(1,1), stage.hitmap)
        .unwrap();
}
