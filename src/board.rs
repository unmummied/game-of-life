use crate::pos::*;
use crate::st::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub cells: Vec<Pos>,
}

impl Board {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Board { cells: vec![] }
    }

    pub fn from(cells: Vec<(u8, u8)>) -> Self {
        Board {
            cells: cells.into_iter().map(Pos::from).collect(),
        }
    }

    pub fn is_alive(&self, pos: &Pos) -> bool {
        self.cells.contains(pos)
    }

    pub fn live_neighbors(&self, pos: &Pos) -> usize {
        pos.neighbors().iter().filter(|p| self.is_alive(p)).count()
    }

    pub fn survivors(&self) -> Vec<Pos> {
        self.cells
            .clone()
            .into_iter()
            .filter(|p| self.live_neighbors(p) == 2 || self.live_neighbors(p) == 3)
            .collect()
    }

    pub fn births(&self) -> Vec<Pos> {
        let mut babies = self
            .cells
            .clone()
            .into_iter()
            .map(|p| p.neighbors())
            .collect::<Vec<Vec<Pos>>>()
            .concat();

        babies.sort();
        babies.dedup();

        babies
            .into_iter()
            .filter(|p| !self.is_alive(p) && self.live_neighbors(p) == 3)
            .collect()
    }

    pub fn next_gen(&self) -> Vec<Pos> {
        let mut s = self.survivors();
        let mut b = self.births();
        s.append(&mut b);
        s
    }

    pub fn life(&mut self) {
        let mut gen = 0;

        loop {
            println!("{self}");

            print!("\x1b[{};0H", HEIGHT + 1);
            println!("      gen: {gen:>4}");
            println!("survivors: {:>4}", self.survivors().len());
            gen += 1;

            std::thread::sleep(SLEEPING);

            self.cells = self.next_gen();
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{CLEAR_SCREEN}")?;
        for pos in &self.cells {
            write!(f, "\x1b[{};{}H", pos.y, pos.x)?;
            write!(f, "{ALIVE}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_alive() {
        let glider = Board::from(GLIDER.to_vec());
        let p = Pos::from((4, 2));
        let q = Pos::from((1, 1));
        assert!(glider.is_alive(&p));
        assert!(!glider.is_alive(&q));
    }

    #[test]
    fn test_live_neighbors() {
        let glider = Board::from(GLIDER.to_vec());
        let answer = glider
            .cells
            .iter()
            .map(|p| glider.live_neighbors(p))
            .collect::<Vec<_>>();
        assert_eq!(vec![1, 3, 1, 3, 2], answer);
    }

    #[test]
    fn test_survivors() {
        let glider = Board::from(GLIDER.to_vec());
        let answer = [(3, 4), (4, 3), (4, 4)].map(Pos::from).to_vec();
        assert_eq!(glider.survivors(), answer);
    }

    #[test]
    fn test_births() {
        let glider = Board::from(GLIDER.to_vec());
        let answer = [(3, 2), (5, 3)].map(Pos::from).to_vec();
        assert_eq!(glider.births(), answer);
    }

    #[test]
    fn test_next_gen() {
        let glider = Board::from(GLIDER.to_vec());
        let answer = [(3, 4), (4, 3), (4, 4), (3, 2), (5, 3)]
            .map(Pos::from)
            .to_vec();
        assert_eq!(glider.next_gen(), answer);
    }
}
