// use crate::todo::{Todo, TodoRequest};
use actix_web::{HttpRequest,delete, get, post, put, web, HttpResponse, Responder};
use sqlx::PgPool;
use chrono::Utc;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use crate::home::{LoginRequest,HomeUser};


#[get("/index")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body(r#"
        home/index
    "#
    )
}



#[post("/login")]
pub async fn login( login: web::Json<LoginRequest>,db_pool: web::Data<PgPool>) -> impl Responder {
    let result = HomeUser::login(login.into_inner(),db_pool.get_ref()).await;
    match result {
        Ok(res) => {
           
            let tempuser=res.loginid.clone();
            if tempuser.is_empty() {
                HttpResponse::BadRequest().body("*")
            }else{
                // let token:Token=utils::get_token(res, 99999999999);
                HttpResponse::Ok().json("token")
            }
           
        },
        _ => HttpResponse::BadRequest().body("*")
    }
}