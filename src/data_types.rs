pub use serde::Deserialize;
pub use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigManState {
    pub configs: Vec<AppConfig>,
    pub labels: Vec<AppLabel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppLabelType {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppLabel {
    pub label_type: AppLabelType,
    pub value: String, // TODO: more powerful generics?
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfigInstance {
    pub config: AppConfig,
    pub content: String,
    pub labels: Vec<AppLabel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigManSettings {
    pub version: String,
}
