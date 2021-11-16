#![allow(dead_code)]

mod background;
mod constants;
mod player;

#[macro_use]
extern crate lazy_static;
use background::*;
use constants::*;
use macroquad::prelude::*;

use player::*;

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

fn background_layers(amount: usize) -> Vec<Background> {
    let mut layers: Vec<Background> = Vec::new();
    for _ in 0..amount {
        layers.push(Background::new(BackgroundType::Mountains));
    }
    layers
}

#[macroquad::main("MTB")]
async fn main() {
    let mut player = Player::new(32., 32.);
    let mut accumulator: f32 = 0.;
    let mut is_running: bool = false;
    let mut background_layers = background_layers(1);
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
        for layer in &mut background_layers {
            layer.tick();
        }

        accumulator += delta_time;
        while accumulator >= TIMESTEP {
            player.step();
            accumulator -= TIMESTEP;
        }

        for layer in &mut background_layers {
            layer.render();
        }

        player.render();
        next_frame().await
    }
}
