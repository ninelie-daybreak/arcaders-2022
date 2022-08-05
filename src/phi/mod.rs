// #[macro_use] asks the complier to import the macros defined in the `events`
// module. This is necessary because macros cannot be namespaced -- macro 
// expansion happens before the concept of namespace event starts to _exist_ in
// the compilation timeline.
#[macro_use]#[macro_use]
mod events;

use sdl2::render::WindowCanvas;

struct_events! {
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down
    },
    else: {
        quit: Quit { .. }
    }
}

/// Bundles the Phi abstractions in a single structure witch
/// can be passed easily between functions.
pub struct Phi {
    pub events: Events,
    pub renderer: WindowCanvas,
}

/// A `ViewAction` is a way for the currently executed view to
/// communicate with the game loop. It specifies which action
/// should be executed before the next rendering.
pub enum ViewAction {
    None,
    Quit,
}

pub trait View {
    /// Called on every frame to take care of both the logic and
    /// the rendering of the current view
    /// 
    /// `elapsed` is expressed in seconds.
    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction;
}