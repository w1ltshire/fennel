use specs::{Dispatcher, World, WorldExt};
use std::sync::{Arc, Mutex};

// no this is probably not fucking safe
// oh hey my ide doesn't highlight 'fucking' as a typo!!!

pub struct ThreadSafeWorld {
    world: Arc<Mutex<World>>,
}

unsafe impl Send for ThreadSafeWorld {}
unsafe impl Sync for ThreadSafeWorld {}

impl ThreadSafeWorld {
    pub fn new(world: World) -> Self {
        ThreadSafeWorld {
            world: Arc::new(Mutex::new(world)),
        }
    }
    
    pub fn lock(&self) -> std::sync::MutexGuard<'_, World> {
        self.world.lock().unwrap()
    }
}

pub struct ThreadSafeDispatcher {
    dispatcher: Arc<Mutex<Dispatcher<'static, 'static>>>,
    world: ThreadSafeWorld
}

unsafe impl Send for ThreadSafeDispatcher {}
unsafe impl Sync for ThreadSafeDispatcher {}

impl ThreadSafeDispatcher {
    pub fn new(dispatcher: Dispatcher<'static, 'static>, world: World) -> Self {
        ThreadSafeDispatcher {
            dispatcher: Arc::new(Mutex::new(dispatcher)),
            world: ThreadSafeWorld::new(world)
        }
    }

    pub fn dispatch(&self) {
        let mut dispatcher = self.dispatcher.lock().unwrap();
        let world = &mut *self.world.lock();
        dispatcher.dispatch(world);
        world.maintain();
    }
    
    pub fn world(&self) -> std::sync::MutexGuard<'_, World> {
        self.world.lock()
    }
}