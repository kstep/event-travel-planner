#[cfg(not(target_arch = "wasm32"))]
use crate::backend::schema::*;

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Queryable))]
pub struct Festival {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub place: Option<String>,
    pub site_url: Option<String>,
    pub facebook_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(Insertable, AsChangeset),
    table_name = "festivals"
)]
pub struct NewFestival {
    pub name: String,
    pub description: String,
    pub place: Option<String>,
    pub site_url: Option<String>,
    pub facebook_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Queryable))]
pub struct Event {
    pub id: i32,
    pub festival_id: i32,
    pub status: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub place: Option<String>,
    pub site_url: Option<String>,
    pub facebook_url: Option<String>,
    pub register_url: Option<String>,
    pub register_start_date: Option<i32>,
    pub register_end_date: Option<i32>,
    pub start_date: i32,
    pub end_date: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(Insertable, AsChangeset),
    table_name = "events"
)]
pub struct NewEvent {
    pub festival_id: i32,
    pub status: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub place: Option<String>,
    pub site_url: Option<String>,
    pub facebook_url: Option<String>,
    pub register_url: Option<String>,
    pub register_start_date: Option<i32>,
    pub register_end_date: Option<i32>,
    pub start_date: i32,
    pub end_date: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Queryable))]
pub struct EventStatus {
    id: i32,
    name: String,
}
