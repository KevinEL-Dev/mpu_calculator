// this will be where we create operations for getting and posting information
use axum::extract::{Json, State, Query,Form};
use axum::{
    response::{IntoResponse, Redirect, Response,Html},
    http::{StatusCode,HeaderMap,Uri,header}
};
use password_auth::{generate_hash};
use serde::{Serialize,Deserialize};
use serde_json::{Value,json};
use std::collections::HashMap;
use futures_util::TryStreamExt;
use crate::views::{render_measurement_unit_search,render_meal_search,render_source_search};
use sqlx::{
    Row,
    Pool,
    Sqlite
};
use crate::web::AppState;
// pub struct AppState{
//     pub db: Pool<Sqlite>
// }
#[derive(sqlx::FromRow,Serialize)]
pub struct FoundSources {
    id: i64,
    name: String,
    brand: String
}
#[derive(sqlx::FromRow,Serialize)]
pub struct FoundMeasurements {
    id: i64,
    name: String
}
#[derive(Debug,Deserialize)]
pub struct SearchSourcesParam {
    source_name: String
}
#[derive(Debug,Deserialize)]
pub struct SearchMealParam {
    pattern: String
}
#[derive(sqlx::FromRow,Serialize)]
pub struct FoundMeals {
    id: i64,
    name: String
}
#[derive(Debug,Deserialize)]
pub struct SearchMeasurementUnitParam {
    measurement_unit_name: String
}
#[derive(Debug,Deserialize)]
pub struct CreateIngredient {
    meal_id: i64,
    name: String,
    source_name: String,
    amount: i64,
    measurement_unit_name: String,
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
    measurement_unit_name: String,
}
pub struct Credientals {
    username: String,
    password: String,
}
#[derive(Debug,Deserialize)]
pub struct RegisterUser {
    username: String,
    password: String
}
#[derive(Debug,Deserialize)]
pub struct CreateMeasurementUnit {
    name: String,
}
static MIN_PASSWORD_LEN: usize = 14;

pub async fn create_source(State(state): State<AppState>,Form(source_form): Form<CreateSource>) -> Html<&'static str> {
    let measurement_unit = sqlx::query_as::<_, FoundMeasurements>("SELECT id, name FROM measurement_unit WHERE name LIKE '%' || $1 || '%'")
        .bind(source_form.measurement_unit_name)
        .fetch_one(&state.pool).await.unwrap();
    sqlx::query("INSERT INTO source (name, brand, price, servings_per_container,serving_size,measurement_unit_id) Values ($1, $2, $3, $4, $5, $6)")
        .bind(source_form.name)
        .bind(source_form.brand)
        .bind(source_form.price)
        .bind(source_form.servings_per_container)
        .bind(source_form.serving_size)
        .bind(measurement_unit.id)
        .execute(&state.pool).await.unwrap();
    Html("<p>New source added!</p>")
}

pub async fn create_measurement_unit(State(state): State<AppState>,Form(form): Form<CreateMeasurementUnit>) -> Html<&'static str>{
    sqlx::query("INSERT INTO measurement_unit (name) Values ($1)")
        .bind(form.name)
        .execute(&state.pool).await.unwrap();
    Html("<p>New Measurement Unit added!</p>")
}
pub async fn create_meal(State(state): State<AppState>,Form(form): Form<CreateMeal>){
    sqlx::query("INSERT INTO meal (name) Values ($1)")
        .bind(form.name)
        .execute(&state.pool).await.unwrap();
}
pub async fn create_ingredient(State(state): State<AppState>,Form(payload): Form<CreateIngredient>){
    let source_id = search_one_source(&state.pool,payload.source_name).await;
    let ingredient_price = get_ingredient_price(&state.pool,source_id,payload.amount).await;
    let measurement_id = search_one_measurement_unit(&state.pool,payload.measurement_unit_name).await;
    let result = sqlx::query("INSERT INTO ingredient (name, source_id, amount, measurement_unit_id, price) Values ($1, $2, $3, $4, $5)")
        .bind(payload.name)
        .bind(source_id)
        .bind(payload.amount)
        .bind(measurement_id)
        .bind(ingredient_price)
        .execute(&state.pool).await.unwrap();
    // search for created ingredient and get the id
    let new_id = result.last_insert_rowid();
    // then create the mti mapping, passing payload.meal_id and new_ingredient.id
    create_meal_to_ingredient(&state.pool,payload.meal_id,new_id).await;
}
pub async fn create_meal_to_ingredient(db: &Pool<Sqlite>,meal_id: i64,ingredient_id: i64){
    sqlx::query("INSERT INTO meal_to_ingredient (meal_id, ingredient_id) Values ($1, $2)")
        .bind(meal_id)
        .bind(ingredient_id)
        .execute(db).await.unwrap();
}

