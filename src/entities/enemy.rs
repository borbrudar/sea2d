use sdl2::{
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};

use crate::{
    entities::{animated_texture::{AnimatedTexture, AnimationType}, animation_data::{AnimationData, AnimationState}, camera::Camera, player::Player, point::Point}, environment::{aabb::AABB, level::Level}
};
use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
};

pub enum EnemyType {
    Slime,
    Stonewalker,
    Wizard,
    Skull,
    Placeholder,
}

pub struct Enemy {
    pub x: f64,
    pub y: f64,
    pub animation_data: Option<AnimationData>,
    pub size_x: u32,
    pub size_y: u32,
    pub hitbox: AABB,
    pub kind : EnemyType,

    pub last_time: f64,
    pub dir: i32,
}

impl Enemy {
    pub fn new<'a>(kind : EnemyType, texture_creator : &'a TextureCreator<WindowContext>, texture_map: &mut std::collections::HashMap<String, Texture<'a>>) -> Enemy {
        let mut ani_data : Option<AnimationData> = None;
        let mut size_x = 50;
        let mut size_y = 50;

        match kind {
            EnemyType::Slime => {
                ani_data = Some(AnimationData::new());
                ani_data.as_mut().unwrap().front = Some(AnimatedTexture::new(1.0/5.0));
                ani_data.as_mut().unwrap().front.as_mut().unwrap().load_animation(
                    "resources/enemies/slime.png".to_string(), 0, 0, 16, 16, 3, texture_creator, texture_map);
                ani_data.as_mut().unwrap().current_animation = AnimationState::Front;
                ani_data.as_mut().unwrap().front.as_mut().unwrap().animation_type = AnimationType::PingPong
            }
            EnemyType::Stonewalker => {
                ani_data = Some(AnimationData::new());
                ani_data.as_mut().unwrap().front = Some(AnimatedTexture::new(1.0/10.0));
                ani_data.as_mut().unwrap().front.as_mut().unwrap().load_animation(
                    "resources/enemies/stonewalker.png".to_string(), 0, 0, 16, 16, 4, texture_creator, texture_map);
                ani_data.as_mut().unwrap().current_animation = AnimationState::Front;

                ani_data.as_mut().unwrap().default = Some(AnimatedTexture::new(1.0/10.0));
                ani_data.as_mut().unwrap().default.as_mut().unwrap().load_animation(
                    "resources/enemies/stonewalker.png".to_string(), 0, 16, 16, 16, 1, texture_creator, texture_map);
            }
            EnemyType::Wizard => {
                size_x = 64; size_y = 64*2;
                ani_data = Some(AnimationData::new());
                ani_data.as_mut().unwrap().current_animation = AnimationState::Default;
                ani_data.as_mut().unwrap().idle = Some(AnimatedTexture::new(1.0));
                ani_data.as_mut().unwrap().idle.as_mut().unwrap().load_animation(
                    "resources/enemies/wizard.png".to_string(), 0, 0, 32, 64, 1, texture_creator, texture_map);

                ani_data.as_mut().unwrap().front = Some(AnimatedTexture::new(1.0/5.0));
                ani_data.as_mut().unwrap().front.as_mut().unwrap().load_animation(
                    "resources/enemies/wizard.png".to_string(), 0, 0, 32, 64, 6, texture_creator, texture_map);

                ani_data.as_mut().unwrap().right = Some(AnimatedTexture::new(1.0/5.0));
                ani_data.as_mut().unwrap().right.as_mut().unwrap().load_animation(
                    "resources/enemies/wizard.png".to_string(), 0, 64, 32, 64, 6, texture_creator, texture_map);

                ani_data.as_mut().unwrap().left = Some(AnimatedTexture::new(1.0/5.0));
                ani_data.as_mut().unwrap().left.as_mut().unwrap().load_animation(
                    "resources/enemies/wizard.png".to_string(), 0, 128, 32, 64, 6, texture_creator, texture_map);

                ani_data.as_mut().unwrap().back = Some(AnimatedTexture::new(1.0/5.0));
                ani_data.as_mut().unwrap().back.as_mut().unwrap().load_animation(
                    "resources/enemies/wizard.png".to_string(), 0, 64*3, 32, 64, 6, texture_creator, texture_map);
                
                ani_data.as_mut().unwrap().default = Some(AnimatedTexture::new(1.0));
                ani_data.as_mut().unwrap().default.as_mut().unwrap().load_animation(
                    "resources/enemies/wizard.png".to_string(), 0, 0, 32, 64, 1, texture_creator, texture_map);
            }
            EnemyType::Skull => {
                size_x = 32*2; size_y = 32*2;
                ani_data = Some(AnimationData::new());
                ani_data.as_mut().unwrap().current_animation = AnimationState::Default;
                ani_data.as_mut().unwrap().idle = Some(AnimatedTexture::new(1.0));
                ani_data.as_mut().unwrap().idle.as_mut().unwrap().load_animation(
                    "resources/enemies/skull.png".to_string(), 0, 0, 32, 32, 1, texture_creator, texture_map);

                ani_data.as_mut().unwrap().front = Some(AnimatedTexture::new(1.0/5.0));
                ani_data.as_mut().unwrap().front.as_mut().unwrap().load_animation(
                    "resources/enemies/skull.png".to_string(), 0, 0, 32, 32, 3, texture_creator, texture_map);
                ani_data.as_mut().unwrap().front.as_mut().unwrap().animation_type = AnimationType::PingPong;

                ani_data.as_mut().unwrap().right = Some(AnimatedTexture::new(1.0/5.0));
                ani_data.as_mut().unwrap().right.as_mut().unwrap().load_animation(
                    "resources/enemies/skull.png".to_string(), 0, 32, 32, 32, 3, texture_creator, texture_map);
                ani_data.as_mut().unwrap().right.as_mut().unwrap().animation_type = AnimationType::PingPong;
                
                ani_data.as_mut().unwrap().left = Some(AnimatedTexture::new(1.0/5.0));
                ani_data.as_mut().unwrap().left.as_mut().unwrap().load_animation(
                    "resources/enemies/skull.png".to_string(), 0, 64, 32, 32, 3, texture_creator, texture_map);
                ani_data.as_mut().unwrap().left.as_mut().unwrap().animation_type = AnimationType::PingPong; 
                
                ani_data.as_mut().unwrap().back = Some(AnimatedTexture::new(1.0/5.0));
                ani_data.as_mut().unwrap().back.as_mut().unwrap().load_animation(
                    "resources/enemies/skull.png".to_string(), 0, 96, 32, 32, 3, texture_creator, texture_map);
                ani_data.as_mut().unwrap().back.as_mut().unwrap().animation_type = AnimationType::PingPong; 
            
                ani_data.as_mut().unwrap().default = Some(AnimatedTexture::new(1.0));
                ani_data.as_mut().unwrap().default.as_mut().unwrap().load_animation(
                    "resources/enemies/skull.png".to_string(), 0, 0, 32, 32, 1, texture_creator, texture_map);
            }
            EnemyType::Placeholder => {}
        }
        
