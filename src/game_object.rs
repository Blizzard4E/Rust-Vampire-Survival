use macroquad::prelude::*;

pub trait GameObject { 
    fn update(&mut self);
    fn draw(&self);
}

pub struct BaseGameObject {
    pub position: Vec2
}

impl BaseGameObject {
    pub fn new(position: Vec2) -> Self {
        Self { position }
    }
}