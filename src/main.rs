use macroquad::prelude::*;

#[macroquad::main("Space Defender")]
async fn main() {
    loop {
        next_frame().await;
    }
}
