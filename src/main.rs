extern crate sdl2;

// #[macro_use] asks the complier to import the macros defined in the `events`
// module. This is necessary because macros cannot be namespaced -- macro 
// expansion happens before the concept of namespace event starts to _exist_ in
// the compilation timeline.
#[macro_use]
mod events;

use sdl2::pixels::Color;

// We cannot call functions at top-level. However, `struct_events` is not your 
// usual function : it's a macro .Which means that you can use a macro to do 
// pretty much anything _normal_ code would.
struct_events!();


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

    // Prepare the events record
    let mut events = Events::new(sdl_context.event_pump().unwrap());

    loop {
        events.pump();

        if true {
            break;
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

    }
}