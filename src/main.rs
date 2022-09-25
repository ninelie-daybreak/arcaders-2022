extern crate sdl2;
extern crate rand;

mod phi;
// mod views;

use sdl2::pixels::Color;

struct MyView;

impl crate::phi::View for MyView {
    fn update(self: Box<Self>, phi: &mut crate::phi::Phi, elapsed: f64) -> crate::phi::ViewAction {
        if phi.events.now.quit || phi.events.key_escape {
            return crate::phi::ViewAction::Quit;
        }

        crate::phi::ViewAction::Render(self)
    }

    fn render(&self, phi: &mut crate::phi::Phi) {
        phi.renderer.set_draw_color(Color::RGB(0, 0, 0));
        phi.renderer.clear();
    }
}

fn main() {
    crate::phi::spawn("ArcadeRS Shooter", |phi| {
        Box::new(MyView)
    });
}