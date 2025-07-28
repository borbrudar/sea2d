use sdl2::render::{Texture, TextureCreator};
use std::collections::HashMap;

use crate::entities::point::Point;
use crate::environment::autotiler::Autotiler;
use crate::environment::{
    level::Level,
    texture_data::TextureData,
    tile::Tile,
    tile_type::{ExitTile, TileType},
};
use crate::game::find_sdl_gl_driver;
use crate::networking::shared::{SCREEN_HEIGHT, SCREEN_WIDTH};
use rand::{Rng, prelude::IndexedRandom, rng};
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

pub mod overlap;

const TILE_SIZE: usize = 50;
const GRID_WIDTH: usize = SCREEN_WIDTH as usize / TILE_SIZE;
const GRID_HEIGHT: usize = SCREEN_HEIGHT as usize / TILE_SIZE;

#[derive(Debug, Clone)]
pub struct WfcTile {
    pub tile_type: TileType,
    pub edges: [Vec<TileType>; 4], // Up, Right, Down, Left
}

pub struct WFCState {
    pub grid: Vec<Vec<Cell>>,
    pub tileset: Vec<WfcTile>,
}

impl<'a> WFCState {
    pub fn new(tileset: &Vec<WfcTile>) -> Self {
        let tileset = tileset.to_vec();
        let valid_tile_count = tileset
            .iter()
            .position(|tile| matches!(tile.tile_type, TileType::Exit(_)))
            .unwrap_or(tileset.len());

        loop {
            let mut grid =
                vec![vec![Cell::new(valid_tile_count); GRID_WIDTH as usize]; GRID_HEIGHT as usize];
            let mut success = true;

            while grid
                .iter()
                .any(|row| row.iter().any(|cell| !cell.collapsed))
            {
                let mut uncollapsed = vec![];
                for y in 0..GRID_HEIGHT {
                    for x in 0..GRID_WIDTH {
                        if !grid[y][x].collapsed {
                            uncollapsed.push((x, y));
                        }
                    }
                }

                if uncollapsed.is_empty() {
                    break;
                }

                uncollapsed.sort_by_key(|&(x, y)| grid[y][x].entropy());

                // Find all cells with minimum entropy
                let min_entropy = grid[uncollapsed[0].1][uncollapsed[0].0].entropy();
                let min_cells: Vec<_> = uncollapsed
                    .into_iter()
                    .filter(|&(x, y)| grid[y][x].entropy() == min_entropy)
                    .collect();

                // Randomly pick one among them
                let (x, y) = *min_cells.choose(&mut rng()).unwrap();

                grid[y][x].collapse();

                if !propagate(&mut grid, &tileset) {
                    success = false;
                    break; // contradiction → restart
                }
            }

            //add exit tile
            let edge_cells = edge_coordinates(GRID_WIDTH as usize, GRID_HEIGHT as usize);
            let mut rng = rng();
            let mut exit_placed = false;

            while !exit_placed {
                let &(x, y) = edge_cells.choose(&mut rng).expect("No edge cells");

                let cell = &mut grid[y][x];
                if cell.collapsed {
                    let tile_index = cell.options[0];
                    let tile_type = &tileset[tile_index].tile_type;

                    // Allow only certain tile types to be converted into Exit
                    if matches!(
                        tile_type,
                        TileType::Grass | TileType::Sand | TileType::Stone
                    ) {
                        let exit_index = tileset
                            .iter()
                            .position(|t| matches!(t.tile_type, TileType::Exit(_)))
                            .expect("No Exit tile in tileset");

                        cell.options = vec![exit_index];
                        exit_placed = true;
                    }
                }
            }

            if success {
                return WFCState { grid, tileset };
            } else {
                eprintln!("Contradiction detected. Retrying WFC...");
            }
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        for y in 0..GRID_HEIGHT as usize {
            for x in 0..GRID_WIDTH as usize {
                let cell = &self.grid[y][x];
                if cell.collapsed {
                    let tile_index = cell.options[0];
                    let tile = &self.tileset[tile_index];
                    let rect = Rect::new(
                        (x as usize * TILE_SIZE) as i32,
                        (y as usize * TILE_SIZE) as i32,
                        TILE_SIZE as u32,
                        TILE_SIZE as u32,
                    );
                    canvas.set_draw_color(tile.tile_type._get_color());
                    canvas.fill_rect(rect).unwrap();
                }
            }
        }
    }
    pub fn to_level(
        &self,
        texture_creator: &'a TextureCreator<sdl2::video::WindowContext>,
        texture_map: &mut HashMap<String, Texture<'a>>,
    ) -> Level {
        let mut layer = HashMap::new();

        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let tile_index = cell.options[0];
                let tile_type = &self.tileset[tile_index].tile_type;
                let mut tile = Tile::new(
                    x as i32 * TILE_SIZE as i32,
                    y as i32 * TILE_SIZE as i32,
                    TILE_SIZE as u32,
                    tile_type.clone(),
                    None, //fix it here!!
                );
                // Assign texture based on tile type
                let texture_path = match tile_type {
                    TileType::Stone => "resources/textures/tile.png",
                    TileType::Water => "resources/textures/water.png",
                    TileType::Grass => "resources/textures/grass.png",
                    TileType::Sand => "resources/textures/sand.png",
                    TileType::Rock => "resources/textures/rock.png",
                    TileType::Tree => "resources/textures/tree.png",
                    TileType::Wall => "resources/textures/wall.png",
                    TileType::Inventory => "resources/textures/cogwheel.png",
                    TileType::Exit(_) => "resources/textures/exit.png",
                    _ => continue, // Skip unknown or unsupported types
                };

                let mut texture_data = TextureData::new(texture_path.to_string());
                texture_data.load_texture(texture_creator, texture_map);
                tile.texture_data = Some(texture_data);
                layer.insert(
                    Point::new(x as i32 * TILE_SIZE as i32, y as i32 * TILE_SIZE as i32),
                    tile,
                );
            }
        }
        let tiles = vec![layer]; //single layer for now
        Level {
            tiles,
            player_spawn: rand_player_spawn(&self),
            tile_size: TILE_SIZE as i32,
            autotiler: Autotiler::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub options: Vec<usize>,
    pub collapsed: bool,
}

