#[macro_use]
extern crate diesel;

extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub mod models;
pub mod schema;

use models::{NewPoem, Poems};
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

// pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
//     use schema::poems;

//     let new_post = NewPoem { title, body };

//     diesel::insert_into(poems::table)
//         .values(&new_post)
//         .get_result(conn)
//         .expect("Error saving new post")
// }

fn main() -> Result<(), Box<dyn error::Error>> {
    use schema::poems;
    let path = "./guwen/test.json";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut lines = buffered.lines();
    // lines.next().unwrap()?;
    let mut test_target = lines.next().unwrap()?;
    let mut v: NewPoem = serde_json::from_str(test_target.as_str())?;
    println!("v:{:?}", v);

    let connection = establish_connection();
    let _ = diesel::insert_into(poems::table)
        .values(&v)
        .get_result::<Poems>(&connection)
        .expect("Error saving new post");
    // println!("{:?}", title);

    Ok(())
}
