pub const WIDTH: i8 = 120;
pub const HEIGHT: i8 = 40;

pub const ALIVE: &str = "*";

pub const CLEAR_SCREEN: &str = "\x1b[2J";

pub const SLEEPING: std::time::Duration = std::time::Duration::from_millis(1);

#[allow(dead_code)]
pub const GLIDER: [(u8, u8); 5] = [(2, 3), (3, 4), (4, 2), (4, 3), (4, 4)];
#[rustfmt::skip]
#[allow(dead_code)]
pub const GLIDER_GUN: [(u8, u8); 37] = [
    (2, 6), (2, 7),
    (3, 6), (3, 7),
    (12, 6), (12, 7), (12, 8),
    (13, 5), (13, 9),
    (14, 4), (14, 10),
    (15, 4), (15, 10),
    (16, 7),
    (17, 5), (17, 9),
    (18, 6), (18, 7), (18, 7), (18, 8),
    (19, 7),
    (22, 4), (22, 5), (22, 6),
    (23, 4), (23, 5), (23, 6),
    (24, 3), (24, 7),
    (26, 2), (26, 3), (26, 7), (26, 8),
    (36, 4), (36, 5),
    (37, 4), (37, 5),
];
#[allow(dead_code)]
pub const ACORN: [(u8, u8); 7] = [(2, 4), (3, 2), (3, 4), (5, 3), (6, 4), (7, 4), (8, 4)];
#[allow(dead_code)]
pub const DIEHARD: [(u8, u8); 7] = [(2, 3), (3, 3), (3, 4), (7, 4), (8, 2), (8, 4), (9, 4)];
