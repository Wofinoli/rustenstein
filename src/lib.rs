mod window;

use window::{
    KeyEvent,
    WindowUtil,
    Line,
};

use sdl2::{
    pixels::Color,
    event::Event,
    keyboard::Keycode,
};

use std::time::Duration;

pub fn run() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'main
                },
                Event::KeyDown {..} | Event::KeyUp {..} => {
                    WindowUtil::handle_keys(KeyEvent{ up: true, up_event: None, down_event: Some(event)});
                },
                _ => {}
            }
        }
        // Screen only updates here.
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