        Enemy {
            x: 50.,
            y: 50.,
            animation_data: ani_data,
            size_x: size_x,
            size_y: size_y,
            hitbox: AABB::new(55., 55., size_x - ((0.1* (size_x as f32)) as u32) , size_y - ((0.1* (size_y as f32)) as u32)),
            last_time: 0.,
            dir: -1,
            kind : kind,
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
                    self.size_x,
                    self.size_y,
                );
            }
            None => {
                canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 192, 203));
                canvas
                    .fill_rect(sdl2::rect::Rect::new(
                        (self.x - camera.x) as i32,
                        (self.y - camera.y) as i32,
                        self.size_x,
                        self.size_y,
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
            0 => {
                self.y -= 2. * level.tile_size as f64 * dt;
                match self.kind {
                    EnemyType::Stonewalker | EnemyType::Slime => self.animation_data.as_mut().unwrap().current_animation = AnimationState::Front,
                    EnemyType::Wizard | EnemyType::Skull => self.animation_data.as_mut().unwrap().current_animation = AnimationState::Back,
                    _ => (),
                }
            }
            1 => {
                self.x += 2. * level.tile_size as f64 * dt;
                match self.kind {
                    EnemyType::Stonewalker | EnemyType::Slime => self.animation_data.as_mut().unwrap().current_animation = AnimationState::Front,
                    EnemyType::Wizard | EnemyType::Skull => self.animation_data.as_mut().unwrap().current_animation = AnimationState::Right,
                    _ => (),
                }
            }
            2 => {
                self.y += 2. * level.tile_size as f64 * dt;
                match self.kind {
                    EnemyType::Stonewalker | EnemyType::Slime => self.animation_data.as_mut().unwrap().current_animation = AnimationState::Front,
                    EnemyType::Wizard | EnemyType::Skull => self.animation_data.as_mut().unwrap().current_animation = AnimationState::Front,
                    _ => (),
                }
            }
            3 => {
                self.x -= 2. * level.tile_size as f64 * dt;
                match self.kind {
                    EnemyType::Stonewalker | EnemyType::Slime => self.animation_data.as_mut().unwrap().current_animation = AnimationState::Front,
                    EnemyType::Wizard | EnemyType::Skull => self.animation_data.as_mut().unwrap().current_animation = AnimationState::Left,
                    _ => (),
                }
            }
            _ => self.animation_data.as_mut().unwrap().current_animation = AnimationState::Default,
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
