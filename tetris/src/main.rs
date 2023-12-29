//! This code is a game of tetris, using glium as communication with the interface. 
//! The logic of the game is in `logic` the rest of the modules 
//! are designed to be part of the logic as if they were an engine but embedded
#![feature(trait_alias)]
#![deny(missing_docs)]
pub mod gui;
/// Module containing the engine core, vital parts such as vectors
#[macro_use]
pub mod core;
pub mod logic;

extern crate glium;
use glium::{
    glutin::{event, event::{KeyboardInput, ElementState, VirtualKeyCode}, event_loop},
    Surface,
};
use std::time;

pub use crate::core::vector2;
use crate::{gui::interface, logic::GameState};

fn main() {
    const TARGET_FPS: u64 = 120;
    let event_loop = event_loop::EventLoop::new();
    let mut facade = interface::Interface::create(&event_loop);
    let mut game_state = GameState::new(10, 20,&facade);
    let mut last_update =   time::Instant::now();
    let mut last_key : Option<VirtualKeyCode> = None;  
    facade.camera.world.center = vec2!(game_state.columns, game_state.rows) * (logic::SIZE /2. );
    event_loop.run(move |ev, _, control_flow| {
        let start_time = time::Instant::now();
        if let event::Event::WindowEvent { event, .. } = ev {
            match event {
                event::WindowEvent::CloseRequested => {
                    *control_flow = event_loop::ControlFlow::Exit;
                    return;
                }
                event::WindowEvent::Resized(window_size) => {
                    facade.camera.target.size.x =
                        (window_size.height as f32) / (window_size.width as f32);
                }
                event::WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(input),
                            state,
                            ..
                        },
                    ..
                } => {
                    match state {
                        ElementState::Pressed => {
                            if last_key.is_none() || last_key.is_some_and(|k| k != input) {
                                game_state.key_down(input);
                            }
                            last_key = Some(input);
                        },
                        ElementState::Released => {
                            last_key = None;
                        },
                    }
                 
                },
                _ => (),
            }
        }

        
        let delta_t = time::Instant::now().duration_since(last_update).as_micros();
        last_update = start_time;
        let mut canvas = facade.draw();
        canvas.target.clear_color(0.0, 0.0, 0.0, 1.0);
        game_state.update(&mut canvas, delta_t);
        canvas.target.finish().unwrap();



        let elapsed_time = time::Instant::now().duration_since(start_time).as_millis() as u64;
        let wait_millis = if 1000 / TARGET_FPS >= elapsed_time {
            1000 / TARGET_FPS - elapsed_time
        } else {
            0
        };
        let new_inst = start_time + time::Duration::from_millis(wait_millis);
        *control_flow = event_loop::ControlFlow::WaitUntil(new_inst);
    });
}