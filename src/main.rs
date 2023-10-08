use sdl2::{event::Event,keyboard::Keycode,pixels::Color,
    image::{LoadTexture,INIT_PNG,INIT_JPG},
    render::TextureCreator};
use std::time::Duration;
fn main() {
    let sdl_context=sdl2::init().expect("inicializacion sdl fallo");
    let video_subsystem=sdl_context.video().expect("no se pudo el video subsystem");
    sdl2::image::init(INIT_PNG|INIT_JPG).expect("no pudo inicializar el contexto imagen");
    let window=video_subsystem.window("Prueba", 800, 600)
        .position_centered()
        .build()
        .expect("fallo crear window");
    let mut canvas=window.into_canvas()
        .build()
        .expect("no se pudo obtener canvas de window");
    let texture_creator:TextureCreator<_>=canvas.texture_creator();

    let image_texture=texture_creator.load_texture("assets/Vector.png")
        .expect("no pudo cargar la imagen");

    let mut event_pump=sdl_context.event_pump().expect("fallo el sdl event pump");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode:Some(Keycode::Escape), .. }=>{break 'running},
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.clear();

        canvas.copy(&image_texture, None, None).expect("fallo el prestamo");
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}