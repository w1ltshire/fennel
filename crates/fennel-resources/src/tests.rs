use std::any::Any;
use crate::manager::ResourceManager;
use crate::resource::Resource;

struct MyResource {
	data: u32
}

impl Resource for MyResource {
	fn data(&self) -> &dyn Any {
		&self.data
	}

	fn data_mut(&mut self) -> &mut dyn Any {
		&mut self.data
	}

	fn name(&self) -> &'static str {
		"my_resource"
	}
}

#[test]
fn resource_remove() {
	let resource = MyResource { data: 42 };
	let mut manager = ResourceManager::new();
	manager.insert(resource);
	
	let resource_removed = manager.remove("my_resource").unwrap();
	
	assert_eq!(resource_removed.data().downcast_ref::<u32>().unwrap(), &42);
}

#[test]
fn resource_ref() {
	let resource = MyResource { data: 42 };
	let mut manager = ResourceManager::new();
	manager.insert(resource);
	
	let resource_ref = manager.get("my_resource").unwrap();
	
	assert_eq!(resource_ref.data().downcast_ref::<u32>().unwrap(), &42);
}

#[test]
fn resource_ref_mut() {
	let mut manager = ResourceManager::new();
	manager.insert(MyResource { data: 42 });

	let resource_ref = manager.get_mut("my_resource").unwrap();
	assert_eq!(resource_ref.data().downcast_ref::<u32>().unwrap(), &42);

	*resource_ref.data_mut().downcast_mut::<u32>().unwrap() = 0x42;
	assert_eq!(resource_ref.data().downcast_ref::<u32>().unwrap(), &0x42);
}