mod map;
mod window;

use map::Map;
use window::WindowState;
use winit::{
    event::*,
    event_loop::ControlFlow,
};


pub fn run() {

    let map = Map::new(String::from("test.txt"));
    match map {
        Ok(m) => println!("{:?}", m),
        Err(e) => println!("{}", e),
    }

    let (event_loop, window) = WindowState::default().expect("Window creation failed.");

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => WindowState::handle_key_input(*input),
                _ => (),
            }
            _ => ()
        }
    });
}
