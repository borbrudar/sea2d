use image::{GenericImageView, ImageBuffer, RgbaImage};
use rand::prelude::IndexedRandom;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::path::Path;

use crate::environment::{
    tile,
    tile_type::{ExitTile, TileType},
};

//TODO: can add symmetries, rotations
//exits, exit files
//automatic saving of levels (string logic).
//add which sample belongs to which level

//trenuten sample je 10x10 ploščic, 200x200 px; koda je temu prilagojena
//nov sample: 5x5 ploščic, 10x10 pixlov
const SAMPLE_TILE_SIZE: usize = 2;
const TILE_SIZE: usize = 50;

pub type Pattern = Vec<Vec<[u8; 4]>>; // 2D array of RGBA colors

pub fn extract_patterns(path: &str, n: usize) -> (Vec<Pattern>, HashMap<Pattern, usize>) {
    let img = image::open(path).unwrap().to_rgba8();
    let (width, height) = img.dimensions();
    let tile_px = SAMPLE_TILE_SIZE;
    let tiles_x = width as usize / tile_px;
    let tiles_y = height as usize / tile_px;

    let mut patterns = Vec::new();
    let mut frequencies = HashMap::new();

    for ty in 0..=tiles_y - n {
        for tx in 0..=tiles_x - n {
            let mut pattern: Pattern = Vec::new();

            for dy in 0..(n * tile_px) {
                let mut row: Vec<[u8; 4]> = Vec::new();

                for dx in 0..(n * tile_px) {
                    let px = (tx * tile_px) + dx;
                    let py = (ty * tile_px) + dy;

                    let pixel = img.get_pixel(px as u32, py as u32).0;
                    row.push([pixel[0], pixel[1], pixel[2], pixel[3]]);
                }

                pattern.push(row);
            }

            if !frequencies.contains_key(&pattern) {
                patterns.push(pattern.clone());
            }
            *frequencies.entry(pattern).or_insert(0) += 1;
        }
    }

    println!(
        "Extracted {} unique patterns ({} total)",
        patterns.len(),
        frequencies.len()
    );
    (patterns, frequencies)
}

//frequencies not used here (yet?!)
pub fn generate_pattern_grid(
    patterns: &Vec<Pattern>,
    width: u32,
    height: u32,
) -> Vec<Vec<Pattern>> {
    let mut rng = rand::rng();
    let mut grid = vec![vec![patterns[0].clone(); width as usize]; height as usize];

    for y in 0..height as usize {
        for x in 0..width as usize {
            let pat = patterns.choose(&mut rng).unwrap();
            grid[y][x] = pat.clone();
        }
    }

    grid
}

pub fn flatten_patterns_to_tile_grid(
    pattern_grid: &Vec<Vec<Pattern>>,
    n: usize,
) -> Vec<Vec<[u8; 4]>> {
    let grid_height = pattern_grid.len();
    let grid_width = pattern_grid[0].len();

    let final_height = grid_height + n - 1;
    let final_width = grid_width + n - 1;

    let mut tile_grid = vec![vec![[0, 0, 0, 0]; final_width]; final_height];

    for gy in 0..grid_height {
        for gx in 0..grid_width {
            let pattern = &pattern_grid[gy][gx];
            for dy in 0..n {
                for dx in 0..n {
                    let y = gy + dy;
                    let x = gx + dx;
                    if y < final_height && x < final_width {
                        tile_grid[y][x] = pattern[dy][dx];
                    }
                }
            }
        }
    }

    tile_grid
}

