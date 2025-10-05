use fennel_common::events::{
    KeyboardEvent, MouseClickEvent, MouseMotionEvent, MouseWheelEvent, WindowEventHandler,
};
use crate::runtime::Runtime;

pub struct RuntimeHandler;
#[async_trait::async_trait]
impl WindowEventHandler for RuntimeHandler {
    type Host = Runtime;

    fn update(&self, _window: &mut Runtime) -> anyhow::Result<()> {
        Ok(())
    }

    fn draw(&self, _window: &mut Runtime) -> anyhow::Result<()> {
        Ok(())
    }

    fn key_down_event(&self, _window: &mut Runtime, _event: KeyboardEvent) -> anyhow::Result<()> {
        Ok(())
    }

    fn key_up_event(&self, _window: &mut Runtime, _event: KeyboardEvent) -> anyhow::Result<()> {
        Ok(())
    }

    fn mouse_motion_event(
        &self,
        _window: &mut Runtime,
        _event: MouseMotionEvent,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn mouse_button_down_event(
        &self,
        _window: &mut Runtime,
        _event: MouseClickEvent,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn mouse_button_up_event(
        &self,
        _window: &mut Runtime,
        _event: MouseClickEvent,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn mouse_wheel_event(
        &self,
        _window: &mut Runtime,
        _event: MouseWheelEvent,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

pub type BoxedEngineHandler = Box<dyn WindowEventHandler<Host = Runtime> + Send + Sync>;
