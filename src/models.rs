use diesel::prelude::*;
use serde::Deserialize;

use crate::schema::links;


#[derive(Queryable, Selectable, Deserialize)]
#[diesel(table_name = links)]
pub struct Link {
    pub id: i32,
    pub salt: String,
    pub link: String,
}


#[derive(Insertable)]
#[diesel(table_name = links)]
pub struct NewLink {
    pub salt: String,
    pub link: String,
}


#[derive(Deserialize, Hash)]
pub struct ShortURL {
    pub url: String
}
