/* Import macros and others */
use crate::schema::*;

/* For beeing able to serialize */
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Poems {
    pub id: i32,
    pub _id: Option<String>,
    pub title: String,
    pub dynasty: Option<String>,
    pub writer: Option<String>,
    pub poemtype: Option<Vec<String>>,
    pub content: Option<String>,
    pub remark: Option<String>,
    pub translation: Option<String>,
    pub shangxi: Option<String>,
    // pub poemtype: Vec<String>,
}

#[derive(Debug, Insertable, AsChangeset, Deserialize)]
#[table_name = "poems"]
pub struct NewPoem {
    pub _id: Option<String>,
    pub title: String,
    pub dynasty: Option<String>,
    pub writer: Option<String>,
    pub poemtype: Vec<String>,
    pub content: Option<String>,
    pub remark: Option<String>,
    pub translation: Option<String>,
    pub shangxi: Option<String>,
}
