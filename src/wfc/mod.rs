use crate::game::find_sdl_gl_driver;
use crate::level::tile_type::TileType;
use crate::networking::shared::{SCREEN_HEIGHT, SCREEN_WIDTH};
use rand::{prelude::IndexedRandom, rng, seq::SliceRandom};
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

const TILE_SIZE: usize = 32;
const GRID_WIDTH: usize = SCREEN_WIDTH as usize / TILE_SIZE;
const GRID_HEIGHT: usize = SCREEN_HEIGHT as usize / TILE_SIZE;

#[derive(Debug, Clone)]
pub struct WfcTile {
    tile_type: TileType,
    edges: [Vec<TileType>; 4], // Up, Right, Down, Left
}

pub struct WFCState {
    grid: Vec<Vec<Cell>>,
    tileset: Vec<WfcTile>,
}

impl WFCState {
    pub fn new(tileset: Vec<WfcTile>) -> Self {
        let mut grid =
            vec![vec![Cell::new(tileset.len()); GRID_WIDTH as usize]; GRID_HEIGHT as usize];

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

            let (x, y) = uncollapsed[0];

            grid[y][x].collapse();

            propagate(&mut grid, &tileset);
        }
        WFCState { grid, tileset }
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
}

#[derive(Debug, Clone)]
struct Cell {
    options: Vec<usize>,
    collapsed: bool,
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

fn is_compatible(tile_a: &WfcTile, tile_b: &WfcTile, direction: usize) -> bool {
    let opposite = (direction + 2) % 4;
    tile_a.edges[direction]
        .iter()
        .any(|edge_a| tile_b.edges[opposite].contains(edge_a))
}

fn propagate(grid: &mut Vec<Vec<Cell>>, tileset: &[WfcTile]) {
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

                if new_options.len() < cell.options.len() {
                    updates.push((x, y, new_options));
                }
            }
        }

        if !updates.is_empty() {
            changed = true;
            for (x, y, new_options) in updates {
                println!(
                    "Propagated update at ({}, {}): options now {:?}",
                    x, y, new_options
                );
                grid[y as usize][x as usize].options = new_options;
            }
        }
    }
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
                vec![TileType::Water, TileType::Grass],
                vec![TileType::Water, TileType::Grass],
                vec![TileType::Water, TileType::Grass],
                vec![TileType::Water, TileType::Grass],
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
    ];
    let mut wfc_state = WFCState::new(tileset);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            match event {
                //         Event::KeyDown {
                //             keycode: Some(sdl2::keyboard::Keycode::N),
                //             ..
                //         } => {
                //             // Press 'N' to generate a new WFC state
                //             println!("Generating new WFC state...");
                //             let new_wfc_state = WFCState::new(tileset.clone());
                //             wfc_state.grid = new_wfc_state.grid;
                //             wfc_state.tileset = new_wfc_state.tileset;
                //         }
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
