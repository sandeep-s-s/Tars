use crate::schema::{collections, requests};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Insertable)]
#[diesel(table_name = collections)]
pub struct NewCollection {
    pub name: String,
    pub uuid: String,
}

// #[derive(Queryable, Selectable, Serialize, Deserialize)]
#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq,Serialize, Deserialize)]
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


// #[derive(Queryable, Selectable, Serialize, Deserialize,Associations)]
#[diesel(table_name = crate::schema::requests)]
// #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Serialize)]
#[diesel(belongs_to(Collection))]
// #[diesel(table_name = requests)]
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
    #[diesel(sql_type = Integer)]
    pub collection_id: i32,
}


#[derive(Serialize)]
pub struct CollectionWithRequests {
    #[serde(flatten)]
    pub collection: Collection,
    pub requests: Vec<Requests>,
}



#[derive(Serialize, Deserialize)]
pub struct JsonResponse {
    pub success: bool,
    pub message: String,
    pub status_code: u16,
    pub headers: HashMap<String, String>,
}