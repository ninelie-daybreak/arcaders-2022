extern crate sdl2;

mod phi;
mod views;

fn main() {
    crate::phi::spawn("ArcadeRS Shooter", |_| {
        Box::new(crate::views::DefaultView)
    });
}