/// Modul za generiranje sovražnikov v igri.
use crate::entities::enemy::{Enemy, EnemyType};
use crate::wfc::overlap::{TILE_SIZE, get_tile_grid_from_png};
use rand::Rng;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

/// Generira seznam sovražnikov glede na indeks nivoja.

pub fn enemy_count_for_level(i: i32) -> usize {
    let mut n = 1;
    if i >= 3 {
        let div = (i - 3) / 4;
        n += div + 1;
    }
    n as usize
}

pub fn generate_enemies<'a>(
    i: i32,
    texture_creator: &'a TextureCreator<WindowContext>,
    texture_map: &mut std::collections::HashMap<String, Texture<'a>>,
) -> Vec<Enemy> {
    let mut enemies = Vec::new();

    //compute the number of enemies given level index
    let n = enemy_count_for_level(i);

    for _ in 1..=n {
        //  pick enemy type
        let tip = pick_random_enemy_type();

        //  compute enemy_spawn_pt
        let spawn_pt = enemy_spawn_pt(&tip, i, 60);

        enemies.push(Enemy::new(tip, spawn_pt, texture_creator, texture_map))
    }

    //return enemies
    enemies
}

/// Naključno izbere tip sovražnika.
pub fn pick_random_enemy_type() -> EnemyType {
    let mut rng = rand::rng();
    let j = rng.random_range(1..=4);
    match j {
        1 => EnemyType::Slime,
        2 => EnemyType::Stonewalker,
        3 => EnemyType::Wizard,
        4 => EnemyType::Skull,
        _ => EnemyType::Placeholder,
    }
}

/// Določi začetno točko za pojavljanje sovražnikov na podlagi nivoja.
pub fn enemy_spawn_pt(tip: &EnemyType, level_index: i32, level_tile_size: i32) -> (f64, f64) {
    //read spawn from second layer picture
    let second_layer = format!(
        "resources/levels/level{}/level{}_2.png",
        level_index, level_index
    );

    let layer_grid = get_tile_grid_from_png(&second_layer, TILE_SIZE as u32);
    let spawn_pt;

    if let Some(grid) = layer_grid {
        spawn_pt = grid.read_spawn();
    } else {
        panic!("Couldn't find layer grid")
    }

    //read level png, get tile grid
    let level_string = format!(
        "resources/levels/level{}/level{}_1.png",
        level_index, level_index
    );

    let level_grid = get_tile_grid_from_png(&level_string, TILE_SIZE as u32);

    //check the path between it and player_spawn is at least 3 tiles
    if let Some(grid) = level_grid {
        loop {
            //pick a random walkable tile
            let (feet_x, feet_y) = grid.random_walkable_tile();
            let distance = (feet_x as i32 - spawn_pt.0).abs() + (feet_y as i32 - spawn_pt.1).abs(); // Manhattan distance

            if distance >= 3 {
                match tip {
                    EnemyType::Slime | EnemyType::Stonewalker => {
                        // slime and stonewalker height
                        let pixel_offset_y = -16.0;
                        let px = feet_x as f64 * level_tile_size as f64;
                        let py = feet_y as f64 * level_tile_size as f64 + pixel_offset_y;
                        return (px, py);
                    }
                    EnemyType::Wizard => {
                        // wizard height
                        let pixel_offset_y = -64.0; // wizard is taller
                        let px = feet_x as f64 * level_tile_size as f64;
                        let py = feet_y as f64 * level_tile_size as f64 + pixel_offset_y;
                        return (px, py);
                    }
                    EnemyType::Skull => {
                        // skull height
                        let pixel_offset_y = -32.0;
                        let px = feet_x as f64 * level_tile_size as f64;
                        let py = feet_y as f64 * level_tile_size as f64 + pixel_offset_y;
                        return (px, py);
                    }
                    _ => {}
                }
            }
        }
    } else {
        panic!("Couldn't find level grid")
    }
}
