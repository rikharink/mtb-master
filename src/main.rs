#![allow(dead_code)]
#![feature(drain_filter)]
mod background;
mod constants;
mod geometry;
mod obstacles;
mod player;
mod shaders;
mod state;
mod util;

#[macro_use]
extern crate lazy_static;
use constants::*;
use macroquad::prelude::*;

use state::*;

#[macroquad::main("MTB")]
async fn main() {
    let mut accumulator: f32 = 0.;
    let mut state = State::default();

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
