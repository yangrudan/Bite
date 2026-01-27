use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse},
};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tera::{Context, Tera};

use crate::models::Recipe;

pub struct AppState {
    pub recipes: Vec<Recipe>,
    pub tags: Vec<String>,
    pub tera: Tera,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
}

#[derive(Deserialize)]
pub struct FilterQuery {
    pub tag: Option<String>,
}

#[derive(Deserialize)]
pub struct RandomQuery {
    pub count: Option<usize>,
}

fn render_index(
    tera: &Tera,
    recipes: Vec<Recipe>,
    tags: &[String],
    current_tag: &str,
    search_query: &str,
    is_random: bool,
) -> String {
    let mut context = Context::new();
    context.insert("recipes", &recipes);
    context.insert("tags", tags);
    context.insert("current_tag", current_tag);
    context.insert("search_query", search_query);
    context.insert("is_random", &is_random);
    tera.render("index.html", &context).unwrap()
}

pub async fn index(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Html(render_index(
        &state.tera,
        state.recipes.clone(),
        &state.tags,
        "",
        "",
        false,
    ))
}

pub async fn search(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SearchQuery>,
) -> impl IntoResponse {
    let search_term = query.q.unwrap_or_default();
    let filtered: Vec<Recipe> = if search_term.is_empty() {
        state.recipes.clone()
    } else {
        state
            .recipes
            .iter()
            .filter(|r| r.name.contains(&search_term))
            .cloned()
            .collect()
    };

    Html(render_index(
        &state.tera,
        filtered,
        &state.tags,
        "",
        &search_term,
        false,
    ))
}

pub async fn filter(
    State(state): State<Arc<AppState>>,
    Query(query): Query<FilterQuery>,
) -> impl IntoResponse {
    let tag = query.tag.unwrap_or_default();
    let filtered: Vec<Recipe> = if tag.is_empty() {
        state.recipes.clone()
    } else {
        state
            .recipes
            .iter()
            .filter(|r| r.tags.contains(&tag))
            .cloned()
            .collect()
    };

    Html(render_index(
        &state.tera,
        filtered,
        &state.tags,
        &tag,
        "",
        false,
    ))
}

pub async fn random(
    State(state): State<Arc<AppState>>,
    Query(query): Query<RandomQuery>,
) -> impl IntoResponse {
    let count = query.count.unwrap_or(3).min(state.recipes.len());
    let mut rng = rand::thread_rng();
    let mut recipes = state.recipes.clone();
    recipes.shuffle(&mut rng);
    let selected: Vec<Recipe> = recipes.into_iter().take(count).collect();

    Html(render_index(
        &state.tera,
        selected,
        &state.tags,
        "",
        "",
        true,
    ))
}
