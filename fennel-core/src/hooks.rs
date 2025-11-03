use sdl3::event::Event;

// i'm feeling kinda jolly rn 
// oohhh i'm faaaalliiing ohhhhh i'm faaadiiiing ohhh have i lost it a-aaaallll

pub trait Hook {
    /// Do here any preparations for your hook that you need. 
    /// Inside, this function is called in the very beginning of the loop (so continuosly called)
    /// before iterating over events. 
    fn prepare(&mut self) {}

    /// Main function of your loop.
    /// Inside, this function is called after updating state of the app or drawing it.
    fn update(&mut self ) {}

    /// Handle sdl3 events, like key press/depress or mouse events.
    fn handle(&mut self, _event: &Event) {}

    /// Return the name of your hook here
    fn name(&self) -> String;
}
