use fennel_core::events::KeyboardEvent;

/// Struct containing a vector of events to pass it to systems
#[derive(Default)]
pub struct KeyEvents(pub Vec<KeyboardEvent>);
