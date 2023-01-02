pub type Sprite = Vec<Vec<char>>;

// turn the sprite retangular (every line has the same size)
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

// stretch sprite "size" times with a given char
pub fn stretch(sprite: &mut Sprite, size: usize, c: char) {
    sprite.iter_mut().for_each(|line| {
        for _ in 0..size {
            line.push(c);
        }
    });
}
