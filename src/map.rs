use crate::entity_manager::Position;

// Placeholder for map struct
pub type GameMap = [[u8; 8]; 8];

pub const MAP: [[u8; 8]; 8] = [
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
];

pub fn in_bounds(map: &[[u8; 8]; 8], position: &Position) -> bool {
    position.x < map[0].len() as u32 && position.y < map.len() as u32
}

// May panic, change this later
pub fn is_walkable(map: &[[u8; 8]; 8], position: &Position) -> bool {
    map[position.y as usize][position.x as usize] == 0
}
