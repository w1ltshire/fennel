use std::sync::{Arc, Mutex};

use fennel_core::{events::{KeyboardEvent, WindowEventHandler}, Window};
use specs::{Dispatcher, DispatcherBuilder, WorldExt};

use crate::{ecs::{input::InputSystem, sprite::{HostPtr, RenderingSystem, Sprite}}, events::KeyEvents};

pub struct App {
    pub window: fennel_core::Window,
    pub world: specs::World,
    pub dispatcher: Dispatcher<'static, 'static>,
}

#[derive(Default, Debug)]
pub struct AppBuilder {
    name: &'static str,
    dimensions: (u32, u32),
}

unsafe impl Send for App {}
unsafe impl Sync for App {}

#[async_trait::async_trait]
impl WindowEventHandler for App {
    fn update(&self, _window: &mut Window) -> anyhow::Result<()> {
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> anyhow::Result<()> {
        self.frame_tick();
        window.graphics.canvas.present();
        Ok(())
    }

    fn key_down_event(&self, _window: &mut Window, event: KeyboardEvent) -> anyhow::Result<()> {
        println!("{:?}", event.keycode);
        Ok(())
    }
}

impl App {
    /// Runs the event loop, must be called only once, UB otherwise
    pub async fn run(mut self) -> anyhow::Result<()> {
        // you know what? fuck you and your borrow checker.
        // i'm 100% sure this app is single-threaded and its 11 pm
        // at the moment so i'm not gonna solve this shit in some
        // safe way
        // as long this works and doesn't SEGFAULTs i'll keep it 
        let ptr: *mut App = &mut self as *mut App;
        fennel_core::events::run(&mut self.window, unsafe { &mut *ptr as &mut App }).await;
        Ok(())
    }

    pub fn frame_tick(&mut self) {
        let host_ptr = HostPtr(self as *mut App);
        self.world.insert(host_ptr);
        self.dispatcher.dispatch(&self.world);
        self.world.maintain();
        self.world.remove::<HostPtr>();
    }
}

impl AppBuilder {
    pub fn new() -> AppBuilder {
        AppBuilder {
            name: "",
            dimensions: (100, 100),
        }
    }

    pub fn name(mut self, name: &'static str) -> AppBuilder {
        self.name = name;
        self
    }

    pub fn dimensions(mut self, dimensions: (u32, u32)) -> AppBuilder {
        self.dimensions = dimensions;
        self
    }

    pub fn build(&self) -> anyhow::Result<App> {
        let resource_manager = Arc::new(Mutex::new(fennel_core::resources::ResourceManager::new()));
        let graphics = fennel_core::graphics::Graphics::new(
            self.name.to_string(),
            self.dimensions,
            resource_manager.clone(),
            |_| {}
        );
        let window = fennel_core::Window::new(
            graphics.expect("failed to initialize graphics"),
            resource_manager,
        );
        let mut world = specs::World::new();
        let mut dispatcher = DispatcherBuilder::new()
            .with_thread_local(RenderingSystem)
            .with(InputSystem, "input_system", &[])
            .build();
        world.register::<Sprite>();
        world.insert(KeyEvents::default());
        dispatcher.setup(&mut world);

        Ok(App {
            window,
            world,
            dispatcher,
        })
    }
}
