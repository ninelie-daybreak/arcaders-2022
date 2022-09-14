extern crate sdl2;
extern crate rand;

mod phi;
mod views;

fn main() {
    crate::phi::spawn("ArcadeRS Shooter", |phi| {
        Box::new(crate::views::main_menu::MainMenuView::new(phi))
    });
}