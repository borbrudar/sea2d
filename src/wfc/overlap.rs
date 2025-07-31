use core::panic;
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

//nov sample: 5x5 ploščic, 10x10 pixlov
const SAMPLE_TILE_SIZE: usize = 2;
const TILE_SIZE: usize = 80;

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
    pattern_width: u32,
    pattern_height: u32,
) -> Vec<Vec<Pattern>> {
    let mut rng = rand::rng();
    let mut grid = vec![vec![patterns[0].clone(); pattern_width as usize]; pattern_height as usize];

    for y in 0..pattern_height as usize {
        for x in 0..pattern_width as usize {
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
/// Alpha >= 128: the tile is not walkable
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

pub fn generate_wfc(
    patterns: &Vec<Pattern>,
    width: u32,
    height: u32,
    n: usize,
) -> Vec<Vec<[u8; 4]>> {
    let mut tile_grid = Vec::new();
    let mut connected = false;
    while !connected {
        let pattern_grid =
            generate_pattern_grid(&patterns, width - n as u32 + 1, height - n as u32 + 1);
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
pub const SPAWN_RGBA: [u8; 4] = [255, 0, 0, 102]; // RGBA color for player spawn tile

//place player spawn point
// Reads the exit tile position from a PNG file
// Returns tile coordinates (x, y) if found

pub fn place_tile(tile_grid: &mut Vec<Vec<[u8; 4]>>, (x, y): (usize, usize), color: [u8; 4]) {
    if y < tile_grid.len() && x < tile_grid[y].len() {
        tile_grid[y][x] = color;
    } else {
        panic!("Attempted to place tile out of bounds at ({}, {})", x, y);
    }
}
use rand::seq::SliceRandom;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edge {
    Top,
    Bottom,
    Left,
    Right,
}

fn opposite_edge(edge: &Edge) -> Edge {
    match edge {
        Edge::Top => Edge::Bottom,
        Edge::Bottom => Edge::Top,
        Edge::Left => Edge::Right,
        Edge::Right => Edge::Left,
    }
}

pub fn find_exit_tile_edge(path: &str, tile_size: u32) -> Option<Edge> {
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

    for ty in 0..tiles_y {
        for tx in 0..tiles_x {
            let px = tx * tile_size;
            let py = ty * tile_size;
            let pixel = img.get_pixel(px, py);

            if pixel.0 == EXIT_RGBA {
                println!("Found exit tile at previous level");

                return match (tx, ty) {
                    (0, _) => Some(Edge::Left),
                    (x, _) if x == tiles_x - 1 => Some(Edge::Right),
                    (_, 0) => Some(Edge::Top),
                    (_, y) if y == tiles_y - 1 => Some(Edge::Bottom),
                    _ => None,
                };
            }
        }
    }

    None
}

fn load_exit_tile(tile_grid: &Vec<Vec<[u8; 4]>>, forbidden_edge: Option<Edge>) -> (usize, usize) {
    let mut rng = rand::rng();
    let width = tile_grid[0].len();
    let height = tile_grid.len();

    // Get all edge coordinates
    let mut edge_tiles = vec![
        (Edge::Top, (0..width).map(|x| (x, 0)).collect::<Vec<_>>()),
        (
            Edge::Bottom,
            (0..width).map(|x| (x, height - 1)).collect::<Vec<_>>(),
        ),
        (Edge::Left, (0..height).map(|y| (0, y)).collect::<Vec<_>>()),
        (
            Edge::Right,
            (0..height).map(|y| (width - 1, y)).collect::<Vec<_>>(),
        ),
    ];

    // Remove forbidden edge
    if let Some(forbidden) = forbidden_edge {
        edge_tiles.retain(|(edge, _)| *edge != forbidden);
    }

    // Shuffle edges and pick one
    edge_tiles.shuffle(&mut rng);
    for (_, tiles) in edge_tiles {
        let candidates: Vec<_> = tiles
            .into_iter()
            .filter(|&(x, y)| tile_grid[y][x][3] < 128) // walkable
            .collect();
        if let Some(&exit_pos) = candidates.choose(&mut rng) {
            return exit_pos;
        }
    }

    panic!("No valid edge found for exit placement");
}

fn load_spawn(tile_grid: &Vec<Vec<[u8; 4]>>, edge: Edge) -> (usize, usize) {
    let mut rng = rand::rng();
    let width = tile_grid[0].len();
    let height = tile_grid.len();

    let candidates: Vec<_> = match edge {
        Edge::Top => (0..width).map(|x| (x, 0)).collect(),
        Edge::Bottom => (0..width).map(|x| (x, height - 1)).collect(),
        Edge::Left => (0..height).map(|y| (0, y)).collect(),
        Edge::Right => (0..height).map(|y| (width - 1, y)).collect(),
    };

    let walkables: Vec<_> = candidates
        .into_iter()
        .filter(|&(x, y)| tile_grid[y][x][3] < 128)
        .collect();

    *walkables
        .choose(&mut rng)
        .expect("No valid spawn tile found near edge")
}

pub fn random_walkable_tile(tile_grid: &Vec<Vec<[u8; 4]>>) -> (usize, usize) {
    let mut rng = rand::rng();

    let walkable_positions: Vec<(usize, usize)> = tile_grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(
                move |(x, pixel)| {
                    if pixel[3] < 128 { Some((x, y)) } else { None }
                },
            )
        })
        .collect();

    *walkable_positions
        .choose(&mut rng)
        .expect("No walkable tiles found in the tile grid")
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
            let width = 16; // grid width in tiles <- te bi mogoče popravl v const
            let height = 12; // grid height in tiles
            let mut tile_grid = generate_wfc(&patterns, width, height, 3);

            //place exit tile
            let forbidden_exit_edge = find_exit_tile_edge(
                &format!("resources/levels/output_image_{}.png", i),
                TILE_SIZE as u32,
            )
            .map(|e| opposite_edge(&e));

            let exit_pos = load_exit_tile(&tile_grid, forbidden_exit_edge);

            let spawn_pos = if let Some(prev_edge) = forbidden_exit_edge {
                load_spawn(&tile_grid, prev_edge)
            } else {
                random_walkable_tile(&tile_grid)
            };

            place_tile(&mut tile_grid, exit_pos, EXIT_RGBA);
            place_tile(&mut tile_grid, spawn_pos, SPAWN_RGBA);

            save_output_image(
                &tile_grid,
                TILE_SIZE as u32,
                &format!("resources/levels/output_image_{}.png", i + 1),
            );
            //create exit file
        }
    }
}
