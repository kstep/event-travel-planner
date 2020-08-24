use crate::db::DbPool;
use crate::models::*;
use crate::result::{JsResult, Result};
use crate::schema::*;
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json, Path, ServiceConfig};
use actix_web::{get, post, put, Error, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct Empty;

impl Future for Empty {
    type Output = std::result::Result<HttpResponse, Error>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(Ok(HttpResponse::new(StatusCode::NO_CONTENT)))
    }
}
impl Responder for Empty {
    type Error = Error;
    type Future = Empty;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        Empty
    }
}

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

#[get("/events/{pk}")]
async fn get_event(pool: Data<DbPool>, pk: Path<i32>) -> JsResult<Event> {
    use self::events::dsl::*;
    let conn = pool.get()?;
    let result = events.filter(id.eq(*pk)).get_result(&*conn)?;
    Ok(Json(result))
}

#[get("/festivals")]
async fn get_festivals(pool: Data<DbPool>) -> JsResult<Vec<Festival>> {
    use self::festivals::dsl::*;
    let conn = pool.get()?;
    let results = festivals.load(&*conn)?;
    Ok(Json(results))
}

#[get("/festivals/{pk}")]
async fn get_festival(pool: Data<DbPool>, pk: Path<i32>) -> JsResult<Festival> {
    use self::festivals::dsl::*;
    let conn = pool.get()?;
    let result = festivals.filter(id.eq(*pk)).get_result(&*conn)?;
    Ok(Json(result))
}

#[post("/festivals")]
async fn post_festival(pool: Data<DbPool>, festival: Json<NewFestival>) -> Result<Empty> {
    let conn = pool.get()?;
    diesel::insert_into(festivals::table)
        .values(&*festival)
        .execute(&*conn)?;
    Ok(Empty)
}

#[post("/festivals")]
async fn post_event(pool: Data<DbPool>, event: Json<NewEvent>) -> Result<Empty> {
    let conn = pool.get()?;
    diesel::insert_into(events::table)
        .values(&*event)
        .execute(&*conn)?;
    Ok(Empty)
}

#[put("/events/{pk}")]
async fn put_event(pool: Data<DbPool>, pk: Path<i32>, event: Json<NewEvent>) -> Result<Empty> {
    use self::events::dsl::*;
    let conn = pool.get()?;
    diesel::update(events.filter(id.eq(*pk)))
        .set(&*event)
        .execute(&*conn)?;
    Ok(Empty)
}

#[put("/festivals/{pk}")]
async fn put_festival(
    pool: Data<DbPool>,
    pk: Path<i32>,
    festival: Json<NewFestival>,
) -> Result<Empty> {
    use self::festivals::dsl::*;
    let conn = pool.get()?;
    diesel::update(festivals.filter(id.eq(*pk)))
        .set(&*festival)
        .execute(&*conn)?;
    Ok(Empty)
}

pub fn setup(cfg: &mut ServiceConfig) {
    cfg.service(get_event_statuses)
        .service(get_events)
        .service(get_festivals)
        .service(get_event)
        .service(get_festival)
        .service(post_festival)
        .service(post_event)
        .service(put_festival)
        .service(put_event);
}
