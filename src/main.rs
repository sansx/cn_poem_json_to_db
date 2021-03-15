#[macro_use]
extern crate diesel;

extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use std::error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

pub mod models;
pub mod schema;

use models::{NewPoem, Poems, ResPoems};
use schema::poems;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let start = Instant::now();
    let path = "./guwen/guwen0-1000.json";
    let connection = establish_connection();
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let lines = buffered.lines();
    for poem in lines {
        let res = poem.unwrap();
        let v: Poems = serde_json::from_str(&res.as_str().replace("type", "poemtype"))?;
        let _ = diesel::insert_into(poems::table)
            .values(&NewPoem::from(v))
            .get_result::<ResPoems>(&connection)
            .expect("Error saving new post");
    }
    // let duration = start.elapsed();

    println!("Time elapsed : {:?}", start.elapsed());
    Ok(())
}
