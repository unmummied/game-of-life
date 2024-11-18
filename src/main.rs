mod board;
mod pos;
mod st;

use board::*;
use st::*;

fn main() {
    let glider = Board::from(GLIDER.to_vec());
    glider.life();
}
