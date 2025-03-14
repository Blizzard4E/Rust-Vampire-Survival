use std::collections::HashMap;

use macroquad::prelude::*;
use noise::{ NoiseFn, Perlin };
use ::rand::{ thread_rng, Rng };

pub trait GameObject {
    fn update(&mut self);
    fn draw(&mut self);
}

pub struct BaseGameObject {
    pub position: Vec2,
}

impl BaseGameObject {
    pub fn new(position: Vec2) -> Self {
        Self { position }
    }
}
#[derive(Debug)]
pub struct Tile {
    pub id: u32,
    pub name: &'static str,
    pub position: Vec2,
    pub tile_pos: Vec2,
    pub tile_size: f32,
}

impl Tile {
    pub fn new(
        id: u32,
        name: &'static str,
        position: Vec2,
        tile_pos: Vec2,
        tile_size: f32
    ) -> Self {
        Self { id, name, position, tile_pos, tile_size }
    }
    pub fn draw(&mut self, texture: &Texture2D) {
        draw_texture_ex(texture, self.position.x, self.position.y, WHITE, DrawTextureParams {
            source: Some(
                Rect::new(
                    self.tile_pos.x * self.tile_size,
                    self.tile_pos.y * self.tile_size,
                    self.tile_size,
                    self.tile_size
                )
            ),
            ..Default::default()
        });
    }
}

pub struct TileChunk {
    pub position: Vec2,
    pub tiles: Vec<Tile>,
    pub width: f32,
    pub height: f32,
    pub sprite_size: f32,
}

impl TileChunk {
    pub fn new(position: Vec2, width: f32, height: f32, sprite_size: f32) -> Self {
        let tiles = Vec::new();
        Self {
            position,
            tiles,
            width,
            height,
            sprite_size,
        }
    }
    pub fn populate(&mut self, perlin: &Perlin, grass_tiles: &Vec<Tile>, dirt_tiles: &Vec<Tile>) {
        for y in 0..self.height as u32 {
            for x in 0..self.width as u32 {
                // Generate noise value
                let nx =
                    ((x as f64) + (self.position.x as f64) / (self.width as f64)) /
                    (self.width as f64);
                let ny =
                    ((y as f64) + (self.position.y as f64) / (self.height as f64)) /
                    (self.height as f64);
                let noise_value = perlin.get([nx, ny]);
                let mut rng = thread_rng();
                // Determine tile type based on noise value
                let tile = if noise_value < -0.2 {
                    &grass_tiles[rng.gen_range(0..grass_tiles.len())]
                } else if noise_value < 0.2 {
                    &grass_tiles[rng.gen_range(0..grass_tiles.len())]
                } else {
                    &dirt_tiles[rng.gen_range(0..dirt_tiles.len())]
                };

                self.tiles.push(
                    Tile::new(
                        tile.id,
                        tile.name,
                        Vec2::new(
                            (x as f32) * tile.tile_size + self.position.x,
                            (y as f32) * tile.tile_size + self.position.y
                        ),
                        tile.tile_pos,
                        tile.tile_size
                    )
                );
            }
        }
    }
}
pub struct Animation {
    pub frame_count: usize,
    pub frame_duration: f32,
    pub sprite_x: f32,
    pub sprite_y: f32,
    pub sprite_width: f32,
    pub sprite_height: f32,
}

pub struct Animator {
    pub animations: HashMap<String, Animation>,
    current_animation: String,
    current_frame: usize,
    elapsed_time: f32,
}
impl Animator {
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            current_animation: String::new(),
            current_frame: 0,
            elapsed_time: 0.0,
        }
    }

    pub fn add_animation(&mut self, name: &str, animation: Animation) {
        self.animations.insert(name.to_string(), animation);
    }

    pub fn set_animation(&mut self, name: &str) {
        if self.animations.contains_key(name) {
            if self.current_animation != name {
                self.current_animation = name.to_string();
                self.current_frame = 0;
                self.elapsed_time = 0.0;
            }
        } else {
            println!("Warning: Animation '{}' not found!", name);
        }
    }
    pub fn update(&mut self, delta_time: f32) {
        if let Some(animation) = self.animations.get(&self.current_animation) {
            self.elapsed_time += delta_time;
            if self.elapsed_time >= animation.frame_duration {
                self.elapsed_time = 0.0;
                self.current_frame = (self.current_frame + 1) % animation.frame_count;
            }
        }
    }
    pub fn animate(&mut self, texture: &Texture2D, pos_x: f32, pos_y: f32, flip_x: bool) {
        if let Some(animation) = self.animations.get(&self.current_animation) {
            let frame_x = ((self.current_frame as f32) * animation.sprite_width).floor();
            let frame_y = animation.sprite_y;

            draw_texture_ex(texture, pos_x, pos_y, WHITE, DrawTextureParams {
                source: Some(
                    Rect::new(frame_x, frame_y, animation.sprite_width, animation.sprite_height)
                ),
                dest_size: Some(
                    Vec2::new(animation.sprite_width * 2.0, animation.sprite_height * 2.0)
                ),
                flip_x: flip_x,
                ..Default::default()
            });
        }
    }
}
