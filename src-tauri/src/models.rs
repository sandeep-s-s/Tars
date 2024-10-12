use crate::schema::collections;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable)]
#[diesel(table_name = collections)]
pub struct NewCollection {
    pub name: String,
    pub uuid: String,
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::collections)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Collection {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub uuid: String,
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Integer)]
    pub sort: i32,
    #[diesel(sql_type = Text)]
    pub create_date: String,
    #[diesel(sql_type = Text)]
    pub update_date: String,
    #[diesel(sql_type = Bool)]
    pub is_open: bool,
}
