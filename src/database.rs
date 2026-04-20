use directories::ProjectDirs;
use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use sqlx::{Connection, SqliteConnection};


pub fn database_exists() -> Option<bool> {
    let fname = "/ppmc.sqlite3";
    if let Some(proj_dirs) = ProjectDirs::from("","","ppmc"){
        let data_dir = proj_dirs.data_dir().to_str().unwrap();
        let path = Path::new(&(data_dir.to_owned() + fname)).exists();
        if path {
            return Some(true);
        } else{
            return Some(false);
        }
    }
    None
}
pub fn get_database_path() -> Option<String> {
    if let Some(proj_dirs) = ProjectDirs::from("","","ppmc"){
        let data_dir = proj_dirs.data_dir().to_str().unwrap();
        return Some(data_dir.to_string());
    }
    None
}
pub async fn init_database() -> anyhow::Result<()> {
    let fname = "/ppmc.sqlite3";
    let mut path = get_database_path().expect("Failed to get database path");
    fs::create_dir_all(path.clone()).unwrap();
    path += fname;
    File::create(path.clone()).unwrap();
    let mut conn = SqliteConnection::connect(&path).await?;
    let meal_table_create = "
        CREATE TABLE meal (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        );
    ";
    let measurement_table_create = "
        CREATE TABLE measurement_unit (
            id INTEGER PRIMARY KEY,
            name TEXT UNIQUE NOT NULL
        );
    ";
    let source_table_create = "
        CREATE TABLE source (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            brand TEXT NOT NULL,
            price FLOAT NOT NULL,
            servings_per_container INTEGER NOT NULL,
            serving_size INTEGER NOT NULL,
            measurement_unit_id INT REFERENCES measurement_unit(id) NOT NULL,
            total_weight_of_container INT GENERATED ALWAYS
                AS (servings_per_container * serving_size)
        );
    ";
    let ingredient_table_create = "
        CREATE TABLE ingredient (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            source_id INT REFERENCES source(id) NOT NULL,
            amount INTEGER NOT NULL,
            measurement_unit_id INT REFERENCES measurement_unit(id) NOT NULL,
            price FLOAT NOT NULL
        );
    ";
    let meal_to_ingredient_table_create = "
        CREATE TABLE meal_to_ingredient (
            id INTEGER PRIMARY KEY,
            meal_id INT REFERENCES meal(id) NOT NULL,
            ingredient_id INT REFERENCES ingredient(id) NOT NULL
        );
    ";

    let queries = vec![meal_table_create,measurement_table_create,source_table_create,ingredient_table_create,meal_to_ingredient_table_create];
    let current_table = ["meal table","measuremnt table","source table","ingredient table","mti table"];
    let mut iterator = current_table.iter();
    for query in queries {
        sqlx::raw_sql(query).execute(&mut conn).await?;
        println!("{} created successfully",iterator.next().unwrap())
    }
    Ok(())
}
