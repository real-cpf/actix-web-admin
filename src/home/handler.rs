use actix_web::web;
use crate::home::{login,index,profile,test,test1,formlogin,formlogin1};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    cfg.service(login);
    cfg.service(profile);
    cfg.service(test);
    cfg.service(test1);
    cfg.service(formlogin);
    cfg.service(formlogin1);
}