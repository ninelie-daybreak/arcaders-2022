extern crate sdl2;

mod phi;
mod views;

fn main() {
    crate::phi::spawn("ArcadeRS Shooter", |phi| {
        Box::new(crate::views::ShipView::new(phi))
    });
}