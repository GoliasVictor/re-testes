mod gui;
#[macro_use]
mod core;
mod logic;

extern crate glium;
use glium::{
    glutin::{event, event::KeyboardInput, event_loop},
    Surface,
};
use std::time;

pub use crate::core::vector2;
use crate::{gui::{ Object, Rect, interface}, logic::GameState};
fn main() {
    const TARGET_FPS: u64 = 120;
    let event_loop = event_loop::EventLoop::new();
    let mut facade = interface::Interface::create(&event_loop);


    let mut obj = Object {
        format: Rect {
            size: vec2!(10., 10.),
            center: vector2::ZERO,
        },
        color: [1., 1., 1.],
    };
    let mut game_state = GameState::new(10, 10);
    let mut last_start =   time::Instant::now();
    let mut last_delta_time: u128 = 0;
    event_loop.run(move |ev, _, control_flow| {
        let start_time = time::Instant::now();
        last_delta_time = time::Instant::now().duration_since(last_start).as_micros() as u128;
        last_start = start_time;
        match ev {   
            event::Event::WindowEvent { event, .. } => match event {
                event::WindowEvent::CloseRequested => {
                    *control_flow = event_loop::ControlFlow::Exit;
                    return;
                }
                event::WindowEvent::Resized(window_size) => {
                    facade
                        .display
                        .gl_window()
                        .window();
                    facade.camera.target.size.x =
                        (window_size.height as f32) / (window_size.width as f32);
                }
                event::WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(input),
                            ..
                        },
                    ..
                } =>{
                    game_state.key_press(input);

                    match input {
                        event::VirtualKeyCode::Up => obj.format.center += vector2::UP,
                        event::VirtualKeyCode::Down => obj.format.center += vector2::DOWN,
                        event::VirtualKeyCode::Left => obj.format.center += vector2::LEFT,
                        event::VirtualKeyCode::Right => obj.format.center += vector2::RIGHT,
                        _ => (),
                    }   
                },    

                _ => (),
            },
            
            event::Event::RedrawRequested(_) => {
                
                let mut canvas = facade.draw();
                canvas.target.clear_color(0.0, 0.0, 0.0, 1.0);
                last_delta_time = time::Instant::now().duration_since(last_start).as_micros() as u64;
                //last_delta_time = time::Instant::now().duration_since(last_start).as_micros() as u64;
                game_state.update(&mut canvas, last_delta_time);
                canvas.draw_obj(&obj);
                canvas.target.finish().unwrap();
            }
            event::Event::RedrawEventsCleared => {
                facade.display.gl_window().window().request_redraw();
            }
            _ => (),
        }

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