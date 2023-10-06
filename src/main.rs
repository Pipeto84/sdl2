extern crate sdl2;
use std::{time::Duration,thread::sleep};
use sdl2::{pixels::Color,event::Event,keyboard::Keycode};

pub fn main() {
    let sdl_context=sdl2::init().expect("SDL initialization failed");
    let video_subsystem=sdl_context.video()
        .expect("couldn't get SDL video subsystem");
    let window=video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .expect("failed to create window");
    let mut canvas=window.into_canvas().build().expect("failed to convert window into canvas");

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump=sdl_context.event_pump().expect("failed to get SDL event pump");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown {  keycode: Some(Keycode::Escape), .. } =>
                {
                    break 'running
                },
                _ => {}
            }
        }
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}