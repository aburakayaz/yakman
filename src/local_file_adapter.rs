use std::fs;

use rocket::serde::json::serde_json;
use serde::{Deserialize, Serialize};

use crate::{
    adapters::ConfigStorageAdapter,
    data_types::{Config, ConfigInstance, Label, LabelType},
};

pub struct LocalFileStorageAdapter {
    pub path: String,
}

const CONFIG_MAN_DIR: &str = ".configman"; // TODO: clean up
const DATA_DIR: &str = "config-instances"; // TODO: clean up

#[derive(Debug, Serialize, Deserialize)]
struct LabelJson {
    labels: Vec<LabelType>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigJson {
    configs: Vec<Config>,
}

#[derive(Debug, Serialize, Deserialize)]
struct InstanceJson {
    instances: Vec<ConfigInstance>,
}

impl ConfigStorageAdapter for LocalFileStorageAdapter {
    fn get_configs(self) -> Vec<Config> {
        let content =
            fs::read_to_string(self.path + "/" + CONFIG_MAN_DIR + "/configs.json").unwrap();
        let v: ConfigJson = serde_json::from_str(&content).unwrap();
        return v.configs;
    }

    fn get_labels(self) -> Vec<LabelType> {
        let label_file = self.path + "/" + CONFIG_MAN_DIR + "/labels.json";
        let content = fs::read_to_string(label_file).unwrap();
        let v: LabelJson = serde_json::from_str(&content).unwrap();
        return v.labels;
    }

    fn get_config_instance_metadata(self, id: &str) -> Option<Vec<ConfigInstance>> {
        let label_file =
            self.path + "/" + CONFIG_MAN_DIR + "/instance-metadata/" + &id.to_string() + ".json";
        if let Some(content) = fs::read_to_string(label_file).ok() {
            let v: InstanceJson = serde_json::from_str(&content).unwrap();
            return Some(v.instances);
        }
        return None;
    }

    fn get_config_data(self, id: &str, labels: Vec<Label>) -> Option<String> {
        let base_path = self.path.to_string();
        if let Some(instances) = self.get_config_instance_metadata(id) {
            let mut selected_instance: Option<ConfigInstance> = None;

            for instance in instances {
                if instance.labels == labels {
                    // TODO: Create better comparison logic
                    selected_instance = Some(instance);
                    break;
                }
            }

            if let Some(instance) = selected_instance {
                let path = base_path + "/" + DATA_DIR + "/" + instance.instance_id.as_str();
                println!("Found path {}", path);
                return fs::read_to_string(path).ok();
            } else {
                println!("No selected instance found");
                return None;
            }
        }
        return None;
    }
}
