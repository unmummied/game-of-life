mod board;
mod pos;
mod st;

use board::*;
use st::*;

fn main() {
    let glider_gun = Board::from(GLIDER_GUN.to_vec());
    glider_gun.life();
}