/// Check if the tile grid is fully connected via walkable tiles
/// Alpha >= 128 means the tile is not walkable
pub fn is_fully_connected(tile_grid: &Vec<Vec<[u8; 4]>>) -> bool {
    let height = tile_grid.len();
    let width = tile_grid[0].len();

    let mut walkable = vec![vec![false; width]; height];
    let mut total_walkable = 0;

    // Mark walkable tiles
    for y in 0..height {
        for x in 0..width {
            let alpha = tile_grid[y][x][3]; // A channel
            if alpha < 128 {
                walkable[y][x] = true;
                total_walkable += 1;
            }
        }
    }

    // Find a starting walkable tile
    let mut start = None;
    'outer: for y in 0..height {
        for x in 0..width {
            if walkable[y][x] {
                start = Some((x, y));
                break 'outer;
            }
        }
    }

    if start.is_none() {
        println!("No walkable tiles found in the grid.");
        return false; // no walkable tiles at all
    }

    let (sx, sy) = start.unwrap();
    let mut visited = vec![vec![false; width]; height];
    let mut queue = VecDeque::new();
    queue.push_back((sx, sy));
    visited[sy][sx] = true;

    let mut visited_count = 1;
    let directions = [(0i32, -1), (1, 0), (0, 1), (-1, 0)];

    //BFS
    while let Some((x, y)) = queue.pop_front() {
        for (dx, dy) in &directions {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx >= 0 && ny >= 0 && (nx as usize) < width && (ny as usize) < height {
                let (nx, ny) = (nx as usize, ny as usize);
                if walkable[ny][nx] && !visited[ny][nx] {
                    visited[ny][nx] = true;
                    visited_count += 1;
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    visited_count == total_walkable
}

//generate_wfc function
pub fn generate_wfc(
    patterns: &Vec<Pattern>,
    width: u32,
    height: u32,
    n: usize,
) -> Vec<Vec<[u8; 4]>> {
    let mut tile_grid = Vec::new();
    let mut connected = false;
    while !connected {
        let pattern_grid = generate_pattern_grid(&patterns, width, height);
        tile_grid = flatten_patterns_to_tile_grid(&pattern_grid, n);
        connected = is_fully_connected(&tile_grid);
        println!("grid not connected, retrying...");
    }
    println!("successfully generated a connected tile grid");
    tile_grid
}

pub fn edge_coordinates(width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();

    for x in 0..width {
        edges.push((x, 0)); // top edge
        edges.push((x, height - 1)); // bottom edge
    }

    for y in 1..(height - 1) {
        edges.push((0, y)); // left edge
        edges.push((width - 1, y)); // right edge
    }

    edges
}

pub const EXIT_RGBA: [u8; 4] = [64, 58, 171, 102]; // RGBA color for exit tile

pub fn place_exit_tile(tile_grid: &Vec<Vec<[u8; 4]>>) -> Vec<Vec<[u8; 4]>> {
    let mut rng = rand::rng();
    let width = tile_grid[0].len();
    let height = tile_grid.len();

    let edges = edge_coordinates(width as usize, height as usize);
    let &(x, y) = edges.choose(&mut rng).expect("No edge positions found");

    // Clone the grid and modify the chosen edge tile
    let mut new_grid = tile_grid.clone();
    new_grid[y][x] = EXIT_RGBA;
    if !is_fully_connected(&new_grid) {
        println!("Exit tile placement resulted in disconnected grid, retrying...");
        return place_exit_tile(tile_grid);
    } else {
        println!("Placed exit at ({}, {})", x, y);
        new_grid
    }
}

fn save_output_image(tile_grid: &Vec<Vec<[u8; 4]>>, tile_size: u32, output_path: &str) {
    let width = tile_grid[0].len() as u32;
    let height = tile_grid.len() as u32;
    let mut img = image::RgbaImage::new((width * tile_size) as u32, (height * tile_size) as u32);

    for y in 0..height as usize {
        for x in 0..width as usize {
            let colour = tile_grid[y][x]; // top-left color
            for dy in 0..tile_size {
                for dx in 0..tile_size {
                    img.put_pixel(
                        x as u32 * tile_size + dx,
                        y as u32 * tile_size + dy,
                        image::Rgba(colour),
                    );
                }
            }
        }
    }

    img.save(Path::new(output_path)).unwrap();
    println!("Saved output to {}", output_path);
}

pub fn run_overlap() {
    for k in 1..2 {
        let (patterns, frequencies) =
            extract_patterns(&format!("resources/levels/sample_{}.png", k), 3);
        for i in (k - 1) * 10..k * 10 {
            let width = 16; // grid width in tiles
            let height = 12; // grid height in tiles
            let mut tile_grid = generate_wfc(&patterns, width, height, 3);
            tile_grid = place_exit_tile(&tile_grid);
            save_output_image(
                &tile_grid,
                TILE_SIZE as u32,
                &format!("resources/levels/output_image_{}.png", i + 1),
            );
        }
    }
}
