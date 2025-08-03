#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TileType {
    Grass,
    Water,
    Sand,
    Rock,
    Tree,
    Wall,
    Stone,
    Inventory,
    Swamp,
    Slime,
    Snow,
    Ice,
    Cactus,
    Cave,
    Lava,
    //PlayerSpawn,
    Exit(ExitTile),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExitTile {
    pub next_level: String,
    pub locked: bool,
}

impl TileType {
    // Associated constants for each tile type's color
    pub const GRASS_COLOR: (u8, u8, u8) = (0, 255, 0);
    pub const WATER_COLOR: (u8, u8, u8) = (0, 0, 255);
    pub const SAND_COLOR: (u8, u8, u8) = (255, 255, 0);
    pub const ROCK_COLOR: (u8, u8, u8) = (128, 128, 128);
    pub const TREE_COLOR: (u8, u8, u8) = (34, 139, 34);
    pub const STONE_COLOR: (u8, u8, u8) = (193, 193, 193);
    pub const WALL_COLOR: (u8, u8, u8) = (50, 47, 77);
    pub const PLAYER_SPAWN_COLOR: (u8, u8, u8) = (255, 0, 0);
    pub const EXIT_COLOR: (u8, u8, u8) = (64, 58, 171);
    pub const INVENTORY_COLOR: (u8, u8, u8) = (255, 0, 255);
    pub const SWAMP_COLOR: (u8, u8, u8) = (0, 49, 31);
    pub const SLIME_COLOR: (u8, u8, u8) = (74, 91, 33);
    pub const SNOW_COLOR: (u8, u8, u8) = (255, 255, 255);
    pub const ICE_COLOR: (u8, u8, u8) = (115, 155, 208);
    pub const CAVE_COLOR: (u8, u8, u8) = (138, 68, 17);
    pub const LAVA_COLOR: (u8, u8, u8) = (255, 102, 0);
    pub const CACTUS_COLOR: (u8, u8, u8) = (96, 164, 79);
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
            TileType::Inventory => TileType::INVENTORY_COLOR,
            TileType::Swamp => TileType::SWAMP_COLOR,
            TileType::Slime => TileType::SLIME_COLOR,
            TileType::Snow => TileType::SNOW_COLOR,
            TileType::Ice => TileType::ICE_COLOR,
            TileType::Cave => TileType::CAVE_COLOR,
            TileType::Lava => TileType::LAVA_COLOR,
            TileType::Cactus => TileType::CACTUS_COLOR,
            //TileType::PlayerSpawn => TileType::PLAYER_SPAWN_COLOR,
            TileType::Exit(..) => TileType::EXIT_COLOR,
            // Add other variants here...
        }
    }
}
