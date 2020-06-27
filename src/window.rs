use winit::{
    event::{Event, WindowEvent, KeyboardInput},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
    dpi::LogicalSize,
    error::OsError,
};

#[derive(Debug)]
pub struct WinitState {
    event_loop: EventLoop<()>,
    window: Window,
}



impl WinitState {
    /// Constructs a new `EventsLoop` and `Window` pair.
    ///
    /// The specified title and size are used, other elements are default.
    /// ## Failure
    /// Window creating may fail, but this is unlikely
    pub fn new<T: Into<String>>(title: T, size: LogicalSize<f64>) -> Result<Self, OsError> {
        let event_loop = EventLoop::new();
        let output = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(size)
            .build(&event_loop);
        output.map(|window| Self {
            event_loop,
            window,
        })
    }

    /// Runs the window and event loop. 
    /// This handles all inputs to the game.
    pub fn run(state: WinitState) {
        let event_loop = state.event_loop;

        event_loop.run( move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,
                Event::WindowEvent {
                    event: WindowEvent::KeyboardInput { input, .. },
                    ..
                } => handle_key_input(input),
                _ => (),
            }
        });
    }

}

pub const WINDOW_NAME: &str = "Rustenstein";

impl Default for WinitState {
    /// Makes an 640x480 windwow with `WINDOW_NAME` as the title.
    /// ## Panics
    /// If an `OsError` occurs.
    fn default() -> WinitState {
        WinitState::new(
            WINDOW_NAME,
            LogicalSize {
                width: 640.0,
                height: 480.0,
            },
        )
        .expect("Could not create window.")
    }
}

/// Takes keyboard input and directs it to the
/// appropriate place.
fn handle_key_input(input: KeyboardInput) {
    println!("{:?}", input);
}
