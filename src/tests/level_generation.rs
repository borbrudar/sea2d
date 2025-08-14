use crate::environment::{
    autotiler::{Autotiler, TileSetType},
    tile_type::TileType,
};
use crate::wfc::overlap::{Pattern, generate_wfc};
use rand::{SeedableRng, rngs::StdRng};

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
    // This should make a very simple pattern set for predictable output
    vec![vec![vec![[0, 255, 0, 102]]]] // Single green tile
}

#[test]
fn autotiler_returns_some_texture_for_simple_case() {
    let mut autotiler = Autotiler::new();
    autotiler.add_tile(TileType::Grass, TileSetType::Simple, "fake_path.png".into());

    let neighbours = [[false; 3]; 3];
    let tex = autotiler.get_tile_texture(neighbours, TileType::Grass);

    assert!(tex.is_some(), "Autotiler returned None for a simple tile");
}
