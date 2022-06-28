extern crate sdl2;

use sdl2::pixels::Color;
use std::thread;
use std::time::Duration;

fn main() {
    // Initialize sdl2
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    // Create the window
    let window = video.window("ArcadeRS Shooter", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas().accelerated().build().unwrap();

    // Render a fully black window
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    thread::sleep(Duration::from_millis(3000));

}