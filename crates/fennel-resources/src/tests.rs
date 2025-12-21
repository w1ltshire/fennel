use std::any::Any;
use crate::manager::ResourceManager;
use crate::resource::Resource;

struct MyResource {
	name: String,
	data: u32
}

impl Resource for MyResource {
	fn data(&self) -> &dyn Any {
		&self.data
	}

	fn data_mut(&mut self) -> &mut dyn Any {
		&mut self.data
	}

	fn name(&self) -> String {
		self.name.clone()
	}
}

#[test]
fn resource_remove() {
	let resource = MyResource { name: "my_resource".to_string(), data: 42 };
	let mut manager = ResourceManager::new();
	manager.insert(resource);
	
	let resource_removed = manager.remove("my_resource").unwrap();
	
	assert_eq!(resource_removed.data().downcast_ref::<u32>().unwrap(), &42);
}

#[test]
fn resource_ref() {
	let resource = MyResource { name: "my_resource".to_string(), data: 42 };
	let mut manager = ResourceManager::new();
	manager.insert(resource);
	
	let resource_ref = manager.get("my_resource").unwrap();
	
	assert_eq!(resource_ref.data().downcast_ref::<u32>().unwrap(), &42);
}

#[test]
fn resource_ref_mut() {
	let mut manager = ResourceManager::new();
	manager.insert(MyResource { name: "my_resource".to_string(), data: 42 });

	let resource_ref = manager.get_mut("my_resource").unwrap();
	assert_eq!(resource_ref.data().downcast_ref::<u32>().unwrap(), &42);

	*resource_ref.data_mut().downcast_mut::<u32>().unwrap() = 0x42;
	assert_eq!(resource_ref.data().downcast_ref::<u32>().unwrap(), &0x42);
}

#[test]
fn multiple_resources() {
	let mut manager = ResourceManager::new();
	manager.insert(MyResource { name: "my_resource_1".to_string(), data: 42 });
	manager.insert(MyResource { name: "my_resource_2".to_string(), data: 0xDEADBEEF });
	let resource_ref1 = manager.get("my_resource_1").unwrap();
	let resource_ref2 = manager.get("my_resource_2").unwrap();
	assert_eq!(resource_ref1.data().downcast_ref::<u32>().unwrap(), &42);
	assert_eq!(resource_ref2.data().downcast_ref::<u32>().unwrap(), &0xDEADBEEF);
}