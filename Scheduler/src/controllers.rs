use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use crate::models::{NewSession, Session, create_session, delete_session, update_session, get_sessions};
use crate::DbPool;

pub async fn add_session(
    pool: web::Data<DbPool>,
    session: web::Json<NewSession>,
) -> HttpResponse {
    let conn = pool.get().expect("Couldn't get DB connection from pool");
    match create_session(&conn, session.into_inner()) {
        Ok(session) => HttpResponse::Ok().json(session),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn remove_session(
    pool: web::Data<DbPool>,
    session_id: web::Path<Uuid>,
) -> HttpResponse {
    let conn = pool.get().expect("Couldn't get DB connection from pool");
    match delete_session(&conn, session_id.into_inner()) {
        Ok(_) => HttpResponse::Ok().body("Session deleted"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn modify_session(
    pool: web::Data<DbPool>,
    session_id: web::Path<Uuid>,
    session: web::Json<NewSession>,
) -> HttpResponse {
    let conn = pool.get().expect("Couldn't get DB connection from pool");
    match update_session(&conn, session_id.into_inner(), session.into_inner()) {
        Ok(session) => HttpResponse::Ok().json(session),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn list_sessions(pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get().expect("Couldn't get DB connection from pool");
    match get_sessions(&conn) {
        Ok(sessions) => HttpResponse::Ok().json(sessions),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}