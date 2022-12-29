use std::{
    io::{self, Write},
    thread,
    time::Duration
};


use termion::{
    raw::IntoRawMode,
    clear
};
use termjumper::{stage, AsObj};

fn main() {
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", clear::All).unwrap();
    stdout.flush().unwrap();

    let sprite = vec![
        vec!['-', '^', '-'],
        vec![' ', '.', ' '],
        vec!['.', ' ', '.'],
    ];

    let mut ground = stage::Layer::new(100, sprite, true, 1);
    for i in 1..=30 {
        ground.as_obj().render(&mut stdout);
        write!(stdout, "frame: {}", i).unwrap();
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(30));
        ground.shift();
    }
}
