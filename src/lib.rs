use std::{
    io::Write,
    collections::LinkedList,
    thread,
    time::Duration
};
use termion::{cursor, clear};

pub mod stage;

use stage::Stage;

// (line, column)
type Pos = (u16, u16);
// (width, height)
type Size = (usize, usize);
// ascii matrix
type Sprite = Vec<Vec<char>>;

// not used for now
pub trait AsObj {
    fn as_obj(&self) -> Obj;
}

pub trait AsSprite {
    fn as_sprite(&self) -> Sprite;
}

// not used for now
pub trait Render {
    fn render<O: Write>(&self, out: &mut O);
    fn erase<O: Write>(&self, out: &mut O);
}

pub struct Obj {
    pub pos: Pos,
    pub sprite: Sprite
}

impl Obj {
    pub fn render<O: Write>(&self, out: &mut O) {
        let (x, mut y) = self.pos;
        for line in &self.sprite {
            let line: String = line.iter().collect();
            write!(out, "{}{}", cursor::Goto(x,y), line).unwrap();
            y += 1;
        }
    }

    pub fn erase<O: Write>(&self, out: &mut O) {
        let (x, mut y) = self.pos;
        for line in &self.sprite {
            let overwrite = " ".repeat(line.len());
            write!(out, "{}{}", cursor::Goto(x,y), overwrite).unwrap();
            y += 1;
        }
    }
}

// Retangular object
#[derive(Debug)]
pub struct RetObj {
    pub size: Size,
    pub pos: Pos,
    pub sprite: Sprite,
}

impl RetObj {
    pub fn new(x: u16, y: u16, mut sprite: Sprite) -> RetObj {
        let pos = (x, y);
        Self::to_ret(&mut sprite);
        let size = (sprite[0].len(), sprite.len());

        RetObj { size, pos, sprite }
    }

    pub fn to_ret(sprite: &mut Sprite) {
        let width = sprite
            .iter()
            .map(|line| line.len())
            .max()
            .expect("Empty sprite!");
        sprite.iter_mut().for_each(|line| {
            // add padding right
            while line.len() < width {
                line.push(' ');
            }
        });
    }

    pub fn render<O: Write>(&self, out: &mut O) {
        let (x, mut y) = self.pos;
        for line in &self.sprite {
            let line: String = line.iter().collect();
            write!(out, "{}{}", cursor::Goto(x,y), line).unwrap();
            y += 1;
        }
    }

    pub fn erase<O: Write>(&self, out: &mut O) {
        let (width, height) = self.size;
        let (x, y) = self.pos;
        let overwrite = " ".repeat(width);
        for i in y..(y + height as u16) {
            write!(out, "{}{}", cursor::Goto(x,i), overwrite).unwrap();
        }
    }
}

pub struct Game {
    player: Player,
    _obstacles: Obs,
    stage: Stage
}

impl Game {
    pub fn new(player_sprite: Sprite, stage: Stage) -> Game {
        let floor = stage.floor().expect("Empty stage!");
        let player_height = player_sprite.len() as u16;
        let player = Player::new(player_sprite, 5, floor - player_height);
        Game { player, _obstacles: Obs, stage }
    }
}

// Player
pub struct Player {
    pub score: u32,
    pub state: PlayerState,
    obj: Obj
}

impl Player {
    pub fn new(sprite: Sprite, x: u16, y: u16) -> Player {
        let score = 0;
        let state = PlayerState::Running;
        let obj = Obj {
            pos: (x, y),
            sprite
        };
        Player { score, state, obj }
    }

    pub fn up(&mut self) {
        let (_, y) = &mut self.obj.pos;
        if *y > 1 { *y -= 1 }
    }

    pub fn down(&mut self) {
        self.obj.pos.1 += 1;
    }
}

pub enum PlayerState {
    Jumping,
    Running,
    Killed
}

// Obstacles
pub struct Obs;


// Runner
pub enum Move {
    Up(u16),
    Down(u16)
}

pub struct Runner {
    pub game: Game,
    pub move_queue: LinkedList<Move> 
}

impl Runner {
    pub fn new(game: Game) -> Runner {
        Runner { game, move_queue: LinkedList::new() }
    }

    pub fn run<O: Write>(&mut self, out: &mut O) {
        let stage = &mut self.game.stage;
        stage.fill_hitmap();
        let player = &self.game.player;

        write!(out, "{}", clear::All).unwrap();
        for i in 1..=30 {
            stage.render(out);
            player.obj.render(out);
            write!(out, "{}frame: {}", cursor::Goto(1,1), i).unwrap();
            write!(out, "{}player possition: {:?}", cursor::Goto(1,2), player.obj.pos).unwrap();
            out.flush().unwrap();
            thread::sleep(Duration::from_millis(100));
            stage.shift();
        }

        write!(out, "{}{}Hitmap: {:?}\r\n", clear::All, cursor::Goto(1,1), stage.hitmap)
            .unwrap();
    }
}
