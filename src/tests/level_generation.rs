use crate::entities::enemy_generation::enemy_count_for_level;
use crate::environment::{
    autotiler::{Autotiler, TileSetType},
    tile_type::TileType,
};
use crate::wfc::overlap::{Pattern, generate_wfc};

#[test]
fn generated_level_is_correct_size() {
    let patterns = load_test_patterns();
    let width = 16;
    let height = 12;
    let grid = generate_wfc(&patterns, width, height, 1);

    assert_eq!(grid.width, width as usize, "Width does not match expected");
    assert_eq!(
        grid.height, height as usize,
        "Height does not match expected"
    );
}

fn load_test_patterns() -> Vec<Pattern> {
    vec![vec![vec![[0, 255, 0, 102]]]] // Single green walkable tile
}

#[test]
fn autotiler_returns_some_texture_for_simple_case() {
    let mut autotiler = Autotiler::new();
    autotiler.add_tile(TileType::Grass, TileSetType::Simple, "fake_path.png".into());

    let neighbours = [[false; 3]; 3];
    let tex = autotiler.get_tile_texture(neighbours, TileType::Grass);

    assert!(tex.is_some(), "Autotiler returned None for a simple tile");
}

#[test]
fn test_enemy_count_levels() {
    // Levels before 3 always have 1 enemy
    assert_eq!(enemy_count_for_level(0), 1);
    assert_eq!(enemy_count_for_level(2), 1);

    // Level 3 should have 2 enemies
    assert_eq!(enemy_count_for_level(3), 2);

    // Level 7 should have 3 enemies ((7-3)/4 = 1 -> 1 + 1 + 1 = 3)
    assert_eq!(enemy_count_for_level(7), 3);

    // Level 11 should have 4 enemies
    assert_eq!(enemy_count_for_level(11), 4);
}
