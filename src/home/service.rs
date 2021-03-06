// use crate::todo::{Todo, TodoRequest};
use crate::middlewares::authorize::UserIdentity;
use actix_web::{HttpRequest,delete, get, post, put, web, HttpResponse, Responder};
use sqlx::PgPool;
use chrono::Utc;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use crate::home::{LoginRequest,HomeUser,Claims,Token};
use crate::utils;

#[get("/index")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body(r#"
        home/index
    "#
    )
}

///test wasm
#[derive(Serialize, Deserialize)]
pub struct UserInfo{
    pub loginid:String,
    pub passwd:String,
}
#[post("/formlogin1")]
pub async fn formlogin1(f: web::Json<UserInfo>) -> impl Responder {
    let res=f.into_inner();
    HttpResponse::Ok().json(res)
}
#[post("/formlogin")]
pub async fn formlogin() -> impl Responder {
    let res=UserInfo{
        loginid:String::from("aaaa"),
        passwd:String::from("xxxx"),
    };
    HttpResponse::Ok().json(res)
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
                let token:Token=utils::get_token(res, 60000);
                HttpResponse::Ok().json(token)
            }
        },
        _ => HttpResponse::BadRequest().body("*")
    }
}


#[post("/profile")]
pub async fn profile( req: HttpRequest,db_pool: web::Data<PgPool>) -> impl Responder {
    let mut loginid=String::new();
    if let Some(ah) = req.headers().get("Authorization") {
        let user:Claims=utils::from_token(String::from(ah.to_str().unwrap()));
        loginid=user.id;
    }
    let result = HomeUser::userinfo(loginid,db_pool.get_ref()).await;
    match result {
        Ok(res) => {
           
            let tempuser=res.loginid.clone();
            if tempuser.is_empty() {
                HttpResponse::BadRequest().body("no user")
            }else{
                // let token:Token=utils::get_token(res, 99999999999);
                HttpResponse::Ok().json(res)
            }
           
        },
        Err(e)=>{
            println!("err :{}",e);
            HttpResponse::BadRequest().body("*")
        },
        _ => HttpResponse::BadRequest().body("*")
    }
}

#[post("/test")]
pub async fn test( id:UserIdentity) -> impl Responder {
    HttpResponse::Ok().json(id.claims.id)
}



#[post("/test1")]
pub async fn test1( req: HttpRequest) -> impl Responder {
    let mut loginid=String::from("*");
    if let Some(ah) = req.headers().get("Authorization") {
        // let user:Claims=utils::decode_token(ah.to_str().unwrap()).unwrap();
        match utils::decode_token(ah.to_str().unwrap()){
            Ok(claims)=>{
                loginid=claims.id;
            },
            Err(e)=>{
                println!("err:{}",e);
            }
        }
        
    }
    HttpResponse::Ok().json(loginid)
}



