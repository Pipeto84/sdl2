extern crate sdl2;
use std::{time::{Duration,SystemTime},thread::sleep};
use sdl2::{pixels::Color,event::Event,keyboard::Keycode,render::{Texture,TextureCreator, Canvas},
    rect::Rect,video::{Window,WindowContext}};
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
    let green_square=create_texture_rect(&mut canvas, &texture_creator, TextureColor::Green, TEXTURE_SIZE)
        .expect("failed to create a texture");
    let blue_square=create_texture_rect(&mut canvas, &texture_creator, TextureColor::Blue, TEXTURE_SIZE)
        .expect("failed to create a texture");
    let timer=SystemTime::now();
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
        let display_green=match timer.elapsed() {
            Ok(elapsed)=>elapsed.as_secs()%2==0,
            Err(_)=>{true}
        };
        let square_texture=if display_green {
            &green_square
        } else{
            &blue_square
        };
        canvas.copy(&square_texture, 
            None, 
            Rect::new(0, 0, TEXTURE_SIZE, TEXTURE_SIZE))
            .expect("couldn't copy texture into window");
        canvas.present();
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
#[derive(Clone, Copy)]
enum TextureColor {
    Green,Blue,
}
fn create_texture_rect<'a>(canvas:&mut Canvas<Window>,texture_creator:&'a TextureCreator<WindowContext>,
        color:TextureColor,size:u32)->Option<Texture<'a>> {
    if let Ok(mut square_texture) = texture_creator.create_texture_target(None, size, size) {
        canvas.with_texture_canvas(&mut square_texture, |texture|{
            match color {
                TextureColor::Green=>texture.set_draw_color(Color::RGB(0, 255, 0)),
                TextureColor::Blue=>texture.set_draw_color(Color::RGB(0, 0, 255)),
            }
            texture.clear();
        }).expect("failed to color a texture");
        Some(square_texture)
    }else {
        None
    }
}
