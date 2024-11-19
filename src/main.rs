mod board;
mod pos;
mod st;

use board::*;
use st::*;

fn main() {
    let mut acorn = Board::from(ACORN.to_vec());
    println!("{:?}", acorn.life());
}
