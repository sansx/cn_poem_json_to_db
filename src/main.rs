#[macro_use]
extern crate diesel;

extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use std::error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

pub mod models;
pub mod schema;

use models::{Author, NewPoem, NewSentence, Poems, ResAuthor, ResPoems, Sentence};
use schema::authors;
use schema::poems;
use schema::sentence;

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

pub fn save_poems(conn: PgConnection, path: &str) -> Result<(), Box<dyn error::Error>> {
    // let path = "./guwen/guwen0-1000.json";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let lines = buffered.lines();
    for poem in lines {
        let res = poem.unwrap();
        let v: Poems = serde_json::from_str(&res.as_str().replace("type", "poemtype"))?;
        let _ = diesel::insert_into(poems::table)
            .values(&NewPoem::from(v))
            .get_result::<ResPoems>(&conn)
            .expect("Error saving new post");
    }
    Ok(())
}

#[allow(non_snake_case)]
fn save_author(conn: PgConnection, path: &str) -> Result<(), Box<dyn error::Error>> {
    use schema::authors::dsl::{self, detailintro, headimageurl, simpleintro};

    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut lines = buffered.lines();
    let res = lines
        .map(|line| {
            let ResAuthor {
                name,
                headImageUrl,
                simpleIntro,
                detailIntro,
            } = serde_json::from_str(line.unwrap().as_str()).unwrap();
            (
                dsl::name.eq(name),
                headimageurl.eq(headImageUrl),
                simpleintro.eq(simpleIntro),
                detailintro.eq(detailIntro),
            )
        })
        .collect::<Vec<_>>();

    diesel::insert_into(authors::table)
        .values(res)
        .get_result::<Author>(&conn)
        .expect("Error saving new post");
    Ok(())
}

fn save_sentence(conn: PgConnection, path: &str) -> Result<(), Box<dyn error::Error>> {
    use schema::sentence::dsl;

    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    // let mut lines = buffered.lines();
    let res = buffered
        .lines()
        .take(1000)
        .map(|line| {
            let NewSentence { name, from } = serde_json::from_str(line.unwrap().as_str()).unwrap();
            (dsl::name.eq(name), dsl::from.eq(from))
        })
        .collect::<Vec<_>>();

    diesel::insert_into(sentence::table)
        .values(res)
        .get_result::<Sentence>(&conn)
        .expect("Error saving new post");
    Ok(())
}

fn get_filen_names(dir: &str) -> Vec<String> {
    let paths = fs::read_dir(dir).unwrap();
    paths
        .map(|path| path.unwrap().path().display().to_string())
        .collect()
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let start = Instant::now();
    let connection = establish_connection();
    // save_to_poems(connection)?;
    let res = get_filen_names("./guwen");
    save_sentence(connection, "./sentence/sentence1-10000.json")?;
    // println!("{:?}", res);
    // save_to_author(connection, "./author/writer0-1000.json")?;
    println!("Time elapsed : {:?}", start.elapsed());
    Ok(())
}
