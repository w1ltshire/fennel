#[cfg(test)]
use std::path::PathBuf;

#[test]
fn resource_management() {
    use crate::resources::{LoadableResource, ResourceManager, loadable};

    let mut manager = ResourceManager::new();
    let path = PathBuf::from("examples/example.png");
    let asset = loadable::Image::load(path.clone());

    manager.cache_asset(asset.unwrap()).unwrap();
    assert_eq!(
        "examples/example.png",
        manager
            .resources
            .get(&path.to_string_lossy().to_string())
            .unwrap()
            .name()
    );
}
