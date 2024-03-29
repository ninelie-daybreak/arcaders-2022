use crate::phi::data::Rectangle;
use crate::phi::gfx::{Sprite, CopySprite};
use sdl2::render::WindowCanvas;

#[derive(Clone)]
pub struct Background {
    pub pos: f64,
    // The amount of pixels moved to the left every second
    pub vel: f64,
    pub sprite: Sprite,
}

impl Background {
    /// Move the background proportionally to the elapsed time since the last
    /// frame and the background's velocity.
    pub fn update(&mut self, elapsed: f64) {
        // We define a logical position as depending solely on the time and the
        // dimensions of the image, not on the screen's size.
        let size = self.sprite.size();
        self.pos += self.vel * elapsed;
        if self.pos > size.0 {
            self.pos -= size.0;
        }
    }

    /// Render the background at ist current position, and as many times as
    /// required to fill the screen.
    pub fn render(&self, renderer: &mut WindowCanvas) {
        // We determine the scale ratio of the window to the sprte.
        let size = self.sprite.size();
        let (win_w, win_h) = renderer.output_size().unwrap();
        let scale = win_h as f64 / size.1;

        // We render as many copies of the background as necessary to fill
        // the screen.
        let mut physical_left = -self.pos * scale;

        while physical_left < win_w as f64 {
            renderer.copy_sprite(&self.sprite, Rectangle {
                x: physical_left,
                y: 0.0,
                w: size.0 * scale,
                h: win_h as f64,
            });

            physical_left += size.0 * scale;
        }
    }   
}