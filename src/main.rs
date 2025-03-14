use std::vec;

use entities::player::Player;
use game_lib::{ BaseGameObject, GameObject, Tile, TileChunk };
use macroquad::prelude::*;
use miniquad::window::set_window_size;
use noise::{ NoiseFn, Perlin, Seedable };
use ::rand::{ thread_rng, Rng };
mod game_lib;
mod entities;

#[macroquad::main("Game")]
async fn main() {
    let tileset = load_texture("src/assets/tileset.png").await.unwrap();
    tileset.set_filter(FilterMode::Nearest);
    let chunk_tile_count = 20.0;
    let tile_size = 16.0;
    let chunk_size = chunk_tile_count * tile_size;
    set_window_size(1280, 960);
    let dirt_tiles = vec![
        Tile::new(10, "dirt0", Vec2::new(0.0, 0.0), Vec2::new(6.0, 0.0), tile_size),
        Tile::new(11, "dirt1", Vec2::new(0.0, 0.0), Vec2::new(6.0, 1.0), tile_size),
        Tile::new(12, "dirt2", Vec2::new(0.0, 0.0), Vec2::new(6.0, 2.0), tile_size)
    ];
    let grass_tiles = vec![
        Tile::new(1, "grass1", Vec2::new(0.0, 0.0), Vec2::new(3.0, 0.0), tile_size),
        Tile::new(2, "grass2", Vec2::new(0.0, 0.0), Vec2::new(4.0, 0.0), tile_size),
        Tile::new(3, "grass3", Vec2::new(0.0, 0.0), Vec2::new(5.0, 0.0), tile_size),
        Tile::new(4, "grass4", Vec2::new(0.0, 0.0), Vec2::new(3.0, 1.0), tile_size),
        Tile::new(5, "grass5", Vec2::new(0.0, 0.0), Vec2::new(4.0, 1.0), tile_size),
        Tile::new(6, "grass6", Vec2::new(0.0, 0.0), Vec2::new(5.0, 1.0), tile_size),
        Tile::new(7, "grass7", Vec2::new(0.0, 0.0), Vec2::new(3.0, 2.0), tile_size),
        Tile::new(8, "grass8", Vec2::new(0.0, 0.0), Vec2::new(4.0, 2.0), tile_size),
        Tile::new(9, "grass9", Vec2::new(0.0, 0.0), Vec2::new(5.0, 2.0), tile_size)
    ];
    let mut rng = thread_rng();
    let random_seed: u32 = rng.gen();
    let perlin = Perlin::new(random_seed);
    // Create an empty vector to hold TileChunk instances
    let mut chunks: Vec<TileChunk> = Vec::new();
    let chunks_x = 4; // Number of chunks in the x direction
    let chunks_y = 3;
    // Populate the vector with TileChunk instances
    for y in 0..chunks_y {
        for x in 0..chunks_x {
            let chunk_position = Vec2::new((x as f32) * chunk_size, (y as f32) * chunk_size);
            let mut chunk = TileChunk::new(
                chunk_position,
                chunk_tile_count,
                chunk_tile_count,
                tile_size
            );
            chunk.populate(&perlin, &grass_tiles, &dirt_tiles);
            chunks.push(chunk);
        }
    }
    let mut game_objects: Vec<Box<dyn GameObject>> = vec![
        Box::new(
            Player::new(
                BaseGameObject {
                    position: Vec2::new(100.0, 100.0),
                },
                100.0,
                200.0,
                "src/assets/spritesheet_player.png",
                45.0,
                39.0
            ).await
        )
    ];
    loop {
        clear_background(BLACK);

        for chunk in chunks.iter_mut() {
            for tile in chunk.tiles.iter_mut() {
                tile.draw(&tileset);
            }
        }
        // Update and draw all game objects
        for obj in game_objects.iter_mut() {
            obj.update();
            obj.draw();
        }

        next_frame().await;
    }
}
