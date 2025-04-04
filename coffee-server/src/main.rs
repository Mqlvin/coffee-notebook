use axum::{
    extract::Query,
    routing::get,
    Router,
};
use tower_http::services::ServeDir;
use std::fs::{read_to_string, write};
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
struct CoffeeParameters {
    #[serde(skip_serializing)]
    old_name: Option<String>, // purely used with requests to update

    name: String,
    colours: String,
    grind_size: u8,
    shot_volume: u16,
    shot_time: u16,
    temperature: f32,
    shot_weight: f32,
}



#[tokio::main]
async fn main() {
    println!(" -> Server starting on port :3000");

    let app = Router::new()
        .fallback_service(ServeDir::new("./routes").append_index_html_on_directories(true))
        .route("/api/add", get(add_recipe))
        .route("/api/get", get(get_recipes))
        .route("/api/remove", get(remove_recipe))
    ;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}




async fn add_recipe(params: Query<CoffeeParameters>) -> String {
    let coffee_parameters: CoffeeParameters = params.0;
    let is_updating = !coffee_parameters.old_name.clone().unwrap().is_empty();

    let original_json = read_to_string("./coffee.json");
    let raw_json: String;
    match original_json {
        Ok(str) => {
            raw_json = str;
        }

        Err(err) => {
            return format!("Error reading JSON file: {}", err).to_string();
        }
    }

    let mut parsed: Vec<CoffeeParameters> = serde_json::from_str(&raw_json.to_string()).unwrap();
    if is_updating {
        parsed.retain(|item| item.name != coffee_parameters.old_name.clone().unwrap());
    }
    parsed.push(coffee_parameters);
    let write_string: String = serde_json::to_string(&parsed).unwrap();
    let task = write("./coffee.json", write_string.into_bytes().clone());
    match task {
        Ok(_) => {
            println!("[info] coffee recipe added.");
            "{'success':true}".to_string()
        }
        Err(err) => {
            format!(r##"{{'success':false, 'reason':'{}'}}"##, err).to_string()
        }
    }
}


async fn get_recipes() -> String {
    read_to_string("./coffee.json").unwrap().to_string()
}

async fn remove_recipe(name: Query<std::collections::HashMap<String, String>>) -> String {
    let to_remove_name = name.get("name").unwrap();

    let original_json = read_to_string("./coffee.json");
    let raw_json: String;
    match original_json {
        Ok(str) => {
            raw_json = str;
        }

        Err(err) => {
            return format!("Error reading JSON file: {}", err).to_string();
        }
    }

    let mut parsed: Vec<CoffeeParameters> = serde_json::from_str(&raw_json.to_string()).unwrap();
    let original_size: usize = parsed.len();
    parsed.retain(|recipe| recipe.name != *to_remove_name); 
    let retained_size: usize = parsed.len();
    let write_string: String = serde_json::to_string(&parsed).unwrap();
    let task = write("./coffee.json", write_string.into_bytes().clone());
    match task {
        Ok(_) => {
            println!("[info] coffee recipe removed.");
            format!("{{'success':true, 'removed':{}}}", original_size - retained_size).to_string()
        }
        Err(err) => {
            format!(r##"{{'success':false, 'reason':'{}'}}"##, err).to_string()
        }
    }
}
