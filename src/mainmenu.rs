use macroquad::prelude::*;

use crate::exit_modes::ExitMode;

pub async fn menu() -> ExitMode {
    loop {
        clear_background(BLACK);

        // Text
        let text = String::from("Press [SPACE] to start");
        let measure = measure_text(&text, None, 30, 1.);
        draw_text(
            &text,
            screen_width() / 2. - measure.width / 2.,
            screen_height() / 2. - measure.height / 2.,
            30.0,
            WHITE,
        );

        if is_key_pressed(KeyCode::Space) {
            break ExitMode::NewGame;
        } else if is_key_pressed(KeyCode::Escape) {
            break ExitMode::Quit;
        }

        next_frame().await
    }
}
