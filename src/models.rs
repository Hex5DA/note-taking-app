use diesel::{Queryable, Insertable};
use serde::Deserialize;


use super::schema::note;

#[derive(Deserialize)]
#[derive(Queryable)]
pub struct Note {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub starred: bool,
}

#[derive(Insertable)]
#[table_name="note"]
pub struct NewNote<'a> {
    pub title: &'a str,
    pub content: &'a str,
}