use std::{fs, path::PathBuf};
use anyhow::Context;
use log::debug;
use serde::{Deserialize, Serialize};
use fennel_resources::manager::ResourceManager;
use fennel_resources::resource::Resource;
use crate::{
    graphics::Graphics,
    resources::{font::DummyFont, image::Image},
};

pub mod font;
pub mod image;

#[derive(Deserialize, Serialize, Debug)]
enum AssetType {
    Image,
    Audio,
    Font,
}

#[derive(Deserialize, Serialize, Debug)]
/// Manifest structure of an asset presented in manifest
struct Asset {
    name: String,
    path: String,
    #[serde(rename(deserialize = "type"))]
    class: AssetType,
}

#[derive(Deserialize, Debug)]
/// Assets package manifest
struct Manifest {
    pub assets: Vec<Asset>,
}

pub fn load_dir(resource_manager: &mut ResourceManager, path_buf: PathBuf, graphics: &mut Graphics) -> anyhow::Result<()> {
    let manifest_file = fs::read(path_buf.join("manifest.toml"))
        .context("failed to read manifest.toml")?;

    let manifest: Manifest = toml::from_slice(&manifest_file)
        .context("failed to parse manifest.toml")?;

    for asset in manifest.assets {
        debug!("loading asset '{}' with class {:?}", asset.name, asset.class);

        let asset_path = path_buf.join(asset.path);

        match asset.class {
            AssetType::Image => {
                let image = Image::load(
                    asset_path.clone(),
                    asset.name,
                    graphics
                )?;
                println!("inserting {}", image.name());
                resource_manager.insert(image);
            }
            AssetType::Audio => {

            }
            AssetType::Font => {
                println!("{:?} - {}", asset_path.clone(), asset.name.clone());
                let font = DummyFont::new(asset_path, asset.name);
                resource_manager.insert(font);
            }
        }
    }

    Ok(())
}