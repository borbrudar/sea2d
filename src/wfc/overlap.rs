/// Modul za generiranje nivojev z uporabo Overlap Wave Function Collapse (WFC) algoritma
use core::panic;
use image;
use rand::Rng;
use rand::prelude::IndexedRandom;
use rand::seq::SliceRandom;
use std::collections::VecDeque;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

//sample: 5x5 ploščic, 10x10 pixlov
const SAMPLE_TILE_SIZE: usize = 2;
pub const TILE_SIZE: usize = 1;
const GRID_HEIGHT: usize = 16;
const GRID_WIDTH: usize = 20;

pub const EXIT_RGBA: [u8; 4] = [64, 58, 171, 102]; // RGBA color for exit tile
pub const SPAWN_RGBA: [u8; 4] = [255, 0, 0, 102]; // RGBA color for player spawn tile
pub const WALL_RGBA: [u8; 4] = [50, 47, 77, 255]; //RGBA color for wrap around wall

/// Vzorec je 2D matrika barv v RGBA formatu.
pub type Pattern = Vec<Vec<[u8; 4]>>; // 2D array of RGBA colors

/// Funkcija izvleče vzorce iz vzorčne slike.
pub fn extract_patterns(path: &str, n: usize) -> Vec<Pattern> {
    let img = image::open(path).unwrap().to_rgba8();
    let (width, height) = img.dimensions();
    let tile_px = SAMPLE_TILE_SIZE;
    let tiles_x = width as usize / tile_px;
    let tiles_y = height as usize / tile_px;

    let mut patterns = Vec::new();

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

            if !patterns.contains(&pattern) {
                patterns.push(pattern);
            }
        }
    }

    println!("Extracted {} unique patterns", patterns.len());
    patterns
}

/// Iz vzorcev generira mrežo vzorcev.
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

/// Struktura, ki predstavlja mrežo ploščic.
pub struct TileGrid {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<[u8; 4]>>,
}

/// Pretvori mrežo vzorcev v mrežo ploščic.
pub fn flatten_patterns_to_tile_grid(pattern_grid: &Vec<Vec<Pattern>>, n: usize) -> TileGrid {
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

    TileGrid {
        width: final_width,
        height: final_height,
        tiles: tile_grid,
    }
}

impl TileGrid {
    /// Ustvari novo prazno mrežo ploščic z danimi dimenzijami.
    pub fn new(width: usize, height: usize) -> Self {
        TileGrid {
            width,
            height,
            tiles: vec![vec![[0; 4]; width]; height],
        }
    }

