use serde::Deserialize;

/// Scene struct
#[derive(Deserialize, Debug)]
pub struct Scene {
    /// Scene internal name
    pub name: String
}
