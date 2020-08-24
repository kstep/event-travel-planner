use diesel::prelude::*;
use actix_web::{get};
use actix_web::web::{Data, Json, ServiceConfig};
use crate::db::DbPool;
use crate::result::JsResult;
use crate::models::*;
use crate::schema::*;

#[get("/events/statuses")]
async fn get_event_statuses(pool: Data<DbPool>) -> JsResult<Vec<EventStatus>> {
    use self::event_statuses::dsl::*;
    let conn = pool.get()?;
    let results = event_statuses.load(&*conn)?;
    Ok(Json(results))
}

#[get("/events")]
async fn get_events(pool: Data<DbPool>) -> JsResult<Vec<Event>> {
    use self::events::dsl::*;
    let conn = pool.get()?;
    let results = events.load(&*conn)?;
    Ok(Json(results))
}

#[get("/festivals")]
async fn get_festivals(pool: Data<DbPool>) -> JsResult<Vec<Festival>> {
    use self::festivals::dsl::*;
    let conn = pool.get()?;
    let results = festivals.load(&*conn)?;
    Ok(Json(results))
}

pub fn setup(cfg: &mut ServiceConfig) {
    cfg.service(get_event_statuses)
        .service(get_events)
        .service(get_festivals);
}