    /// Preveri, ali je mreža ploščic popolnoma povezana s prehodnimi ploščicami.
    pub fn is_fully_connected(&self) -> bool {
        let height = self.height;
        let width = self.width;

        let mut walkable = vec![vec![false; width]; height];
        let mut total_walkable = 0;

        // Mark walkable tiles
        for y in 0..height {
            for x in 0..width {
                let alpha = self.tiles[y][x][3]; // A channel
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

    /// Postavi ploščico za pojavno mesto igralca na dani koordinati.
    pub fn place_spawn_tile(&mut self, (x, y): (usize, usize), color: [u8; 4]) {
        if y + 1 < self.height && x < self.width {
            self.tiles[y + 1][x + 1] = color
        } else {
            panic!(
                "Attempted to place tile out of bounds at ({}, {})",
                x + 1,
                y + 1
            );
        }
    }

    /// Ustvari novo mrežo ploščic z obrobo, ki obdaja obstoječo mrežo.
    /// Obroba je napolnjena z dano barvo ploščice.
    pub fn wrap_edge(&self, tile_color: [u8; 4]) -> Self {
        let height = self.height;
        let width = self.width;

        // Create a new grid with +2 in both dimensions
        let mut new_grid = vec![vec![tile_color; width + 2]; height + 2];

        // Copy old grid into center of new grid
        for y in 0..height {
            for x in 0..width {
                new_grid[y + 1][x + 1] = self.tiles[y][x];
            }
        }

        TileGrid {
            width: width + 2,
            height: height + 2,
            tiles: new_grid,
        }
    }

    /// Postavi ploščico na rob mreže na dani koordinati.
    /// To funkcijo se uporablja na mreži, ki je že obrobljena z `wrap_edge`.
    pub fn place_tile_on_edge(
        &mut self,
        edge: Option<Edge>,
        (x, y): (usize, usize),
        color: [u8; 4],
    ) {
        if let Some(e) = edge {
            let adj_coord;
            match e {
                Edge::Bottom => adj_coord = (x + 1, y + 2),
                Edge::Top => adj_coord = (x + 1, 0),
                Edge::Left => adj_coord = (0, y + 1),
                Edge::Right => adj_coord = (x + 2, y + 1),
            }
            if adj_coord.1 < self.height && adj_coord.0 < self.width {
                self.tiles[adj_coord.1][adj_coord.0] = color;
            } else {
                panic!(
                    "Attempted to place tile out of bounds at ({}, {})",
                    adj_coord.0, adj_coord.1
                );
            }
        } else {
            if y + 1 < self.height && x + 1 < self.width {
                self.tiles[y + 1][x + 1] = color
            } else {
                panic!(
                    "Attempted to place tile out of bounds at ({}, {})",
                    x + 1,
                    y + 1
                );
            }
        }
    }

    /// Naloži izhodno ploščico na rob mreže, pri čemer se izogne prepovedanemu robu (kjer je pojavno mesto igralca).
    fn load_exit_tile(&self, forbidden_edge: Option<Edge>) -> ((usize, usize), Option<Edge>) {
        let mut rng = rand::rng();
        let width = self.width;
        let height = self.height;

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
        for (edge, tiles) in edge_tiles {
            let candidates: Vec<_> = tiles
                .into_iter()
                .filter(|&(x, y)| self.tiles[y][x][3] < 128) // walkable
                .collect();
            if let Some(&exit_pos) = candidates.choose(&mut rng) {
                return (exit_pos, Some(edge));
            }
        }

        panic!("No valid edge found for exit placement");
    }

    /// Poišče naključno pojavno mesto na ustreznem robu mreže.
    fn load_spawn(&self, edge: Edge) -> (usize, usize) {
        let mut rng = rand::rng();
        let width = self.width;
        let height = self.height;

        let candidates: Vec<_> = match edge {
            Edge::Top => (0..width).map(|x| (x, 0)).collect(),
            Edge::Bottom => (0..width).map(|x| (x, height - 1)).collect(),
            Edge::Left => (0..height).map(|y| (0, y)).collect(),
            Edge::Right => (0..height).map(|y| (width - 1, y)).collect(),
        };

        let walkables: Vec<_> = candidates
            .into_iter()
            .filter(|&(x, y)| self.tiles[y][x][3] < 128)
            .collect();

        *walkables
            .choose(&mut rng)
            .expect("No valid spawn tile found near edge")
    }

    /// Poišče naključno prehodno ploščico v mreži.
    pub fn random_walkable_tile(&self) -> (usize, usize) {
        let mut rng = rand::rng();

        let walkable_positions: Vec<(usize, usize)> =
            self.tiles
                .iter()
                .enumerate()
                .flat_map(|(y, row)| {
                    row.iter().enumerate().filter_map(move |(x, pixel)| {
                        if pixel[3] < 128 { Some((x, y)) } else { None }
                    })
                })
                .collect();

        *walkable_positions
            .choose(&mut rng)
            .expect("No walkable tiles found in the tile grid")
    }

    /// Shrani izhodno sliko mreže ploščic v PNG datoteko.
    pub fn save_output_image(&self, tile_size: u32, output_path: &str) {
        if let Some(parent) = Path::new(output_path).parent() {
            fs::create_dir_all(parent).expect("Failed to create output directory");
        }

        let width = self.width as u32;
        let height = self.height as u32;
        let mut img =
            image::RgbaImage::new((width * tile_size) as u32, (height * tile_size) as u32);

        for y in 0..height as usize {
            for x in 0..width as usize {
                let colour = self.tiles[y][x]; // top-left color
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
    }

    /// Prebere pojavno mesto igralca iz mreže ploščic.
    pub fn read_spawn(&self) -> (i32, i32) {
        let height = self.height;
        let width = self.width;

        for ty in 0..height {
            for tx in 0..width {
                if self.tiles[ty][tx] == SPAWN_RGBA {
                    return (tx as i32, ty as i32); // x is column, y is row
                }
            }
        }

        panic!("Couldn't read spawn: no tile matches SPAWN_RGBA");
    }
}

/// Generira mrežo ploščic z uporabo Overlap WFC algoritma.
/// Ponavlja generacijo, dokler ni mreža povezana.
pub fn generate_wfc(patterns: &Vec<Pattern>, width: u32, height: u32, n: usize) -> TileGrid {
    let mut tile_grid = TileGrid::new(width as usize, height as usize);
    let mut connected = false;
    while !connected {
        let pattern_grid =
            generate_pattern_grid(&patterns, width - n as u32 + 1, height - n as u32 + 1);
        tile_grid = flatten_patterns_to_tile_grid(&pattern_grid, n);
        connected = tile_grid.is_fully_connected();
    }
    println!("successfully generated a connected tile grid");
    tile_grid
}

/// Iz PNG datoteke ustvari mrežo ploščic.
pub fn get_tile_grid_from_png(path: &str, tile_size: u32) -> Option<TileGrid> {
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

    let mut tile_grid = TileGrid::new(width as usize, height as usize);

    for ty in 0..tiles_y {
        for tx in 0..tiles_x {
            let px = tx * tile_size;
            let py = ty * tile_size;
            let pixel = img.get_pixel(px, py);
            tile_grid.tiles[py as usize][px as usize] = pixel.0;
        }
    }
    Some(tile_grid)
}

/// Struktura, ki predstavlja rob mreže ploščic.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edge {
    Top,
    Bottom,
    Left,
    Right,
}

/// Poišče nasprotni rob danega roba mreže ploščic.
fn opposite_edge(edge: &Edge) -> Edge {
    match edge {
        Edge::Top => Edge::Bottom,
        Edge::Bottom => Edge::Top,
        Edge::Left => Edge::Right,
        Edge::Right => Edge::Left,
    }
}

/// Poišče izhodno ploščico na robu mreže ploščic iz PNG datoteke in vrne njen rob, če je najden.
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

/// Ustvari izhodno datoteko z imenom trenutnega nivoja in potjo do naslednjega nivoja.
pub fn write_exits_file(current_level_name: &str, next_level_path: &str) {
    let exits_file_path = format!(
        "resources/levels/{}/{}_exits.txt",
        current_level_name, current_level_name
    );
    let mut file = File::create(&exits_file_path)
        .unwrap_or_else(|_| panic!("Failed to create exits file: {}", exits_file_path));

    writeln!(file, "{}", next_level_path)
        .unwrap_or_else(|_| panic!("Failed to write to exits file: {}", exits_file_path));
}

/// Zažene Overlap WFC algoritem za generiranje nivoja z danim indeksom `k` in `i`.
/// `k` določa velikost vzorcev, `i` pa indeks prejšnjega nivoja.
pub fn run_overlap(k: i32, i: i32) {
    let patterns = extract_patterns(&format!("resources/levels/sample_{}.png", k), 3);

    let width = GRID_WIDTH as u32; // grid width in tiles
    let height = GRID_HEIGHT as u32; // grid height in tiles
    let mut tile_grid = generate_wfc(&patterns, width, height, 3);

    //find exit tile
    let forbidden_exit_edge = find_exit_tile_edge(
        &format!("resources/levels/level{}/level{}_1.png", i, i),
        TILE_SIZE as u32,
    )
    .map(|e| opposite_edge(&e));

    let (exit_pos, exit_edge) = tile_grid.load_exit_tile(forbidden_exit_edge);

    //find spawn tile
    let (spawn_pos, _spawn_edge) = if let Some(prev_edge) = forbidden_exit_edge {
        (tile_grid.load_spawn(prev_edge), Some(prev_edge))
    } else {
        (tile_grid.random_walkable_tile(), None)
    };

    //wrap edge of the map
    tile_grid = tile_grid.wrap_edge(WALL_RGBA);

    //place exit
    tile_grid.place_tile_on_edge(exit_edge, exit_pos, EXIT_RGBA);

    //create second layer
    let mut second_layer = TileGrid::new((width + 2) as usize, (height + 2) as usize);

    //place spawn on second layer
    second_layer.place_spawn_tile(spawn_pos, SPAWN_RGBA);

    //save level image
    tile_grid.save_output_image(
        TILE_SIZE as u32,
        &format!("resources/levels/level{}/level{}_1.png", i + 1, i + 1),
    );

    // save second layer
    second_layer.save_output_image(
        TILE_SIZE as u32,
        &format!("resources/levels/level{}/level{}_2.png", i + 1, i + 1),
    );

    //create exit file
    write_exits_file(
        &format!("level{}", i + 1),
        &format!("resources/levels/level{}/level{}_1.png", i + 2, i + 2),
    );
}

/// Izvleče indeks nivoja iz poti do nivoja.
pub fn extract_level_index(level_path: &str) -> Option<i32> {
    let path = Path::new(level_path);
    let folder_name = path.parent()?.file_name()?.to_str()?;

    if let Some(index_str) = folder_name.strip_prefix("level") {
        if let Ok(index) = index_str.parse::<i32>() {
            return Some(index);
        }
    }

    None
}

/// Pridobi pot do mape nivoja iz poti do nivoja.
fn get_folder_path_from_level(level_path: &str) -> Option<String> {
    Path::new(level_path)
        .parent()
        .map(|p| p.to_string_lossy().to_string())
}

/// Izbriše mapo nivoja in vse njene datoteke.
fn delete_level_folder(folder_path: &str) -> std::io::Result<()> {
    let path = Path::new(folder_path);
    if path.exists() {
        fs::remove_dir_all(path)?; // Recursively deletes folder + files
    } else {
        println!("Folder does not exist: {}", folder_path);
    }
    Ok(())
}

/// Shrani najvišji nivo, če je novi nivo višji od trenutnega najvišjega.
pub fn save_highest_level_if_higher(level: i32, path: &str) -> std::io::Result<()> {
    let path_obj = Path::new(path);

    // Read current highest if file exists
    let current_highest = if path_obj.exists() {
        let mut contents = String::new();
        fs::File::open(path_obj)?.read_to_string(&mut contents)?;
        contents.trim().parse::<i32>().unwrap_or(0)
    } else {
        0
    };

    // Only update if level is higher
    if level > current_highest {
        if let Some(parent) = path_obj.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = fs::File::create(path_obj)?;
        write!(file, "{}", level)?;
    }

    Ok(())
}

/// Prebere najvišji nivo iz datoteke.
pub fn read_highest_level(path: &str) -> i32 {
    let path_obj = Path::new(path);

    if !path_obj.exists() {
        return 0;
    }

    let mut contents = String::new();
    if let Ok(mut file) = fs::File::open(path_obj) {
        if file.read_to_string(&mut contents).is_ok() {
            return contents.trim().parse::<i32>().unwrap_or(0);
        }
    }

    0
}

/// Generira naslednji nivo z uporabo Overlap WFC algoritma.
//prev_level: Some(player.current_level)
pub fn wfc_level_generator(prev_level: Option<&String>) {
    let mut rng = rand::rng();
    let k = rng.random_range(1..=8);
    if let Some(previous) = prev_level {
        //parse previous
        let j = extract_level_index(&previous);

        //generate new
        if let Some(i) = j {
            run_overlap(k, i);
        } else {
            panic!("Couldn't extract number of previous level")
        }

        //delete previous level and exit file
        let folder = get_folder_path_from_level(&previous);
        if let Some(path) = folder {
            if let Err(e) = delete_level_folder(&path) {
                eprintln!("Error deleting folder: {}", e);
            }
        } else {
            panic!("Couldn't delete previous level folder")
        }
        println!("successfully generated next level!");
    } else {
        //first level
        run_overlap(k, 0);
        println!("first level successful");
    }
}
