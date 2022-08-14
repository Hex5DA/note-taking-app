use serde::{self, Deserialize, Serialize};
use diesel::{Queryable, Insertable};

use super::schema::note;

#[derive(Deserialize)]
#[derive(Queryable)]
#[derive(Serialize)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub content: String,
    #[serde(default = "bool::default")]
    pub starred: bool,
}

#[derive(Deserialize)]
pub struct JsonNote {
    pub title: String,
    pub content: String,
    #[serde(default = "bool::default")]
    pub starred: bool,
}

#[derive(Insertable)]
#[table_name="note"]
pub struct NewNote<'a> {
    pub title: &'a str,
    pub content: &'a str,
    pub starred: &'a bool,
}