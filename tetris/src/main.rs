//! This code is a game of tetris, using glium as communication with the interface. 
//! The logic of the game is in `logic` the rest of the modules 
//! are designed to be part of the logic as if they were an engine but embedded
#![feature(trait_alias)]
//#![deny(missing_docs)]
pub mod gui;
/// Module containing the engine core, vital parts such as vectors
#[macro_use]
pub mod core;
pub mod logic;

extern crate glium;
use glium::{
    glutin::{event, event::{KeyboardInput, ElementState, VirtualKeyCode, MouseButton}, event_loop, dpi::PhysicalPosition},
    Surface,
};
use std::{time, cmp::{min, max}};

pub use crate::core::vector2;
use crate::{gui::interface, logic::GameState, vector2::{Vec2, ToVec2}};

fn main() {
    const TARGET_FPS: u64 = 120;
    let event_loop = event_loop::EventLoop::new();
    let mut facade = interface::Interface::create(&event_loop);
    let mut game_state = GameState::new(&facade);
    let mut last_update =   time::Instant::now();
    let mut last_key : Option<VirtualKeyCode> = None;  
    let mut mouse_position = Vec2::ZERO;
    
    event_loop.run(move |ev, _, control_flow| {
        let start_time = time::Instant::now();
        match &control_flow {
            event_loop::ControlFlow::WaitUntil(_) => {},
             _ => {
                *control_flow = event_loop::ControlFlow::WaitUntil(start_time);
             }
        };

        if let event::Event::WindowEvent { event, .. } = ev {
            match event {
                event::WindowEvent::CursorMoved { position, .. } => {
                    let scale_factor=facade.display.gl_window().window().scale_factor();
                    let size = &facade.display.gl_window().window().inner_size();
                    let (x,y) : (f32, f32) = position.to_logical::<f64>(scale_factor).into();
                    let (w,h) : (f32, f32)= size.to_logical::<f64>(scale_factor).into();
                    mouse_position = vec2!(x/w-0.5, -y/h + 0.5) * 2.;

                }
                event::WindowEvent::MouseInput { button: MouseButton::Left, .. } => {
                    game_state.on_click(facade.camera.target_to_world(mouse_position));
                }
                event::WindowEvent::CloseRequested => {
                    *control_flow = event_loop::ControlFlow::Exit
                }
                event::WindowEvent::Resized(window_size) => {
                    facade.camera.target.size.x = (window_size.height as f32) / (window_size.width as f32);
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
        }  else {
            match ev {
                event::Event::MainEventsCleared => {

                },
                event::Event::NewEvents(event::StartCause::ResumeTimeReached { .. } ) => {
                    let delta_t = time::Instant::now().duration_since(last_update).as_micros();
                    last_update = start_time;
                    facade.camera.world = game_state.world_region();
                    let mut canvas = facade.draw();
                    canvas.target.clear_color(0.0, 0.0, 0.0, 1.0);
                    game_state.update(&mut canvas, delta_t);
                    canvas.target.finish().unwrap();
            
                    let elapsed_time = time::Instant::now().duration_since(start_time).as_millis() as u64;
                    let wait_millis = max(1000 / TARGET_FPS, elapsed_time) - elapsed_time;
                    let new_inst = start_time + time::Duration::from_millis(wait_millis);
                    *control_flow = event_loop::ControlFlow::WaitUntil(new_inst);
                },
                event::Event::RedrawEventsCleared => {

                },
                _ => {}
            }
        }


    });
}