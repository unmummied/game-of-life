pub const WIDTH: i8 = 30;
pub const HEIGHT: i8 = 30;

pub const ALIVE: &str = "*";

pub const CLEAR_SCREEN: &str = "\x1b[2J";

pub const SLEEPING: std::time::Duration = std::time::Duration::from_millis(100);

pub const GLIDER: [(u8, u8); 5] = [(2, 3), (3, 4), (4, 2), (4, 3), (4, 4)];
