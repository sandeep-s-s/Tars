use crate::schema::{collections, requests};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Insertable)]
#[diesel(table_name = collections)]
pub struct NewCollection {
    pub name: String,
    pub uuid: String,
    pub is_open: bool,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Deserialize)]
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

// #[diesel(table_name = crate::schema::requests)]
#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Serialize)]
#[diesel(table_name = crate::schema::requests ,belongs_to(Collection))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestObject {
    pub v: String,
    pub endpoint: String,
    pub name: String,
    pub params: Vec<Param>,
    pub headers: Vec<Header>,
    pub method: String,
    pub auth: Auth,
    pub pre_request_script: String,
    pub test_script: String,
    pub body: Body,
}

impl RequestObject {
    pub fn new(name: String, method: String, endpoint: String) -> RequestObject {
        RequestObject {
            v: String::from("1.0"),
            endpoint,
            name,
            params: Vec::new(),
            headers: Vec::new(),
            method,
            auth: Auth::default(),
            pre_request_script: String::new(),
            test_script: String::new(),
            body: Body::default(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Param {
    pub key: String,
    pub value: String,
    pub checked: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub key: String,
    pub value: String,
    pub checked: Option<bool>,
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Auth {
    pub auth_type: String,
    pub auth_active: bool,
    pub username: String,
    pub password: String,
    pub token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    pub mode: String,
    pub form_data: Vec<FormDaum>,
    pub x_www_form_urlencoded: Vec<XWwwFormUrlencoded>,
    pub raw_type: String,
    pub raw: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormDaum {
    pub key: String,
    pub value: String,
    pub checked: Option<bool>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub src: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XWwwFormUrlencoded {
    pub key: String,
    pub value: String,
    pub checked: Option<bool>,
    #[serde(rename = "type")]
    pub type_field: String,
}
