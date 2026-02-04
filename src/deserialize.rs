// config_loader.rs
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct Position { pub x: f32, pub y: f32 }

#[derive(Deserialize, Debug)]
pub struct Scale { pub x: f32, pub y: f32 }

#[derive(Deserialize, Debug)]
pub struct Collider {
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Deserialize, Debug)]
pub struct ScriptConfig {
    #[serde(rename = "type")]
    pub script_type: String,
    #[serde(flatten)]
    pub params: Value,
}

#[derive(Deserialize, Debug)]
pub struct GameObjectConfig {
    pub name: String,
    pub position: Position,
    pub rotation: f32,
    pub scale: Scale,
    pub colliders: Vec<Collider>,
    pub scripts: Vec<ScriptConfig>,
    pub sprite: Option<String>,
}