impl Cell {
    fn new(tile_count: usize) -> Self {
        Cell {
            options: (0..tile_count).collect(),
            collapsed: false,
        }
    }

    fn entropy(&self) -> usize {
        self.options.len()
    }

    fn collapse(&mut self) {
        let mut rng = rng();
        if self.collapsed || self.options.is_empty() {
            return;
        }
        let choice = *self
            .options
            .choose(&mut rng)
            .expect("No options to collapse");
        self.options = vec![choice];
        self.collapsed = true;
    }
}

pub fn rand_player_spawn(wfc: &WFCState) -> (i32, i32) {
    let mut rng = rng();
    let mut spawn_x = 0;
    let mut spawn_y = 0;

    loop {
        spawn_x = rng.random_range(0..GRID_WIDTH as i32);
        spawn_y = rng.random_range(0..GRID_HEIGHT as i32);

        let cell = &wfc.grid[spawn_y as usize][spawn_x as usize];
        if cell.collapsed
            && matches!(
                wfc.tileset[cell.options[0]].tile_type,
                TileType::Grass | TileType::Sand | TileType::Stone
            )
        {
            break;
        }
    }
    println!("Player spawn at: ({}, {})", spawn_x, spawn_y);
    (spawn_x * TILE_SIZE as i32, spawn_y * TILE_SIZE as i32)
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

fn is_compatible(tile_a: &WfcTile, tile_b: &WfcTile, direction: usize) -> bool {
    let opposite = (direction + 2) % 4;
    tile_a.edges[direction]
        .iter()
        .any(|edge_a| tile_b.edges[opposite].contains(edge_a))
}

fn propagate(grid: &mut Vec<Vec<Cell>>, tileset: &[WfcTile]) -> bool {
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut changed = true;

    while changed {
        changed = false;
        let mut updates = vec![];

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let cell = &grid[y as usize][x as usize];
                if cell.collapsed {
                    continue;
                }

                let mut new_options = cell.options.clone();

                for (dir, (dx, dy)) in directions.iter().enumerate() {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx < 0 || ny < 0 || nx >= GRID_WIDTH as isize || ny >= GRID_HEIGHT as isize {
                        continue;
                    }

                    let neighbor = &grid[ny as usize][nx as usize];

                    new_options.retain(|&opt| {
                        neighbor
                            .options
                            .iter()
                            .any(|&nopt| is_compatible(&tileset[opt], &tileset[nopt], dir))
                    });
                }

                if new_options.is_empty() {
                    eprintln!("Contradiction at ({}, {})", x, y);
                    return false;
                }

                if new_options.len() < cell.options.len() {
                    updates.push((x, y, new_options));
                }
            }
        }

        if !updates.is_empty() {
            changed = true;
            for (x, y, new_options) in updates {
                grid[y as usize][x as usize].options = new_options;
            }
        }
    }

    true
}

pub fn run_wfc() {
    // Example usage
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("wfc prev", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window
        .into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();

    let tileset = vec![
        WfcTile {
            tile_type: TileType::Water,
            edges: [
                vec![TileType::Water, TileType::Grass, TileType::Sand],
                vec![TileType::Water, TileType::Grass, TileType::Sand],
                vec![TileType::Water, TileType::Grass, TileType::Sand],
                vec![TileType::Water, TileType::Grass, TileType::Sand],
            ],
        },
        WfcTile {
            tile_type: TileType::Grass,
            edges: [
                vec![TileType::Grass, TileType::Water],
                vec![TileType::Grass, TileType::Water],
                vec![TileType::Grass, TileType::Water],
                vec![TileType::Grass, TileType::Water],
            ],
        },
        WfcTile {
            tile_type: TileType::Sand,
            edges: [
                vec![TileType::Sand, TileType::Water],
                vec![TileType::Sand, TileType::Water],
                vec![TileType::Sand, TileType::Water],
                vec![TileType::Sand, TileType::Water],
            ],
        },
        WfcTile {
            tile_type: TileType::Exit(ExitTile {
                next_level: "level1_2".to_string(),
            }),
            edges: [
                vec![TileType::Grass, TileType::Water],
                vec![TileType::Grass, TileType::Water],
                vec![TileType::Grass, TileType::Water],
                vec![TileType::Grass, TileType::Water],
            ],
        },
    ];
    let mut wfc_state = WFCState::new(&tileset);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            match event {
                Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::N),
                    ..
                } => {
                    // Press 'N' to generate a new WFC state
                    println!("Generating new WFCstate...");
                    let new_wfc_state = WFCState::new(&tileset);
                    wfc_state.grid = new_wfc_state.grid;
                    wfc_state.tileset = new_wfc_state.tileset;
                }
                Event::Quit { .. } => {
                    break 'running;
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        wfc_state.draw(&mut canvas);
        canvas.present();
    }
}
