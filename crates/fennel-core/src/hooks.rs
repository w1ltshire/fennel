use sdl3::{EventPump, event::Event};

use crate::Window;

// i'm feeling kinda jolly rn
// oohhh i'm faaaalliiing ohhhhh i'm faaadiiiing ohhh have i lost it a-aaaallll

/// Trait that a hook type must implement to be passed to [`events::run`]
pub trait Hook {
    /// Do here any preparations for your hook that you need.
    /// Inside, this function is called before entering the main loop
    fn prepare(&mut self, _event_pump: &mut EventPump, _window: &mut Window) {}

    /// Main function of your loop.
    /// Inside, this function is called after updating state of the app or drawing it.
    fn update(&mut self, _event_pump: &mut EventPump, _window: &mut Window) {}

    /// Handle sdl3 events, like key press/depress or mouse events.
    fn handle(&mut self, _event: &Event) {}

    /// Return the name of your hook here
    fn name(&self) -> String;
}
