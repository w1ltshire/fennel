use kanal::Receiver;
use log::error;
use crate::events::WindowEventHandler;
use crate::graphics::Drawable;
use crate::Window;

pub(crate) struct EventHandler {
	pub(crate) render_receiver: Receiver<Vec<Drawable>>
}

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
}