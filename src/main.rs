mod adapters;
mod config_man;
mod config_man_state;
mod data_types;
mod raw_query;

use adapters::ConfigStorageAdapter;
use data_types::{Config, ConfigInstance, Label, LabelType};
use adapters::local_file_adapter::LocalFileStorageAdapter;
use adapters::postgres_adapter::PostgresAdapter;
use adapters::redis_adapter::RedisStorageAdapter;
use rocket::{serde::json::Json, State};
use std::vec;

use raw_query::RawQuery;

#[macro_use]
extern crate rocket;

struct StateManager {
    adapter: Box<dyn ConfigStorageAdapter>,
}

impl StateManager {
    fn get_adapter(&self) -> &dyn ConfigStorageAdapter {
        return self.adapter.as_ref();
    }
}

#[launch]
async fn rocket() -> _ {

    let settings = config_man::load_config_man_settings();
    println!("Settings: {:?}", settings);

    // Handle multi adapters
    // let adapter = get_postgres_adapter();
    let adapter = get_local_file_adapter();
    // let adapter = get_redis_adapter();

    rocket::build()
        .manage(StateManager {
            adapter: Box::new(adapter),
        })
        .mount("/", routes![configs, labels, instances, data])
}

#[get("/configs")]
async fn configs(state: &State<StateManager>) -> Json<Vec<Config>> {
    let adapter = state.get_adapter();
    return Json(adapter.get_configs().await);
}

#[get("/labels")]
async fn labels(state: &State<StateManager>) -> Json<Vec<LabelType>> {
    let adapter = state.get_adapter();
    return Json(adapter.get_labels().await);
}

#[get("/instances/<id>")]
async fn instances(id: &str, state: &State<StateManager>) -> Option<Json<Vec<ConfigInstance>>> {
    let adapter = state.get_adapter();
    return match adapter.get_config_instance_metadata(id).await {
        Some(data) => Some(Json(data)),
        None => None,
    };
}

#[get("/data/<config_name>")]
async fn data(config_name: &str, query: RawQuery, state: &State<StateManager>) -> Option<String> {
    let adapter = state.get_adapter();

    let labels: Vec<Label> = query
        .params
        .iter()
        .map(|param| Label {
            label_type: param.0.to_string(),
            value: param.1.to_string(),
        })
        .collect();

    println!("Search for config {} with labels: {:?}", config_name, labels);

    return adapter.get_config_data(config_name, labels).await;
}

fn get_local_file_adapter() -> impl ConfigStorageAdapter {
    return LocalFileStorageAdapter {
        path: "/home/ross/projects/config-manager/testing-directory".to_string(),
    };
}

fn get_redis_adapter() -> impl ConfigStorageAdapter {
    return RedisStorageAdapter {
        host: "127.0.0.1".to_string(),
        port: 6379,
        username: "".to_string(),
        password: "".to_string(),
    };
}
 fn get_postgres_adapter() -> impl ConfigStorageAdapter {
    return PostgresAdapter {
        host: "localhost".to_string(),
        port: 5432,
        username: "postgres".to_string(),
        password: "password".to_string()

    };
 }