use macroquad::prelude::*;

pub async fn menu() {
    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Space) {
            break;
        }

        next_frame().await
    }
}