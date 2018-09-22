#![allow(dead_code)]
extern crate winconsole;
mod render;
mod matrix;
mod piece;
mod input_handler;
mod state;
mod input;

use input_handler::*;
use render::*;
use state::*;

use std::time::Duration;

fn main()
{
    let mut state = State::new();
    let mut input_handler = InputHandler::new();
    let mut render = Render::new(&state.matrix);
    
    let target_ms = 50;
    let mut next_update = std::time::Instant::now();
    loop
    {
        let now = std::time::Instant::now();
        let input = input_handler.current();
        if now > next_update
        {
            input_handler.update();
            state.update(input);
            render.draw(&state);
            next_update = now + Duration::from_millis(target_ms);
        }
    }
}
