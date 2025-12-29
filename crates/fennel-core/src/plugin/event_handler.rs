use kanal::{Receiver, Sender};
use log::error;
use crate::events::{KeyboardEvent, MouseClickEvent, MouseMotionEvent, MouseWheelEvent, WindowEventHandler};
use crate::graphics::Drawable;
use crate::Window;

#[derive(Debug)]
#[allow(dead_code)]
pub enum PluginEvent {
	KeyboardEvent(KeyboardEvent),
	MouseMotionEvent(MouseMotionEvent),
	MouseClickEvent(MouseClickEvent),
	MouseWheelEvent(MouseWheelEvent),
}

pub(crate) struct EventHandler {
	pub(crate) render_receiver: Receiver<Vec<Drawable>>,
	pub(crate) event_sender: Sender<PluginEvent>,
}

// Corpse locked in the bathroom
// Blood inside my sink
// I wanna be catatonic
// Where I can't even think
// Bleeding out inside my closet

impl WindowEventHandler for EventHandler {
	fn update(&mut self, _window: &mut Window) -> anyhow::Result<()> {
		Ok(())
	}

	fn draw(&mut self, window: &mut Window) -> anyhow::Result<()> {
		if let Ok(Some(queue)) = self.render_receiver.try_recv() {
			window.graphics.canvas.clear();
			for drawable in queue {
				match drawable {
					Drawable::Image(sprite) => {
						window.graphics.draw_image(
							sprite.image,
							sprite.transform.position,
							sprite.transform.rotation,
							false,
							false
						).unwrap_or_else(|e| { error!("failed to draw image: {e}"); });
					},
					Drawable::Text { font, position, text, color, size } => {
						window.graphics.draw_text(text, position, font, color, size)
							.unwrap_or_else(|e| { error!("failed to draw text: {e}"); });
					},
					Drawable::Rect { w, h, x, y } => {
						window.graphics.draw_rect(x, y, w, h)
							.unwrap_or_else(|e| { error!("failed to draw rectangle: {e}"); });
					}
				}
			}
		}
		window.graphics.canvas.present();
		Ok(())
	}

	fn key_down_event(&mut self, _window: &mut Window, event: KeyboardEvent) -> anyhow::Result<()> {
		self.event_sender.send(PluginEvent::KeyboardEvent(event))?;
		Ok(())
	}

	fn key_up_event(&mut self, _window: &mut Window, event: KeyboardEvent) -> anyhow::Result<()> {
		self.event_sender.send(PluginEvent::KeyboardEvent(event))?;
		Ok(())
	}

	fn mouse_motion_event(&mut self, _window: &mut Window, event: MouseMotionEvent) -> anyhow::Result<()> {
		self.event_sender.send(PluginEvent::MouseMotionEvent(event))?;
		Ok(())
	}

	fn mouse_button_down_event(&mut self, _window: &mut Window, event: MouseClickEvent) -> anyhow::Result<()> {
		self.event_sender.send(PluginEvent::MouseClickEvent(event))?;
		Ok(())
	}

	fn mouse_button_up_event(&mut self, _window: &mut Window, event: MouseClickEvent) -> anyhow::Result<()> {
		self.event_sender.send(PluginEvent::MouseClickEvent(event))?;
		Ok(())
	}

	fn mouse_wheel_event(&mut self, _window: &mut Window, event: MouseWheelEvent) -> anyhow::Result<()> {
		self.event_sender.send(PluginEvent::MouseWheelEvent(event))?;
		Ok(())
	}
}