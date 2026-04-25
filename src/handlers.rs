// this will be where we create operations for getting and posting information
use axum::extract::{Json, State, Query,Form};
use axum::{
    response::{IntoResponse, Redirect, Response,Html}
};
use serde::{Serialize,Deserialize};
use serde_json::{Value,json};
use std::collections::HashMap;
use futures_util::TryStreamExt;
use sqlx::{
    Row,
    Pool,
    Sqlite
};
#[derive(Debug,Clone)]
pub struct AppState{
    pub pool: Pool<Sqlite>
}
#[derive(sqlx::FromRow,Serialize)]
pub struct FoundSources {
    id: i64,
    name: String,
    brand: String
}
#[derive(Debug,Deserialize)]
pub struct SearchSourcesParam {
    pattern: String
}
#[derive(Debug,Deserialize)]
pub struct CreateIngredient {
    name: String,
    source_id: i64,
    amount: i64,
    measurement_unit_id: i64,
    price: f64
}

#[derive(Debug,Deserialize)]
pub struct GetIngredientPriceParam {
    source_id: i64,
    amount: i64
}
#[derive(Debug,Deserialize)]
pub struct GetMealPriceParam {
    meal_id: i64
}
#[derive(Debug,Deserialize)]
pub struct CreateMeal {
    name: String
}
#[derive(Debug,Deserialize)]
pub struct CreateMealToIngredient {
    meal_id: i64,
    ingredient_id: i64
}
#[derive(Debug,Deserialize)]
pub struct CreateSource {
    name: String,
    brand: String,
    price: f64,
    servings_per_container: f64,
    serving_size: f64,
    measurement_unit_id: i64,
}
#[derive(Debug,Deserialize)]
pub struct CreateMeasurementUnitBak {
    name: String,
}
#[derive(Debug,Deserialize)]
pub struct CreateMeasurementUnit {
    name: String,
}
pub async fn create_source(State(state): State<AppState>,Form(source_form): Form<CreateSource>) -> Html<&'static str> {
    sqlx::query("INSERT INTO source (name, brand, price, servings_per_container,serving_size,measurement_unit_id) Values ($1, $2, $3, $4, $5, $6)")
        .bind(source_form.name)
        .bind(source_form.brand)
        .bind(source_form.price)
        .bind(source_form.servings_per_container)
        .bind(source_form.serving_size)
        .bind(source_form.measurement_unit_id)
        .execute(&state.pool).await.unwrap();
    Html("<p>New source added!</p>")
}

pub async fn create_measurement_unit(State(state): State<AppState>,Form(form): Form<CreateMeasurementUnit>) -> Html<&'static str>{
    sqlx::query("INSERT INTO measurement_unit (name) Values ($1)")
        .bind(form.name)
        .execute(&state.pool).await.unwrap();
    Html("<p>New Measurement Unit added!</p>")
}
pub async fn create_meal(State(state): State<AppState>,Json(payload): Json<CreateMeal>){
    sqlx::query("INSERT INTO meal (name) Values ($1)")
        .bind(payload.name)
        .execute(&state.pool).await.unwrap();
}
pub async fn create_ingredient(State(state): State<AppState>,Json(payload): Json<CreateIngredient>){
    sqlx::query("INSERT INTO ingredient (name, source_id, amount, measurement_unit_id, price) Values ($1, $2, $3, $4, $5)")
        .bind(payload.name)
        .bind(payload.source_id)
        .bind(payload.amount)
        .bind(payload.measurement_unit_id)
        .bind(payload.price)
        .execute(&state.pool).await.unwrap();
}
pub async fn create_meal_to_ingredient(State(state): State<AppState>,Json(payload): Json<CreateMealToIngredient>){
    sqlx::query("INSERT INTO meal_to_ingredient (meal_id, ingredient_id) Values ($1, $2)")
        .bind(payload.meal_id)
        .bind(payload.ingredient_id)
        .execute(&state.pool).await.unwrap();
}

pub async fn get_meal_price(State(state): State<AppState>, Query(params): Query<GetMealPriceParam>) -> Json<f64>{

    let mut ingredient_ids = vec![];
    let mut rows = sqlx::query("SELECT ingredient_id FROM meal_to_ingredient WHERE meal_id=?")
        .bind(params.meal_id)
        .fetch(&state.pool);

    while let Some(row) = rows.try_next().await.unwrap() {
        let ingredient_id: i64 = row.try_get("ingredient_id").unwrap();
        ingredient_ids.push(ingredient_id);
    }

    let mut prices: f64 = 0.0;
    for id in ingredient_ids {
        let mut rows = sqlx::query("SELECT price FROM ingredient WHERE id=?")
            .bind(id)
            .fetch(&state.pool);
        while let Some(row) = rows.try_next().await.unwrap() {
            let price: f64 = row.try_get("price").unwrap();
            prices += price;
        }
    }
    Json(prices)
}
pub async fn get_ingredient_price(State(state): State<AppState>, Query(params): Query<GetIngredientPriceParam>) -> Json<f64>{
    let row: (f64,f64) = sqlx::query_as("SELECT price, total_weight_of_container FROM source WHERE id=?")
        .bind(params.source_id)
        .fetch_one(&state.pool).await.unwrap();
    Json( (row.0 / row.1 ) * (params.amount as f64) )
}
// ### get request that searches for sources get_source
// payload should look like the currently typed field. so everytime the field updates with a new character, make a get request and return the similar values.
//
// ```SELECT id, name FROM source WHERE name LIKE '%pattern%'```
//
// return a list of potential sources they may be interested in
pub async fn search_sources(State(state): State<AppState>, Query(params): Query<SearchSourcesParam>) -> Json<Vec< FoundSources >>{
    let sources = sqlx::query_as::<_, FoundSources>("SELECT id, name, brand FROM source WHERE name LIKE '%' || $1 || '%'")
        .bind(params.pattern)
        .fetch_all(&state.pool).await.unwrap();
    Json(sources)
}
