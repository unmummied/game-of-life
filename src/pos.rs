use crate::st::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Pos {
    pub x: u8,
    pub y: u8,
}

impl Pos {
    pub fn from(t: (u8, u8)) -> Self {
        Pos { x: t.0, y: t.1 }
    }

    pub fn torus(&self) -> Self {
        Pos {
            x: ((self.x as i8 - 1).rem_euclid(WIDTH) + 1) as _,
            y: ((self.y as i8 - 1).rem_euclid(HEIGHT) + 1) as _,
        }
    }

    pub fn neighbors(&self) -> Vec<Pos> {
        [
            (self.x - 1, self.y - 1),
            (self.x, self.y - 1),
            (self.x + 1, self.y - 1),
            (self.x - 1, self.y),
            (self.x + 1, self.y),
            (self.x - 1, self.y + 1),
            (self.x, self.y + 1),
            (self.x + 1, self.y + 1),
        ]
        .into_iter()
        .map(|p| Pos::from(p).torus())
        .collect()
    }
}
