use crate::handlers::depth_handler;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/depths").route(web::get().to(depth_handler::get_depths)));
}
