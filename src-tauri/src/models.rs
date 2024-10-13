use crate::schema::{collections, requests};
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


#[derive(Insertable)]
#[diesel(table_name = requests)]
pub struct NewRequest {
    pub name: String,
    pub uuid: String,
    pub request_data: String,
    pub collection_id: i32,
}


#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::requests)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Requests {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub uuid: String,
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Text)]
    pub request_data: String,
    #[diesel(sql_type = Integer)]
    pub sort: i32,
    #[diesel(sql_type = Text)]
    pub create_date: String,
    #[diesel(sql_type = Text)]
    pub update_date: String,
}