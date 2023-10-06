extern crate sdl2;
use std::{time::Duration,thread::sleep};
use sdl2::{pixels::Color,event::Event,keyboard::Keycode,render::{Texture,TextureCreator},
    rect::Rect};
const TEXTURE_SIZE:u32=32;
pub fn main() {
    let sdl_context=sdl2::init().expect("SDL initialization failed");
    let video_subsystem=sdl_context.video()
        .expect("couldn't get SDL video subsystem");
    let window=video_subsystem.window("Tetris", 800, 600)
        .position_centered()
        .build()
        .expect("failed to create window");
    let mut canvas=window.into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .expect("couldn't get window's canvas");
    let texture_creator:TextureCreator<_>=canvas.texture_creator();
    let mut square_texture:Texture=texture_creator
        .create_texture_target(None, TEXTURE_SIZE, TEXTURE_SIZE)
        .expect("failed to create a texture");
    canvas.with_texture_canvas(&mut square_texture, |texture|{
        texture.set_draw_color(Color::RGB(0, 255, 0));
        texture.clear();
    }).expect("faliled to color a texture");
    let mut event_pump=sdl_context.event_pump().expect("failed to get SDL event pump");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown {  keycode: Some(Keycode::Escape), .. } =>{break 'running},
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();
        canvas.copy(&square_texture, 
            None, 
            Rect::new(0, 0, TEXTURE_SIZE, TEXTURE_SIZE))
            .expect("couldn't copy texture into window");
        canvas.present();
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
