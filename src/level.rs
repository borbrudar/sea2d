use std::{collections::HashMap, io::BufRead};

use ::image::RgbaImage;
use sdl2::render::{Texture, TextureCreator};

use crate::{aabb::AABB, camera::Camera, texture_data::TextureData, tile::Tile, tile_type::{ExitTile, TileType}};

pub struct Level{
    pub tiles : Vec<Vec<Tile>>,
    pub player_spawn : (i32,i32),
}

impl<'a> Level{
    pub fn new() -> Level{
        Level{
            tiles : Vec::new(),
            player_spawn : (0,0)
        }
    }

    pub fn load_from_file(&mut self, path : String, texture_creator : & 'a TextureCreator<sdl2::video::WindowContext>, texture_map : &mut HashMap<String,Texture<'a>>){
        // delete previous level (if any)
        self.tiles.clear();
        
        // load exits file 
        let mut exits = path.clone();
        exits = exits.chars().take(exits.chars().count()-5).collect();
        exits.push_str(String::from("exits.txt").as_str());
        if !::std::path::Path::new(&exits).exists(){
            panic!("Exits file not found");
        }
        let exits = ::std::fs::File::open(exits).expect("Failed to read exits file");
        let exits = ::std::io::BufReader::new(exits);
        let mut exits : Vec<String> = ::std::io::BufReader::new(exits).lines().filter_map(Result::ok).collect();
        exits.reverse();

        // load layer by layer from file, change path for each layer from "layer1_1.png" to "layer_2.png" while you can
        self.load_layer(path.clone(),texture_creator,texture_map,&mut exits);

        let mut i = 2;
        loop{
            let mut new_path = path.clone();
            new_path = new_path.chars().take(new_path.chars().count()-5).collect();
            new_path.push_str(String::from(format!("{}.png",i)).as_str());
            if !::std::path::Path::new(&new_path).exists(){
                break;
            }
            self.load_layer(new_path,texture_creator,texture_map,&mut exits);
            i += 1;
        }
    }

    fn load_layer(&mut self, path : String, texture_creator : & 'a TextureCreator<sdl2::video::WindowContext>, texture_map : &mut HashMap<String,Texture<'a>>, exits : &mut Vec<String>){
        let img = ::image::ImageReader::open(path).expect("Failed to load image").decode().expect("Failed to decode image");
        let img: RgbaImage = img.to_rgba8();
        let (width, height) = img.dimensions();
        
        let mut layer = Vec::new();

        let tile_size  = 50;
        for y in 0..height{
            for x in 0..width{
                let pixel = img.get_pixel(x, y);
                let pixel = (pixel[0],pixel[1],pixel[2]);


                //println!("Pixel: {:?}",pixel);
                match pixel{
                    TileType::STONE_COLOR => {
                        layer.push(Tile::new(x as i32 * tile_size, y as i32 * tile_size, tile_size as u32, TileType::Stone, None));
                        layer.last_mut().unwrap().texture_data = Some(TextureData::new("resources/textures/tile.png".to_string()));
                        layer.last_mut().unwrap().texture_data.as_mut().unwrap().load_texture(&texture_creator, texture_map, );
                    },
                    TileType::WATER_COLOR => {
                        layer.push(Tile::new(x as i32 * tile_size, y as i32 * tile_size, tile_size as u32, TileType::Water, Some(AABB::new(x as i32 * tile_size, y as i32 * tile_size, tile_size as u32, tile_size as u32))));
                        layer.last_mut().unwrap().texture_data = Some(TextureData::new("resources/textures/water.png".to_string()));
                        layer.last_mut().unwrap().texture_data.as_mut().unwrap().load_texture(&texture_creator, texture_map);
                    },
                    TileType::GRASS_COLOR => {
                        layer.push(Tile::new(x as i32 * tile_size, y as i32 * tile_size, tile_size as u32, TileType::Grass, None));
                        layer.last_mut().unwrap().texture_data = Some(TextureData::new("resources/textures/grass.png".to_string()));
                        layer.last_mut().unwrap().texture_data.as_mut().unwrap().load_texture(&texture_creator, texture_map);
                    },
                    TileType::SAND_COLOR => {
                        layer.push(Tile::new(x as i32 * tile_size, y as i32 * tile_size, tile_size as u32, TileType::Sand, None));
                        layer.last_mut().unwrap().texture_data = Some(TextureData::new("resources/textures/sand.png".to_string()));
                        layer.last_mut().unwrap().texture_data.as_mut().unwrap().load_texture(&texture_creator, texture_map);
                    },
                    TileType::ROCK_COLOR => {
                        layer.push(Tile::new(x as i32 * tile_size, y as i32 * tile_size, tile_size as u32, TileType::Rock, Some(AABB::new(x as i32 * tile_size, y as i32 * tile_size, tile_size as u32, tile_size as u32))));
                        layer.last_mut().unwrap().texture_data = Some(TextureData::new("resources/textures/rock.png".to_string()));
                        layer.last_mut().unwrap().texture_data.as_mut().unwrap().load_texture(&texture_creator, texture_map);
                    },
                    TileType::TREE_COLOR => {
                        layer.push(Tile::new(x as i32 * tile_size, y as i32 * tile_size, tile_size as u32, TileType::Tree, Some(AABB::new(x as i32 * tile_size, y as i32 * tile_size, tile_size as u32, tile_size as u32))));
                        layer.last_mut().unwrap().texture_data = Some(TextureData::new("resources/textures/tree.png".to_string()));
                        layer.last_mut().unwrap().texture_data.as_mut().unwrap().load_texture(&texture_creator, texture_map);
                    },
                    TileType::WALL_COLOR => {
                        layer.push(Tile::new(x as i32 * tile_size, y as i32 * tile_size, tile_size as u32, TileType::Wall, Some(AABB::new(x as i32 * tile_size, y as i32 * tile_size, tile_size as u32, tile_size as u32))));
                        layer.last_mut().unwrap().texture_data = Some(TextureData::new("resources/textures/wall.png".to_string()));
                        layer.last_mut().unwrap().texture_data.as_mut().unwrap().load_texture(&texture_creator, texture_map);
                    },
                    TileType::PLAYER_SPAWN_COLOR => {
                        self.player_spawn = (x as i32 * tile_size, y as i32 * tile_size);
                    }
                    TileType::EXIT_COLOR => {
                        let last = exits.pop().unwrap();
                        layer.push(Tile::new(x as i32 * tile_size, y as i32 * tile_size, tile_size as u32, TileType::Exit(ExitTile{next_level : last}), Some(AABB::new(x as i32 * tile_size + tile_size/4, y as i32 * tile_size+tile_size/4, tile_size as u32/2, tile_size as u32/2))));
                        //layer.last_mut().unwrap().texture_data = Some(TextureData::new("resources/textures/empty.png".to_string()));
                        //layer.last_mut().unwrap().texture_data.as_mut().unwrap().load_texture(&texture_creator, texture_map);
                    },
                    _ => ()
                }
            }
        }
    
        self.tiles.push(layer);
    }


    pub fn draw(&self,canvas : &mut sdl2::render::Canvas<sdl2::video::Window>, texture_map : &std::collections::HashMap<String,sdl2::render::Texture>, camera : &Camera){
        for layer in &self.tiles{
            for tile in layer{
                tile.draw(canvas,texture_map,camera);
            }
        }
    }

    pub fn draw_hitboxes(&self,canvas : &mut sdl2::render::Canvas<sdl2::video::Window>, camera : &Camera){
        for layer in &self.tiles{
            for tile in layer{
                match tile.bounding_box{
                    Some(ref bounding_box) => {
                        bounding_box.draw(canvas,sdl2::pixels::Color::RGB(255,0,0),camera);
                    },
                    None => ()
                }
            }
        }
    }
}
