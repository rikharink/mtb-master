#![allow(dead_code)]
#![feature(drain_filter)]
#![feature(exclusive_range_pattern)]
mod background;
mod constants;
mod game;
mod geometry;
mod obstacles;
mod player;
mod shaders;
mod util;

#[macro_use]
extern crate lazy_static;
use constants::*;
use macroquad::{prelude::*, window};

use game::*;

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "Jos Harink's MTB Master".to_owned(),
        high_dpi: true,
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");
    let mut accumulator: f32 = 0.;
    let rock = load_texture("rock.png").await.unwrap();
    rock.set_filter(FilterMode::Nearest);
    let mut state = Game::new(rock);

    loop {
        let delta_time = get_frame_time();
        clear_background(PALETTE[0]);
        if delta_time > 1. {
            // skip updating and make sure the pause menu is shown
            state.state = GameState::Paused;
            next_frame().await;
            continue;
        }
        state.tick();
        accumulator += delta_time;
        while accumulator >= TIMESTEP {
            state.step();
            accumulator -= TIMESTEP;
        }
        let alpha = accumulator / delta_time;
        state.render(alpha);
        next_frame().await
    }
}
