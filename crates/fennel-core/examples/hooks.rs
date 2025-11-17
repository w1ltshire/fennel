// This example involves rendering ONLY the ImGui, because it uses GPU rendering, while
// Fennel in its core graphics module utilizes rendering through a canvas.
// You can't have ImGui or something that utilizes GPU rendering and draw something onto the canvas
// at the same time.

use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use fennel_core::{
    Window,
    events::{self, WindowEventHandler},
    graphics,
    hooks::Hook,
    resources::ResourceManager,
};
use imgui_sdl3::ImGuiSdl3;
use sdl3::{
    EventPump,
    event::Event,
    gpu::{ColorTargetInfo, Device, LoadOp, ShaderFormat, StoreOp},
    pixels::Color,
};

struct State;
struct MyHook {
    device: Option<Device>,
    imgui: Option<ImGuiSdl3>,
}

impl WindowEventHandler for State {
    fn update(&mut self, _window: &mut Window) -> anyhow::Result<()> {
        Ok(())
    }

    fn draw(&mut self, _window: &mut Window) -> anyhow::Result<()> {
        Ok(())
    }
}

impl Hook for MyHook {
    fn prepare(&mut self, _event_pump: &mut EventPump, window: &mut Window) {
        let dev = Device::new(ShaderFormat::SPIRV, true)
            .unwrap()
            .with_window(window.graphics.canvas.window())
            .unwrap();

        self.device = Some(dev);

        let device_ref = self.device.as_ref().unwrap();
        self.imgui = Some(ImGuiSdl3::new(
            device_ref,
            window.graphics.canvas.window(),
            |ctx| {
                ctx.set_ini_filename(None);
                ctx.set_log_filename(None);
                ctx.fonts()
                    .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);
            },
        ));
    }

    fn update(&mut self, event_pump: &mut EventPump, window: &mut Window) {
        let device = self
            .device
            .as_mut()
            .expect("device not initialized (oddly af)");
        let imgui = self
            .imgui
            .as_mut()
            .expect("imgui not initialized (oddly af)");
        let mut command_buffer = device.acquire_command_buffer().unwrap();
        if let Ok(swapchain) =
            command_buffer.wait_and_acquire_swapchain_texture(window.graphics.canvas.window())
        {
            let color_targets = [ColorTargetInfo::default()
                .with_texture(&swapchain)
                .with_load_op(LoadOp::CLEAR)
                .with_store_op(StoreOp::STORE)
                .with_clear_color(Color::RGB(128, 128, 128))];

            imgui.render(
                &mut window.graphics.sdl_context,
                device,
                window.graphics.canvas.window(),
                event_pump,
                &mut command_buffer,
                &color_targets,
                |ui| {
                    ui.show_demo_window(&mut true);
                    ui.text("hi!!! hello hi hiiiii!!! :3 :3 ;3");
                },
            );

            command_buffer.submit().unwrap();
        } else {
            println!("Swapchain unavailable, cancel work");
            command_buffer.cancel();
        }
    }

    fn handle(&mut self, event: &Event) {
        let imgui = self
            .imgui
            .as_mut()
            .expect("imgui not initialized (oddly af)");
        imgui.handle_event(event);
    }

    fn name(&self) -> String {
        String::from("test hook")
    }
}

#[tokio::main]
async fn main() {
    let resource_manager = Arc::new(Mutex::new(ResourceManager::new()));
    let graphics = graphics::GraphicsBuilder::new()
        .window_name(String::from("game"))
        .dimensions((1360, 768))
        .resource_manager(resource_manager.clone())
        .initializer(|graphics| {
            resource_manager
                .lock()
                .expect("failed to acquire resource_manager lock")
                .load_dir(PathBuf::from("assets"), graphics)
                .expect("failed to load assets from directory");
        })
        .build();
    let mut window = Window::new(
        graphics.expect("failed to create graphics"),
        resource_manager,
    );

    let handler: &'static mut dyn WindowEventHandler = {
        let boxed = Box::new(State);
        Box::leak(boxed) as &'static mut dyn WindowEventHandler
    };

    events::run(
        &mut window,
        handler,
        vec![Box::new(MyHook {
            device: None,
            imgui: None,
        })],
    )
    .await;
}
