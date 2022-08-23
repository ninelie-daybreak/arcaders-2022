use crate::phi::{Phi, View, ViewAction};
use sdl2::pixels::Color;
use sdl2::rect::Rect as SdlRect;

const PLAYER_SPEED:f64 = 180.0;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rectangle {
    /// Generates an SDL-compatible Rect equivalent to `self`
    /// Panics if it could not be created, for example if a
    /// coordinate of a corner overflows an `i32`
    pub fn to_sdl(self) -> SdlRect {
        // Reject negative width and height
        assert!(self.w >= 0.0 && self.h >= 0.0);

        // SdlRect::new : `(i32, i32, i32, i32) -> Result<Option<SdlRect>>
        SdlRect::new(self.x as i32, self.y as i32, self.w as u32, self.h as u32)
    }
}

struct Ship {
    rect: Rectangle
}
pub struct ShipView {
    player: Ship,
}

impl ShipView {
    pub fn new(phi: &mut Phi) -> ShipView {
        ShipView {
            player: Ship { 
                rect: Rectangle {
                    x: 64.0,
                    y: 64.0,
                    w: 32.0,
                    h: 32.0,
                } 
            }
        }
    }
}

impl View for ShipView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        // TODO: Insert the moving logic here
        let diagonal = 
            (phi.events.key_up ^ phi.events.key_down) &&
            (phi.events.key_left ^ phi.events.key_right);

        let moved = 
            if diagonal { 1.0 / 2.0f64.sqrt()}
            else { 1.0 } * PLAYER_SPEED * elapsed;
        
        let dx = match (phi.events.key_left, phi.events.key_right) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -moved,
            (false, true) => moved,
        };

        let dy = match (phi.events.key_up, phi.events.key_down) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -moved,
            (false, true) => moved,
        };

        self.player.rect.x += dx;
        self.player.rect.y += dy;

        // Clear the screen
        phi.renderer.set_draw_color(Color::RGB(0, 0, 0));
        phi.renderer.clear();

        // Render the scene
        phi.renderer.set_draw_color(Color::RGB(200, 200, 50));
        phi.renderer.fill_rect(self.player.rect.to_sdl()).unwrap();

        ViewAction::None
    }
}
