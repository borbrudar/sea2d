use std::{collections::HashMap, io::BufRead};

use ::image::RgbaImage;
use sdl2::render::{Texture, TextureCreator};

use crate::{
    environment::{
        aabb::AABB,
        texture_data::TextureData,
        tile::Tile,
        tile_type::{ExitTile, TileType},
    },
    entities::{camera::Camera, point::Point},
};

pub struct Level {
    pub tiles: Vec<HashMap<Point<i32>, Tile>>, // vector for each layer, hashmap for fast position queries
    pub player_spawn: (i32, i32),
    pub tile_size: i32,
}

impl<'a> Level {
    pub fn new() -> Level {
        Level {
            tiles: Vec::new(),
            player_spawn: (0, 0),
            tile_size: 50,
        }
    }

    pub fn load_from_file(
        &mut self,
        path: String,
        texture_creator: &'a TextureCreator<sdl2::video::WindowContext>,
        texture_map: &mut HashMap<String, Texture<'a>>,
    ) {
        // delete previous level (if any)
        self.tiles.clear();

        // load exits file
        let mut exits = path.clone();
        exits = exits.chars().take(exits.chars().count() - 5).collect();
        exits.push_str(String::from("exits.txt").as_str());
        if !::std::path::Path::new(&exits).exists() {
            panic!("Exits file not found");
        }
        let exits = ::std::fs::File::open(exits).expect("Failed to read exits file");
        let exits = ::std::io::BufReader::new(exits);
        let mut exits: Vec<String> = ::std::io::BufReader::new(exits)
            .lines()
            .filter_map(Result::ok)
            .collect();
        exits.reverse();

        // load layer by layer from file, change path for each layer from "layer1_1.png" to "layer_2.png" while you can
        self.load_layer(path.clone(), texture_creator, texture_map, &mut exits);

        let mut i = 2;
        loop {
            let mut new_path = path.clone();
            new_path = new_path
                .chars()
                .take(new_path.chars().count() - 5)
                .collect();
            new_path.push_str(String::from(format!("{}.png", i)).as_str());
            if !::std::path::Path::new(&new_path).exists() {
                break;
            }
            self.load_layer(new_path, texture_creator, texture_map, &mut exits);
            i += 1;
        }
    }

    fn load_layer(
        &mut self,
        path: String,
        texture_creator: &'a TextureCreator<sdl2::video::WindowContext>,
        texture_map: &mut HashMap<String, Texture<'a>>,
        exits: &mut Vec<String>,
    ) {
        let img = ::image::ImageReader::open(path)
            .expect("Failed to load image")
            .decode()
            .expect("Failed to decode image");
        let img: RgbaImage = img.to_rgba8();
        let (width, height) = (img.dimensions().0 as i32, img.dimensions().1 as i32);

        let mut layer: HashMap<Point<i32>, Tile> = HashMap::new();

        for y in 0..height {
            for x in 0..width {
                let pixel = img.get_pixel(x as u32, y as u32);

                let mut default_bounding_box = None;
                if pixel[3] >= 128 {
                    default_bounding_box = Some(AABB::new(
                        (x * self.tile_size) as f64,
                        (y * self.tile_size) as f64,
                        self.tile_size as u32,
                        self.tile_size as u32,
                    ));
                }
                let pixel = (pixel[0], pixel[1], pixel[2]);
                //println!("Pixel: {:?}",pixel);
                let pos: Point<i32> =
                    Point::new((x * self.tile_size) as i32, (y * self.tile_size) as i32);

                match pixel {
                    TileType::STONE_COLOR => {
                        layer.insert(
                            pos,
                            Tile::new(
                                pos.x,
                                pos.y,
                                self.tile_size as u32,
                                TileType::Stone,
                                default_bounding_box,
                            ),
                        );
                        layer.get_mut(&pos).unwrap().texture_data =
                            Some(TextureData::new("resources/textures/tile.png".to_string()));
                        layer
                            .get_mut(&pos)
                            .unwrap()
                            .texture_data
                            .as_mut()
                            .unwrap()
                            .load_texture(&texture_creator, texture_map);
                    }
                    TileType::WATER_COLOR => {
                        layer.insert(
                            pos,
                            Tile::new(
                                pos.x,
                                pos.y,
                                self.tile_size as u32,
                                TileType::Water,
                                default_bounding_box,
                            ),
                        );
                        layer.get_mut(&pos).unwrap().texture_data =
                            Some(TextureData::new("resources/textures/water.png".to_string()));
                        layer
                            .get_mut(&pos)
                            .unwrap()
                            .texture_data
                            .as_mut()
                            .unwrap()
                            .load_texture(&texture_creator, texture_map);
                    }
                    TileType::GRASS_COLOR => {
                        layer.insert(
                            pos,
                            Tile::new(
                                pos.x,
                                pos.y,
                                self.tile_size as u32,
                                TileType::Grass,
                                default_bounding_box,
                            ),
                        );
                        layer.get_mut(&pos).unwrap().texture_data =
                            Some(TextureData::new("resources/textures/grass.png".to_string()));
                        layer
                            .get_mut(&pos)
                            .unwrap()
                            .texture_data
                            .as_mut()
                            .unwrap()
                            .load_texture(&texture_creator, texture_map);
                    }
                    TileType::SAND_COLOR => {
                        layer.insert(
                            pos,
                            Tile::new(
                                pos.x,
                                pos.y,
                                self.tile_size as u32,
                                TileType::Sand,
                                default_bounding_box,
                            ),
                        );
                        layer.get_mut(&pos).unwrap().texture_data =
                            Some(TextureData::new("resources/textures/sand.png".to_string()));
                        layer
                            .get_mut(&pos)
                            .unwrap()
                            .texture_data
                            .as_mut()
                            .unwrap()
                            .load_texture(&texture_creator, texture_map);
                    }
                    TileType::ROCK_COLOR => {
                        layer.insert(
                            pos,
                            Tile::new(
                                pos.x,
                                pos.y,
                                self.tile_size as u32,
                                TileType::Rock,
                                default_bounding_box,
                            ),
                        );
                        layer.get_mut(&pos).unwrap().texture_data =
                            Some(TextureData::new("resources/textures/rock.png".to_string()));
                        layer
                            .get_mut(&pos)
                            .unwrap()
                            .texture_data
                            .as_mut()
                            .unwrap()
                            .load_texture(&texture_creator, texture_map);
                    }
                    TileType::TREE_COLOR => {
                        layer.insert(
                            pos,
                            Tile::new(
                                pos.x,
                                pos.y,
                                self.tile_size as u32,
                                TileType::Tree,
                                default_bounding_box,
                            ),
                        );
                        layer.get_mut(&pos).unwrap().texture_data =
                            Some(TextureData::new("resources/textures/tree.png".to_string()));
                        layer
                            .get_mut(&pos)
                            .unwrap()
                            .texture_data
                            .as_mut()
                            .unwrap()
                            .load_texture(&texture_creator, texture_map);
                    }
                    TileType::WALL_COLOR => {
                        layer.insert(
                            pos,
                            Tile::new(
                                pos.x,
                                pos.y,
                                self.tile_size as u32,
                                TileType::Wall,
                                default_bounding_box,
                            ),
                        );
                        layer.get_mut(&pos).unwrap().texture_data =
                            Some(TextureData::new("resources/textures/wall.png".to_string()));
                        layer
                            .get_mut(&pos)
                            .unwrap()
                            .texture_data
                            .as_mut()
                            .unwrap()
                            .load_texture(&texture_creator, texture_map);
                    }
                    TileType::PLAYER_SPAWN_COLOR => {
                        self.player_spawn = (x * self.tile_size, y * self.tile_size)
                    }
                    TileType::EXIT_COLOR => {
                        let last = exits.pop().unwrap();
                        let exit_bb = Some(AABB::new(
                            (x * self.tile_size + self.tile_size / 4) as f64,
                            (y * self.tile_size + self.tile_size / 4) as f64,
                            self.tile_size as u32 / 2,
                            self.tile_size as u32 / 2,
                        ));
                        layer.insert(
                            pos,
                            Tile::new(
                                pos.x,
                                pos.y,
                                self.tile_size as u32,
                                TileType::Exit(ExitTile { next_level: last }),
                                exit_bb,
                            ),
                        );
                    }
                    TileType::INVENTORY_COLOR => {
                        layer.insert(
                            pos,
                            Tile::new(
                                pos.x,
                                pos.y,
                                self.tile_size as u32,
                                TileType::Inventory,
                                default_bounding_box,
                            ),
                        );
                        layer.get_mut(&pos).unwrap().texture_data = Some(TextureData::new(
                            "resources/textures/cogwheel.png".to_string(),
                        ));
                        layer
                            .get_mut(&pos)
                            .unwrap()
                            .texture_data
                            .as_mut()
                            .unwrap()
                            .load_texture(&texture_creator, texture_map);
                    }
                    _ => (),
                }
            }
        }

        self.tiles.push(layer);
    }

    pub fn draw(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        texture_map: &std::collections::HashMap<String, sdl2::render::Texture>,
        camera: &Camera,
    ) {
        for layer in &self.tiles {
            for (_, tile) in layer {
                tile.draw(canvas, texture_map, camera);
            }
        }
    }

    pub fn draw_hitboxes(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        camera: &Camera,
    ) {
        for layer in &self.tiles {
            for (_, tile) in layer {
                match tile.bounding_box {
                    Some(ref bounding_box) => {
                        bounding_box.draw(canvas, sdl2::pixels::Color::RGB(255, 0, 0), camera);
                    }
                    None => (),
                }
            }
        }
    }

    // snap to nearest tile
    pub fn get_snapped_position(&self, hitbox: &AABB) -> (i32, i32) {
        let x = (hitbox.x + hitbox.w as f64 / 2.0) as i32;
        let y = (hitbox.y + hitbox.h as f64 / 2.0) as i32;
        (
            x / self.tile_size * self.tile_size,
            y / self.tile_size * self.tile_size,
        )
    }

    pub fn check_collision(&self, hitbox: &AABB) -> Vec<Tile> {
        let mut ret = Vec::new();
        let player_tile = (
            self.get_snapped_position(hitbox).0,
            self.get_snapped_position(hitbox).1,
        );
        //println!("Player tile: {:?}",player_tile);
        // check 9 neighbouring tiles
        for offx in -1..2 {
            for offy in -1..2 {
                let offset_pos = Point::new(
                    player_tile.0.wrapping_add(offx * self.tile_size),
                    player_tile.1.wrapping_add(offy * self.tile_size),
                );

                for layer in &self.tiles {
                    match layer.get(&offset_pos) {
                        Some(tile) => match tile.bounding_box {
                            Some(ref bounding_box) => {
                                if hitbox.intersects(bounding_box) {
                                    ret.push(tile.clone());
                                }
                            }
                            None => (),
                        },
                        None => (),
                    }
                }
            }
        }
        ret
    }
    pub fn resolve_collision(&self, hitbox: &mut AABB) {
        let player_tile = (
            self.get_snapped_position(hitbox).0,
            self.get_snapped_position(hitbox).1,
        );
        for offx in -1..2 {
            for offy in -1..2 {
                let offset_pos = Point::new(
                    player_tile.0.wrapping_add(offx * self.tile_size),
                    player_tile.1.wrapping_add(offy * self.tile_size),
                );

                for layer in &self.tiles {
                    match layer.get(&offset_pos) {
                        Some(tile) => {
                            match tile.bounding_box {
                                Some(ref bounding_box) => {
                                    if hitbox.intersects(bounding_box) {
                                        let x1 = hitbox.x + hitbox.w as f64 - bounding_box.x; // right side of player - left side of tile
                                        let x2 = bounding_box.x + bounding_box.w as f64 - hitbox.x; // right side of tile - left side of player
                                        let y1 = hitbox.y + hitbox.h as f64 - bounding_box.y; // bottom side of player - top side of tile
                                        let y2 = bounding_box.y + bounding_box.h as f64 - hitbox.y; // bottom side of tile - top side of player
                                        let min = x1.min(x2).min(y1).min(y2);
                                        if min == x1 {
                                            hitbox.x -= x1;
                                        } else if min == x2 {
                                            hitbox.x += x2;
                                        } else if min == y1 {
                                            hitbox.y -= y1;
                                        } else if min == y2 {
                                            hitbox.y += y2;
                                        }
                                    }
                                }
                                None => (),
                            }
                        }
                        None => (),
                    }
                }
            }
        }
    }
}
