use actix_web::{web, HttpResponse, Scope};
use crate::controllers::{create_session_controller, update_session_controller, delete_session_controller};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/sessions", web::post().to(create_session_controller))
            .route("/sessions/{id}", web::put().to(update_session_controller))
            .route("/sessions/{id}", web::delete().to(delete_session_controller))
    );
}