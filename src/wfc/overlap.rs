use image::{GenericImageView, ImageBuffer, RgbaImage};
use rand::prelude::IndexedRandom;
use std::collections::HashMap;
use std::path::Path;

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
fn generate_wfc_grid(patterns: &Vec<Pattern>, width: u32, height: u32) -> Vec<Vec<Pattern>> {
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

fn save_output_image(grid: &Vec<Vec<Pattern>>, tile_size: u32, output_path: &str) {
    let width = grid[0].len() as u32 * tile_size;
    let height = grid.len() as u32 * tile_size;
    let mut img = ImageBuffer::new(width, height);

    for (y, row) in grid.iter().enumerate() {
        for (x, pattern) in row.iter().enumerate() {
            let color = pattern[0][0]; // top-left color
            for dy in 0..tile_size {
                for dx in 0..tile_size {
                    img.put_pixel(
                        x as u32 * tile_size + dx,
                        y as u32 * tile_size + dy,
                        image::Rgba([color[0], color[1], color[2], color[3]]),
                    );
                }
            }
        }
    }

    img.save(Path::new(output_path)).unwrap();
    println!("Saved output to {}", output_path);
}

pub fn run_overlap() {
    for n in 1..2 {
        let (patterns, frequencies) =
            extract_patterns(&format!("resources/levels/sample_{}.png", n), 3);
        for i in (n - 1) * 10..n * 10 {
            let width = 16; // grid width in tiles
            let height = 12; // grid height in tiles

            let grid = generate_wfc_grid(&patterns, width, height);
            save_output_image(
                &grid,
                TILE_SIZE as u32,
                &format!("resources/levels/output_image_{}.png", i + 1),
            );
        }
    }
}
