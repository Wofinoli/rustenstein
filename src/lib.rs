mod game;

use game::Game;

use sdl2::{
    pixels::Color,
    event::Event,
    keyboard::Keycode,
};

use time::Instant;

pub fn run() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Rustenstein", 640, 480)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Initialise Game
    let mut time = Instant::now();
    let mut game = Game::default();

    'main: loop {
         // Keep track of how long a frame took to draw so that all movement is at a constant speed
        let prev_time = time;
        time = Instant::now();
        let frame_time = (time - prev_time).whole_milliseconds() as f64 / 1000.0;
        println!("FPS {:?}", 1.0 / frame_time);

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {

           match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'main
                },
                Event::KeyDown {scancode: Some(code), ..} => {
                    game.handle_keys(code, frame_time);
                },
                _ => {}
            }
        }
        game.draw(&mut canvas);
        // Screen only updates here.
        canvas.present();

        game.update();
    }
}
