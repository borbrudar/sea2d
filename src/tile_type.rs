pub enum TileType {
    Grass,
    Water,
    Sand,
    Rock,
    Tree,
    Wall,
    Stone,
    PlayerSpawn,
}

impl TileType {
    // Associated constants for each tile type's color
    pub const GRASS_COLOR: (u8, u8, u8) = (0, 255, 0);
    pub const WATER_COLOR: (u8, u8, u8) = (0, 0, 255);
    pub const SAND_COLOR: (u8, u8, u8) = (255, 255, 0);
    pub const ROCK_COLOR: (u8, u8, u8) = (128, 128, 128);
    pub const TREE_COLOR: (u8, u8, u8) = (34, 139, 34);
    pub const STONE_COLOR : (u8,u8,u8) = (192,192,192);
    pub const WALL_COLOR : (u8,u8,u8) = (50,47,77);
    pub const PLAYER_SPAWN_COLOR : (u8,u8,u8) = (255,0,0);
    // Other colors for each type can be defined here...

    pub fn _get_color(&self) -> (u8, u8, u8) {
        match *self {
            TileType::Grass => TileType::GRASS_COLOR,
            TileType::Water => TileType::WATER_COLOR,
            TileType::Sand => TileType::SAND_COLOR,
            TileType::Rock => TileType::ROCK_COLOR,
            TileType::Tree => TileType::TREE_COLOR,
            TileType::Stone => TileType::STONE_COLOR,
            TileType::Wall => TileType::WALL_COLOR,
            TileType::PlayerSpawn => TileType::PLAYER_SPAWN_COLOR,
            // Add other variants here...
        }
    }
}
