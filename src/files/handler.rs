use actix_web::web;
use crate::files::service::{list,upload,download};


pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(list);
    cfg.service(upload);
    cfg.service(download);
}