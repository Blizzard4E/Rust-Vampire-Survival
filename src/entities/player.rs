use macroquad::prelude::*;
use crate::game_object::BaseGameObject;
use crate::game_object::GameObject;
pub struct Player {
    pub game_object: BaseGameObject,
    pub hp: f32,
    pub speed: f32,
    pub texture: Texture2D,
}
impl Player {
    pub async fn new(game_object: BaseGameObject, hp: f32, speed: f32, texture_path: &str) -> Self {
        let texture = load_texture(texture_path).await.unwrap();
        Self { game_object, hp, speed, texture }
    }
}

impl GameObject for Player {
    fn update(&mut self) {
        let delta_time = get_frame_time();
        // Movement direction vector
        let mut direction = Vec2::ZERO;

        if is_key_down(KeyCode::D) {
            direction.x += 1.0;
        }
        if is_key_down(KeyCode::A) {
            direction.x -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            direction.y += 1.0;
        }
        if is_key_down(KeyCode::W) {
            direction.y -= 1.0;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize(); // Normalize to ensure consistent speed
        }

        self.game_object.position += direction * self.speed * delta_time;
    }
    fn draw(&self) {
        draw_texture(&self.texture, self.game_object.position.x, self.game_object.position.y, WHITE)
    }
}
