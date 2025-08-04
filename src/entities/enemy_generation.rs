use crate::entities::enemy::{Enemy, EnemyType};
use crate::wfc::overlap::{SPAWN_RGBA, TILE_SIZE, random_walkable_tile};
use rand::Rng;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use std::path::Path;

pub fn generate_enemies<'a>(
    i: i32,
    texture_creator: &'a TextureCreator<WindowContext>,
    texture_map: &mut std::collections::HashMap<String, Texture<'a>>,
) -> Vec<Enemy> {
    let mut enemies = Vec::new();

    //compute the number of enemies given level index
    let mut n = 1;
    if i >= 3 {
        let div = (i - 3) / 4;
        println!("div: {}", div);
        n += div + 1;
    }
    for _ in 1..=n {
        //  pick enemy type
        let tip = pick_random_enemy_type();

        //  compute enemy_spawn_pt
        let spawn_pt = enemy_spawn_pt(i, 60);

        enemies.push(Enemy::new(tip, spawn_pt, texture_creator, texture_map))
    }

    //return enemies
    enemies
}

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

pub fn get_tile_grid_from_png(path: &str, tile_size: u32) -> Option<Vec<Vec<[u8; 4]>>> {
    if !Path::new(path).exists() {
        return None; // File doesn't exist
    }

    let img = match image::open(path) {
        Ok(img) => img.to_rgba8(),
        Err(_) => return None, // File is unreadable or invalid format
    };

    let (width, height) = img.dimensions();
    let tiles_x = width / tile_size;
    let tiles_y = height / tile_size;

    let mut tile_grid = vec![vec![[0; 4]; width as usize]; height as usize];

    for ty in 0..tiles_y {
        for tx in 0..tiles_x {
            let px = tx * tile_size;
            let py = ty * tile_size;
            let pixel = img.get_pixel(px, py);
            tile_grid[py as usize][px as usize] = pixel.0;
        }
    }
    Some(tile_grid)
}

pub fn read_spawn(tile_grid: &Vec<Vec<[u8; 4]>>) -> (i32, i32) {
    let height = tile_grid.len();
    let width = tile_grid[0].len();

    for ty in 0..height {
        for tx in 0..width {
            if tile_grid[ty][tx] == SPAWN_RGBA {
                return (tx as i32, ty as i32); // x is column, y is row
            }
        }
    }

    panic!("Couldn't read spawn: no tile matches SPAWN_RGBA");
}

pub fn enemy_spawn_pt(level_index: i32, level_tile_size: i32) -> (f64, f64) {
    //read spawn from second layer picture
    let second_layer = format!(
        "resources/levels/level{}/level{}_2.png",
        level_index, level_index
    );

    let layer_grid = get_tile_grid_from_png(&second_layer, TILE_SIZE as u32);
    let mut spawn_pt = (0, 0);

    if let Some(grid) = layer_grid {
        spawn_pt = read_spawn(&grid);
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
            let (x, y) = random_walkable_tile(&grid);
            let distance = (x as i32 - spawn_pt.0).abs() + (y as i32 - spawn_pt.1).abs(); // Manhattan distance

            if distance >= 3 {
                println!("enemy spawn: {}, {}", x, y);
                return (
                    (x * level_tile_size as usize) as f64,
                    (y * level_tile_size as usize) as f64,
                );
            }
        }
    } else {
        panic!("Couldn't find level grid")
    }
}
