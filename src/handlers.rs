// this will be where we create operations for getting and posting information
//
//
// first i want to be able to allow the user to add a source
//
// i need a post request to allow the user to input information and take that information and update
// the database 
//
//
use axum::extract::{Json, State};
use serde::Deserialize;
use std::collections::HashMap;
use sqlx::{
    Pool,
    Sqlite
};
#[derive(Debug,Clone)]
pub struct AppState{
    pub pool: Pool<Sqlite>
}
#[derive(Debug,Deserialize)]
pub struct CreateSource {
    name: String,
    brand: String,
    price: f32,
    servings_per_container: i64,
    serving_size: i64,
    measurement_unit_id: i64,
}
#[derive(Debug,Deserialize)]
pub struct CreateMeasurementUnit {
    name: String,
}
pub async fn create_source(State(state): State<AppState>,Json(payload): Json<CreateSource>) {
    println!("{:?}",payload);
    println!("{:?}",state);
    sqlx::query("INSERT INTO source (name, brand, price, servings_per_container,serving_size,measurement_unit_id) Values ($1, $2, $3, $4, $5, $6)")
        .bind(payload.name)
        .bind(payload.brand)
        .bind(payload.price)
        .bind(payload.servings_per_container)
        .bind(payload.serving_size)
        .bind(payload.measurement_unit_id)
        .execute(&state.pool).await.unwrap();
}

pub async fn create_measurement_unit(State(state): State<AppState>,Json(payload): Json<CreateMeasurementUnit>){
    println!("{:?}",state);
    sqlx::query("INSERT INTO measurement_unit (name) Values ($1)")
        .bind(payload.name)
        .execute(&state.pool).await.unwrap();
}
