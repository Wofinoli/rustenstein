mod map;
mod window;

use map::Map;
use window::WindowState;
use winit::{
    event::*,
    event_loop::ControlFlow,
};
use futures::executor::block_on;


pub fn run() {

    let map = Map::new(String::from("test.txt"));
    match map {
        Ok(m) => println!("{:?}", m),
        Err(e) => println!("{}", e),
    }

    let (event_loop, mut state) = block_on(WindowState::default()).expect("Window creation failed.");

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window.id() => if !state.input(event) {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input, .. } => WindowState::handle_key_input(*input),
                    WindowEvent::Resized(physical_size) => { state.resize(*physical_size) },
                    WindowEvent::ScaleFactorChanged {new_inner_size, ..} => { state.resize(**new_inner_size) },
                    _ => (),
                }
            },
            Event::RedrawRequested(_) => {
                state.update();
                state.render();
            },
            Event::MainEventsCleared => {
                state.window.request_redraw();
            },
            _ => ()
        }
    });
}
