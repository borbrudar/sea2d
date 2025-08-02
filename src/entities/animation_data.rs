use sdl2::{
    render::{Canvas, Texture},
    video::Window,
};
use serde::{Deserialize, Serialize};

use crate::entities::animated_texture::AnimatedTexture;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum AnimationState {
    Front,
    Back,
    Left,
    Right,
    Idle,
    Default,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AnimationData {
    pub front: Option<AnimatedTexture>,
    pub back: Option<AnimatedTexture>,
    pub left: Option<AnimatedTexture>,
    pub right: Option<AnimatedTexture>,
    pub idle: Option<AnimatedTexture>,
    pub default: Option<AnimatedTexture>,
    pub current_animation: AnimationState,
}
impl AnimationData {
    pub fn new() -> AnimationData {
        AnimationData {
            front: None,
            back: None,
            left: None,
            right: None,
            idle: None,
            default: None,
            current_animation: AnimationState::Default,
        }
    }

    pub fn draw(
        &self,
        canvas: &mut Canvas<Window>,
        texture_map: &std::collections::HashMap<String, Texture>,
        x: f64,
        y: f64,
        width: u32,
        height: u32,
    ) {
        let default_draw =
            |canvas: &mut Canvas<Window>, x: f64, y: f64, width: u32, height: u32| {
                canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 192, 203));
                canvas
                    .fill_rect(sdl2::rect::Rect::new(x as i32, y as i32, width, height))
                    .unwrap();
            };
        match self.current_animation {
            AnimationState::Default => {
                if let Some(ref animation_data) = self.default {
                    animation_data.draw(canvas, texture_map, x, y, width, height);
                } else {
                    default_draw(canvas, x, y, width, height);
                }
            }
            AnimationState::Front => {
                if let Some(ref animation_data) = self.front {
                    animation_data.draw(canvas, texture_map, x, y, width, height);
                } else {
                    default_draw(canvas, x, y, width, height);
                }
            }
            AnimationState::Back => {
                if let Some(ref animation_data) = self.back {
                    animation_data.draw(canvas, texture_map, x, y, width, height);
                } else {
                    default_draw(canvas, x, y, width, height);
                }
            }
            AnimationState::Left => {
                if let Some(ref animation_data) = self.left {
                    animation_data.draw(canvas, texture_map, x, y, width, height);
                } else {
                    default_draw(canvas, x, y, width, height);
                }
            }
            AnimationState::Right => {
                if let Some(ref animation_data) = self.right {
                    animation_data.draw(canvas, texture_map, x, y, width, height);
                } else {
                    default_draw(canvas, x, y, width, height);
                }
            }
            AnimationState::Idle => {
                if let Some(ref animation_data) = self.idle {
                    animation_data.draw(canvas, texture_map, x, y, width, height);
                } else {
                    default_draw(canvas, x, y, width, height);
                }
            }
        }
    }

    pub fn update(&mut self, dt: f64) {
        match self.current_animation {
            AnimationState::Default => match self.default {
                Some(ref mut anim) => anim.update(dt),
                None => (),
            },
            AnimationState::Front => match self.front {
                Some(ref mut anim) => anim.update(dt),
                None => (),
            },
            AnimationState::Back => match self.back {
                Some(ref mut anim) => anim.update(dt),
                None => (),
            },
            AnimationState::Left => match self.left {
                Some(ref mut anim) => anim.update(dt),
                None => (),
            },
            AnimationState::Right => match self.right {
                Some(ref mut anim) => anim.update(dt),
                None => (),
            },
            AnimationState::Idle => match self.idle {
                Some(ref mut anim) => anim.update(dt),
                None => (),
            },
        }
    }
}
