use crate::phi::{Phi, View, ViewAction};
use sdl2::pixels::Color;

pub struct DefaultView;

impl View for DefaultView {
    fn render(&mut self, context: &mut Phi, _: f64) -> ViewAction {
        let renderer = &mut context.renderer;
        let events = &context.events;

        if events.now.quit || events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        ViewAction::None

    }
}