use actix_web::web;
use crate::home::{login,index,profile,test};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    cfg.service(login);
    cfg.service(profile);
    cfg.service(test);
}