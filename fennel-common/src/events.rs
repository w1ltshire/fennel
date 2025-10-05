use sdl3::{
    keyboard::{Keycode, Mod, Scancode},
    mouse::{MouseButton, MouseState, MouseWheelDirection},
};

pub struct KeyboardEvent {
    pub timestamp: u64,
    pub window_id: u32,
    pub keycode: Option<Keycode>,
    pub scancode: Option<Scancode>,
    pub keymod: Mod,
    pub repeat: bool,
    pub which: u32,
    pub raw: u16,
}

pub struct MouseMotionEvent {
    pub timestamp: u64,
    pub window_id: u32,
    pub which: u32,
    pub mousestate: MouseState,
    pub x: f32,
    pub y: f32,
    pub xrel: f32,
    pub yrel: f32,
}

pub struct MouseClickEvent {
    pub timestamp: u64,
    pub window_id: u32,
    pub which: u32,
    pub mouse_btn: MouseButton,
    pub clicks: u8,
    pub x: f32,
    pub y: f32,
}

pub struct MouseWheelEvent {
    pub timestamp: u64,
    pub window_id: u32,
    pub which: u32,
    pub x: f32,
    pub y: f32,
    pub direction: MouseWheelDirection,
    pub mouse_x: f32,
    pub mouse_y: f32,
}

#[async_trait::async_trait]
pub trait WindowEventHandler: Send + Sync {
    type Host;

    fn update(&self, host: &mut Self::Host) -> anyhow::Result<()>;
    fn draw(&self, host: &mut Self::Host) -> anyhow::Result<()>;

    fn key_down_event(&self, _host: &mut Self::Host, _event: KeyboardEvent) -> anyhow::Result<()> {
        Ok(())
    }
    fn key_up_event(&self, _host: &mut Self::Host, _event: KeyboardEvent) -> anyhow::Result<()> {
        Ok(())
    }
    fn mouse_motion_event(
        &self,
        _host: &mut Self::Host,
        _event: MouseMotionEvent,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn mouse_button_down_event(
        &self,
        _host: &mut Self::Host,
        _event: MouseClickEvent,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn mouse_button_up_event(
        &self,
        _host: &mut Self::Host,
        _event: MouseClickEvent,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn mouse_wheel_event(
        &self,
        _host: &mut Self::Host,
        _event: MouseWheelEvent,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
