#![allow(dead_code)]

mod constants;
mod player;

#[macro_use]
extern crate lazy_static;
use constants::*;
use player::*;
use macroquad::{prelude::*};

#[macroquad::main("MTB")]
async fn main() {
    
    let mut player = Player::new(32., 32.);
    let mut accumulator: f32 = 0.;
    let mut started: bool = false;
    loop {
        clear_background(PALETTE[0]);

        /* START */
        if !started {
            let half_width = screen_width() * 0.5;
            let half_height = screen_height() * 0.5;

            let mut x = half_width;
            let mut y = half_height;
            let title = "MTB MASTER";
            let title_measure = measure_text(title, None, 96, 1.);
            x -= title_measure.width * 0.5;
            y -= title_measure.height * 0.5;
            draw_text(title, x, y, 96., PALETTE[15]);

            y += title_measure.height * 2.;
            let press_to_start = "CLICK/TOUCH TO START";
            let press_to_start_measure = measure_text(press_to_start, None, 64, 1.);
            x = half_width - press_to_start_measure.width * 0.5;
            draw_text(press_to_start, x, y, 64., PALETTE[15]);
            
            started = is_mouse_button_pressed(MouseButton::Left);
            next_frame().await;
            continue;
        }

        /* SETUP */
        let delta_time = get_frame_time();
        if delta_time > 1. {
            next_frame().await;
            continue;
        }
        accumulator += delta_time;

        /* TICK */
        player.tick();

        /* STEP */
        while accumulator >= 1. / TPS {
            player.step();
            accumulator -= 1. / TPS;
        }

        /* RENDER */
        player.render();
        draw_text(&format!("distance: {}m", player.distance.round() as i32), 16., 64., 64., PALETTE[15]);
        next_frame().await
    }
}