pub async fn get_meal_price(db: &Pool<Sqlite>,meal_id: i64) -> f64{

    let mut ingredient_ids = vec![];
    let mut rows = sqlx::query("SELECT ingredient_id FROM meal_to_ingredient WHERE meal_id=?")
        .bind(meal_id)
        .fetch(db);

    while let Some(row) = rows.try_next().await.unwrap() {
        let ingredient_id: i64 = row.try_get("ingredient_id").unwrap();
        ingredient_ids.push(ingredient_id);
    }

    let mut prices: f64 = 0.0;
    for id in ingredient_ids {
        let mut rows = sqlx::query("SELECT price FROM ingredient WHERE id=?")
            .bind(id)
            .fetch(db);
        while let Some(row) = rows.try_next().await.unwrap() {
            let price: f64 = row.try_get("price").unwrap();
            prices += price;
        }
    }
    prices
}
pub async fn get_ingredient_price(db: &Pool<Sqlite>, source_id: i64, amount: i64) -> f64{
    let row: (f64,f64) = sqlx::query_as("SELECT price, total_weight_of_container FROM source WHERE id=?")
        .bind(source_id)
        .fetch_one(db).await.unwrap();
    (row.0 / row.1 ) * (amount as f64)
}
// ### get request that searches for sources get_source
// payload should look like the currently typed field. so everytime the field updates with a new character, make a get request and return the similar values.
//
// ```SELECT id, name FROM source WHERE name LIKE '%pattern%'```
//
// return a list of potential sources they may be interested in
pub async fn search_sources(State(state): State<AppState>, Query(params): Query<SearchSourcesParam>) -> impl IntoResponse{
    let sources = sqlx::query_as::<_, FoundSources>("SELECT id, name, brand FROM source WHERE name LIKE '%' || $1 || '%'")
        .bind(params.source_name)
        .fetch_all(&state.pool).await.unwrap();
    let mut res = String::new();
    for source in sources {
        let option = format!("<option value=\"{}\"></option>",source.name);
        res += &option;
    }
    Html(render_source_search(&res).into_string())
}
pub async fn search_measurement_units(State(state): State<AppState>, Query(params): Query<SearchMeasurementUnitParam>) -> impl IntoResponse{
    let measurements = sqlx::query_as::<_, FoundMeasurements>("SELECT id, name FROM measurement_unit WHERE name LIKE '%' || $1 || '%'")
        .bind(params.measurement_unit_name)
        .fetch_all(&state.pool).await.unwrap();
    let mut res = String::new();
    for measurement in measurements {
        let option = format!("<option value=\"{}\"></option>",measurement.name);
        res += &option;
    }
    Html(render_measurement_unit_search(&res).into_string())
}
pub async fn get_ingredients_for_meal(db: &Pool<Sqlite>,meal_id: i64) -> Vec<(String, f64)>{

    let mut ingredient_ids = vec![];
    let mut rows = sqlx::query("SELECT ingredient_id FROM meal_to_ingredient WHERE meal_id=?")
        .bind(meal_id)
        .fetch(db);

    while let Some(row) = rows.try_next().await.unwrap() {
        let ingredient_id: i64 = row.try_get("ingredient_id").unwrap();
        ingredient_ids.push(ingredient_id);
    }

    let mut list_of_ingredients: Vec<(String,f64)> = Vec::new();
    for id in ingredient_ids {
        let mut rows = sqlx::query("SELECT name, price FROM ingredient WHERE id=?")
            .bind(id)
            .fetch(db);
        while let Some(row) = rows.try_next().await.unwrap() {
            let price: f64 = row.try_get("price").unwrap();
            let name: String = row.try_get("name").unwrap();
            list_of_ingredients.push((name,price));
        }
    }
    list_of_ingredients
}
pub async fn search_meals(State(state): State<AppState>, Query(params): Query<SearchMealParam>) -> impl IntoResponse{
    let meals = sqlx::query_as::<_, FoundMeals>("SELECT id, name FROM meal WHERE name LIKE '%' || $1 || '%'")
        .bind(params.pattern)
        .fetch_all(&state.pool).await.unwrap();
    let mut res = String::new();
    let mut counter = 0;
    for meal in meals {
        let meal_price = get_meal_price(&state.pool,meal.id).await;
        let ingredients_in_meal = get_ingredients_for_meal(&state.pool,meal.id).await;
        let mut html_ingredient_list = String::new();
        for ingredient in ingredients_in_meal {
            let ingredient_paragraph = format!("
                    <p> {}, ingredient price: ${}</p>
                ",
                ingredient.0,
                ingredient.1,
                );
            html_ingredient_list += &ingredient_paragraph;
        }
        let form = format!("
            <div style=\"border: 2px solid black;padding: 20px;\">
                <h2>{}</h2>
                <p>Total meal price: ${}</p>
                <h3>Ingredient List</h3>
                {}
                <form hx-post=\"/create_ingredient\" hx-target=\"#form{}-result\">
                    <input type=\"hidden\" name=\"meal_id\" value=\"{}\">
                    <label for=\"name\">Ingredient name </label>
                    <input type=\"text\" name=\"name\">
                    <label for=\"source_name\">Source name </label>
                    <input class=\"form-control\" type=\"text\" list=\"source{}\" name=\"source_name\" hx-get=\"/search_sources\" hx-params=\"*\" hx-trigger=\"input changed delay:500ms, keyup[key=='Enter'], load\" hx-target=\"#source{}\" hx-swap=\"innerHTML\">
                    <datalist id=\"source{}\"></datalist>
                    <label for=\"amount\">Amount </label>
                    <input type=\"number\" name=\"amount\">
                    <label for=\"measurement_unit_name\">Measurement Unit </label>
                    <input class=\"form-control\" type=\"text\" list=\"measurements{}\" name=\"measurement_unit_name\" hx-get=\"/search_measurement_units\" hx-params=\"*\" hx-trigger=\"input changed delay:500ms, keyup[key=='Enter'], load\" hx-target=\"#measurements{}\" hx-swap=\"innerHTML\">
                    <datalist id=\"measurements{}\"></datalist>
                    <input type=\"submit\" value=\"Add ingredient\">
                </form>
                <div id=\"form{}-result\"></div>
            </div>
            ",
            meal.name,
            meal_price,
            html_ingredient_list,
            counter,
            meal.id,
            counter,
            counter,
            counter,
            counter,
            counter,
            counter,
            counter,
            );
        let meal_info = format!("{}",form);
        res += &meal_info;
        counter += 1;
    }
    Html(render_meal_search(&res).into_string())
}
pub async fn search_one_source(db: &Pool<Sqlite>, pattern: String) -> i64{

    let sources = sqlx::query_as::<_, FoundSources>("SELECT id, name, brand FROM source WHERE name LIKE '%' || $1 || '%'")
        .bind(pattern)
        .fetch_one(db).await.unwrap();
    sources.id
}
pub async fn search_one_measurement_unit(db: &Pool<Sqlite>, pattern: String) -> i64{

    let measurement = sqlx::query_as::<_, FoundMeasurements>("SELECT id, name FROM measurement_unit WHERE name LIKE '%' || $1 || '%'")
        .bind(pattern)
        .fetch_one(db).await.unwrap();
    measurement.id
}
pub async fn register (State(state): State<AppState>, Form(register_form): Form<RegisterUser>) -> impl IntoResponse{
    // check first if a user already exist
    if let Ok(user) = sqlx::query("SELECT username FROM user WHERE username LIKE '%' || $1 || '%'")
        .bind(register_form.username.clone())
        .fetch_one(&state.pool).await {
        return (
                StatusCode::NOT_FOUND,
                [(header::CONTENT_TYPE, "text/plain")],
                "This username is already taken"
        )
    }else{
        if register_form.password.len() < MIN_PASSWORD_LEN {
            return (
                StatusCode::BAD_REQUEST,
                [(header::CONTENT_TYPE, "text/plain")],
                "Password does not meet complexity requirements: minimum is 14 chars."
            )
        }
        // hash the users password
        let hashed_password = generate_hash(register_form.password);

        sqlx::query("INSERT INTO user (username, password) Values ($1, $2)")
            .bind(register_form.username)
            .bind(hashed_password)
            .execute(&state.pool).await.unwrap();
        (
            StatusCode::ACCEPTED,
            [(header::CONTENT_TYPE, "text/plain")],
            "User created"
        )

    }
}
