#![allow(dead_code)]

mod constants;
mod player;
mod util;
mod shaders;

#[macro_use]
extern crate lazy_static;
use constants::*;
use macroquad::prelude::*;

use player::*;
use util::rgba_texture;

fn render_menu() -> bool {
    let half_width = screen_width() * 0.5;
    let mut x = half_width;
    let mut y = screen_height() * 0.5;

    // DRAW TITLE
    let title = "MTB MASTER";
    let title_measure = measure_text(title, None, 96, 1.);
    x -= title_measure.width * 0.5;
    y -= title_measure.height * 0.5;
    draw_text(title, x, y, 96., PALETTE[15]);

    // DRAW INSTRUCTIONS
    y += title_measure.height * 2.;
    let press_to_start = "CLICK/TOUCH TO START";
    let press_to_start_measure = measure_text(press_to_start, None, 64, 1.);
    x = half_width - press_to_start_measure.width * 0.5;
    draw_text(press_to_start, x, y, 64., PALETTE[15]);

    is_mouse_button_pressed(MouseButton::Left)
}

#[macroquad::main("MTB")]
async fn main() {
    let mut player = Player::new(32., 32.);
    let mut accumulator: f32 = 0.;
    let mut is_running: bool = false;
    let texture = rgba_texture(256, 256);
    loop {
        clear_background(PALETTE[0]);
        if !is_running {
            is_running = render_menu();
            next_frame().await;
            continue;
        }

        let delta_time = get_frame_time();
        if delta_time > 1. {
            // skip updating and make sure the pause menu is shown
            is_running = false;
            next_frame().await;
            continue;
        }

        player.tick();

        accumulator += delta_time;
        while accumulator >= TIMESTEP {
            player.step();
            accumulator -= TIMESTEP;
        }

        player.render();
        draw_texture(texture, 250., 250., WHITE);
        next_frame().await
    }
}


