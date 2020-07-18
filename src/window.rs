use winit::{
    event::KeyboardInput,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
    dpi::LogicalSize,
    error::OsError,
};

#[derive(Debug)]
pub struct WindowState {
    event_loop: EventLoop<()>,
    window: Window,
}

pub const WINDOW_NAME: &str = "Rustenstein";

impl WindowState {
    /// Constructs a new `EventsLoop` and `Window` pair.
    ///
    /// The specified title and size are used, other elements are default.
    /// ## Failure
    /// Window creating may fail, but this is unlikely
    pub fn new<T: Into<String>>(title: T, size: LogicalSize<f64>) 
            -> Result<(EventLoop<()>, Window), OsError> {
        let event_loop = EventLoop::new();
        let output = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(size)
            .build(&event_loop);
        
        Ok( (event_loop, output.unwrap()) )
    }

    /// Takes keyboard input and directs it to the
    /// appropriate place.
    pub fn handle_key_input(input: KeyboardInput) {
        println!("{:?}", input);
    }

    /// Makes an 640x480 windwow with `WINDOW_NAME` as the title.
    /// ## Panics
    /// If an `OsError` occurs.
    pub fn default() -> Result<(EventLoop<()>, Window ), OsError> {
        WindowState::new(
            WINDOW_NAME,
            LogicalSize {
                width: 640.0,
                height: 480.0,
            },
        )
    }
}
