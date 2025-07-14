use sdl2::{
    render::{Canvas, Texture},
    video::Window,
};

use crate::{
    environment::{level::Level, aabb::AABB},
    entities::{player::Player, animated_texture::AnimatedTexture, camera::Camera, point::Point},
};
use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
};

pub struct Enemy {
    pub x: f64,
    pub y: f64,
    pub animation_data: Option<AnimatedTexture>,
    pub size: u32,
    pub hitbox: AABB,

    pub last_time: f64,
    pub dir: i32,
}

impl Enemy {
    pub fn new() -> Enemy {
        Enemy {
            x: 50.,
            y: 50.,
            animation_data: None,
            size: 50,
            hitbox: AABB::new(55., 55., 40, 40),
            last_time: 0.,
            dir: -1,
        }
    }

    pub fn draw(
        &self,
        canvas: &mut Canvas<Window>,
        texture_map: &std::collections::HashMap<String, Texture>,
        camera: &Camera,
    ) {
        match self.animation_data {
            Some(ref animation_data) => {
                animation_data.draw(
                    canvas,
                    texture_map,
                    self.x - camera.x,
                    self.y - camera.y,
                    self.size,
                    self.size,
                );
            }
            None => {
                canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 192, 203));
                canvas
                    .fill_rect(sdl2::rect::Rect::new(
                        (self.x - camera.x) as i32,
                        (self.y - camera.y) as i32,
                        self.size,
                        self.size,
                    ))
                    .unwrap();
            }
        }
    }

    pub fn update(&mut self, dt: f64, level: &Level, player: &Player, instant: &Instant) {
        match self.animation_data {
            Some(ref mut animation_data) => {
                animation_data.update(dt);
            }
            None => (),
        };

        //println!("TIme: {}",instant.elapsed().as_secs_f64());
        if instant.elapsed().as_secs_f64() - self.last_time > 0.5 {
            //println!("Enemy moving");
            self.dir = self.calculate_player_direction(level, player);
            self.last_time = instant.elapsed().as_secs_f64();
        }
        match self.dir {
            0 => self.y -= 2. * level.tile_size as f64 * dt,
            1 => self.x += 2. * level.tile_size as f64 * dt,
            2 => self.y += 2. * level.tile_size as f64 * dt,
            3 => self.x -= 2. * level.tile_size as f64 * dt,
            _ => (),
        }
        self.hitbox.x = self.x + 5.;
        self.hitbox.y = self.y + 5.;
    }

    pub fn calculate_player_direction(&self, level: &Level, player: &Player) -> i32 {
        let player_tile = Point::new(
            level.get_snapped_position(&player.hitbox).0,
            level.get_snapped_position(&player.hitbox).1,
        );
        let enemy_tile = Point::new(
            level.get_snapped_position(&self.hitbox).0,
            level.get_snapped_position(&self.hitbox).1,
        );

        // run a bfs to find the shortest path to the player and return the direction
        let mut queue = VecDeque::new();
        let mut distance = HashMap::new();
        queue.push_back((enemy_tile, Point::new(-1, -1), -1));
        // tile, (distance, direction) - 0 up, 1 right, 2 down, 3 left
        distance.insert(Point::new(-1, -1), (-1, -1));

        while let Some(current) = queue.pop_front() {
            let (current, prev, dir) = current;
            if distance.contains_key(&current) {
                continue;
            }

            distance.insert(current, (distance[&prev].0 + 1, dir));
            if current == player_tile {
                break;
            }

            let (x, y) = (current.x, current.y);
            let mut next = Point::new(x + level.tile_size, y);

            let check = |next| -> bool {
                let mut exists = false;
                let mut obstacle = false;

                for i in 0..level.tiles.len() {
                    if level.tiles[i].contains_key(&next) {
                        exists = true;
                        if level.tiles[i][&next].bounding_box.is_some() {
                            obstacle = true;
                        }
                    }
                }
                exists && !obstacle
            };

            if check(next) {
                queue.push_back((next, current, 1));
            }
            next = Point::new(x - level.tile_size, y);
            if check(next) {
                queue.push_back((next, current, 3));
            }
            next = Point::new(x, y + level.tile_size);
            if check(next) {
                queue.push_back((next, current, 2));
            }
            next = Point::new(x, y - level.tile_size);
            if check(next) {
                queue.push_back((next, current, 0));
            }
        }

        // path reconstruction
        let mut current = player_tile;
        let mut dir = -1;
        while current != enemy_tile {
            if !distance.contains_key(&current) {
                return -1;
            }
            dir = distance[&current].1;
            match dir {
                0 => current = Point::new(current.x, current.y + level.tile_size),
                1 => current = Point::new(current.x - level.tile_size, current.y),
                2 => current = Point::new(current.x, current.y - level.tile_size),
                3 => current = Point::new(current.x + level.tile_size, current.y),
                _ => (),
            }
        }
        dir
    }
}
