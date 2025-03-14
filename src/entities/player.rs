use macroquad::prelude::*;
use crate::game_lib::Animation;
use crate::game_lib::Animator;
use crate::game_lib::BaseGameObject;
use crate::game_lib::GameObject;
pub struct Player {
    pub game_object: BaseGameObject,
    pub hp: f32,
    pub speed: f32,
    pub texture: Texture2D,
    pub animator: Animator,
    pub sprite_width: f32,
    pub sprite_height: f32,
    pub direction: Vec2,
}
impl Player {
    pub async fn new(
        game_object: BaseGameObject,
        hp: f32,
        speed: f32,
        texture_path: &str,
        sprite_width: f32,
        sprite_height: f32
    ) -> Self {
        let texture = load_texture(texture_path).await.unwrap();
        texture.set_filter(FilterMode::Nearest);

        let mut animator = Animator::new();
        animator.add_animation("idle", Animation {
            frame_count: 4,
            frame_duration: 0.2,
            sprite_x: 0.0,
            sprite_y: 0.0,
            sprite_width: sprite_width,
            sprite_height: sprite_height,
        });
        animator.add_animation("walk", Animation {
            frame_count: 6,
            frame_duration: 0.2,
            sprite_x: 0.0,
            sprite_y: 39.0,
            sprite_width: sprite_width,
            sprite_height: sprite_height,
        });
        animator.set_animation("idle");

        let direction = Vec2::new(0.0, 0.0);

        Self { game_object, hp, speed, texture, animator, sprite_width, sprite_height, direction }
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
            direction = direction.normalize();
            self.animator.set_animation("walk");
        } else {
            self.animator.set_animation("idle");
        }
        self.direction = direction;
        self.game_object.position += direction * self.speed * delta_time;
        self.animator.update(delta_time)
    }
    fn draw(&mut self) {
        self.animator.animate(
            &self.texture,
            self.game_object.position.x,
            self.game_object.position.y,
            self.direction.x < 0.0
        );
    }
}
