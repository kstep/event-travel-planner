use crate::schema::*;

#[derive(Queryable, Serialize)]
pub struct Festival {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub place: Option<String>,
    pub site_url: Option<String>,
    pub facebook_url: Option<String>,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[table_name = "festivals"]
pub struct NewFestival {
    pub name: String,
    pub description: String,
    pub place: Option<String>,
    pub site_url: Option<String>,
    pub facebook_url: Option<String>,
}

#[derive(Queryable, Serialize)]
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

#[derive(Insertable, AsChangeset, Deserialize)]
#[table_name = "events"]
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

#[derive(Queryable, Serialize)]
pub struct EventStatus {
    id: i32,
    name: String,
}
