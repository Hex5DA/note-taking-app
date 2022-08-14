use diesel::{Queryable, Insertable, AsChangeset};
use serde::{self, Deserialize, Serialize};

use super::schema::note;

#[derive(Queryable, AsChangeset)]
#[derive(Deserialize, Serialize)]
#[derive(Clone)]
#[table_name="note"]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub content: String,
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

#[derive(Deserialize)]
#[derive(Serialize)]
pub struct OptionalNote {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub starred: Option<bool>,
}
