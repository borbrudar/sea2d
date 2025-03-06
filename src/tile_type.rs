pub struct TileTypeInfo {
    pub name: String,
    pub texture_name: String,
    pub editor_color: (u8, u8, u8),
}

impl TileTypeInfo {
    pub fn new(name: &str, texture_name: &str, editor_color: (u8, u8, u8)) -> TileTypeInfo {
        TileTypeInfo {
            name: name.to_string(),
            texture_name: texture_name.to_string(),
            editor_color,
        }
    }
}

pub enum TileType {
    Grass,
    Water,
    Sand,
    Rock,
    Tree,
    Bush,
    Flower,
    Dirt,
    Stone,
    Wood,
    Leaves,
    Empty,
}

impl TileType {
    // Associated constants for each tile type's color
    pub const GRASS_COLOR: (u8, u8, u8) = (0, 255, 0);
    pub const WATER_COLOR: (u8, u8, u8) = (0, 0, 255);
    pub const SAND_COLOR: (u8, u8, u8) = (255, 255, 0);
    pub const ROCK_COLOR: (u8, u8, u8) = (128, 128, 128);
    pub const TREE_COLOR: (u8, u8, u8) = (34, 139, 34);
    pub const STONE_COLOR : (u8,u8,u8) = (192,192,192);
    // Other colors for each type can be defined here...

    pub fn get_color(&self) -> (u8, u8, u8) {
        match *self {
            TileType::Grass => TileType::GRASS_COLOR,
            TileType::Water => TileType::WATER_COLOR,
            TileType::Sand => TileType::SAND_COLOR,
            TileType::Rock => TileType::ROCK_COLOR,
            TileType::Tree => TileType::TREE_COLOR,
            TileType::Stone => TileType::STONE_COLOR,
            // Add other variants here...
            _ => (0, 0, 0), // Default case for Empty or unhandled types
        }
    }
}
