mod handlers;
mod models;

use axum::{routing::get, Router};
use handlers::AppState;
use models::{Recipe, TagsConfig};
use std::sync::Arc;
use std::{fs, path::Path};
use tera::Tera;
use tower_http::services::ServeDir;

fn find_z_image(dir_path: &Path) -> Option<String> {
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let file_name = entry.file_name().to_string_lossy().to_string();
            if file_name.starts_with("z-image") && file_name.ends_with(".png") {
                return Some(file_name);
            }
        }
    }
    None
}

fn load_recipes(base_path: &Path) -> (Vec<Recipe>, Vec<String>) {
    let tags_path = base_path.join("tags.json");
    let tags_content = fs::read_to_string(&tags_path).expect("Failed to read tags.json");
    let tags_config: TagsConfig = serde_json::from_str(&tags_content).expect("Failed to parse tags.json");

    let mut recipes = Vec::new();

    if let Ok(entries) = fs::read_dir(base_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let dir_name = entry.file_name().to_string_lossy().to_string();
                // Skip hidden directories and known non-recipe directories
                if dir_name.starts_with('.') || dir_name == "src" || dir_name == "static" || dir_name == "templates" || dir_name == "target" {
                    continue;
                }

                if let Some(image_name) = find_z_image(&path) {
                    let image_path = format!(
                        "/{}/{}",
                        urlencoding::encode(&dir_name),
                        urlencoding::encode(&image_name)
                    );
                    let tags = tags_config
                        .recipes
                        .get(&dir_name)
                        .cloned()
                        .unwrap_or_default();
                    recipes.push(Recipe::new(dir_name, image_path, tags));
                }
            }
        }
    }

    recipes.sort_by(|a, b| a.name.cmp(&b.name));
    (recipes, tags_config.tags)
}

#[tokio::main]
async fn main() {
    let base_path = Path::new(".");
    let (recipes, tags) = load_recipes(base_path);

    println!("Loaded {} recipes", recipes.len());
    for recipe in &recipes {
        println!("  - {} {:?}", recipe.name, recipe.tags);
    }

    let tera = Tera::new("templates/**/*").expect("Failed to load templates");

    let state = Arc::new(AppState { recipes, tags, tera });

    let app = Router::new()
        .route("/", get(handlers::index))
        .route("/search", get(handlers::search))
        .route("/filter", get(handlers::filter))
        .route("/random", get(handlers::random))
        .nest_service("/static", ServeDir::new("static"))
        .fallback_service(ServeDir::new("."))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
