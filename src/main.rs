use entities::player::Player;
use game_object::{ BaseGameObject, GameObject };
use macroquad::prelude::*;

mod game_object;
mod entities;

#[macroquad::main("Game")]
async fn main() {
    let base: BaseGameObject = BaseGameObject {
        position: Vec2::new(100.0, 100.0),
    };
    let mut player: Player = Player::new(base, 100.0, 200.0, "src/assets/player/idle.png").await;
    loop {
        clear_background(BLACK);

        player.update();
        player.draw();

        next_frame().await;
    }
}
