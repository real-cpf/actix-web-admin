use serde::{Serialize, Deserialize};
use actix_web::{HttpResponse, HttpRequest, Responder, Error};
use futures::future::{ready, Ready};
use sqlx::{PgPool, FromRow, Row};
use sqlx::postgres::PgRow;
use anyhow::Result;

#[derive(Serialize, Deserialize)]
pub struct LoginRequest{
    pub loginid:String,
    pub passwd:String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id:String,
    pub sub: String,
    pub company: String,
    pub px:i64,
    pub py:i64,
    pub dept:i16,
    pub iat: i64,
    pub exp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub token:String,
}

#[derive(Serialize, Deserialize,FromRow)]
pub struct HomeUser{
    pub displayname:String,
    pub loginid:String,
    pub rolex:i64,
    pub roley:i64,
    pub deptid:i16,
    pub passwd:String,
    pub email:String,    
}

impl Responder for HomeUser{
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;
    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(
            HttpResponse::Ok()
                .content_type("application/json")
                .body(body)
        ))
    }
}

impl HomeUser{

    pub async fn login(u_login:LoginRequest,pool :&PgPool)->Result<HomeUser>{
        let recs=sqlx::query(r#" SELECT displayname, loginid, rolex, roley, deptid, email FROM public.user_table where loginid =$1 and passwd =$2        "#)
        .bind(u_login.loginid)
        .bind(u_login.passwd)
        .map(|row: PgRow| {
            HomeUser {
                displayname:row.get(0),
                rolex:row.get(2),
                roley:row.get(3),
                passwd:String::from("******"),
                deptid:row.get(4),
                email:row.get(5),
                loginid:row.get(1),
            }
         }).fetch_one(pool).await;

         Ok(recs.unwrap_or(HomeUser {
            displayname:String::from(""),
            rolex:0_i64,
            roley:0_i64,
            passwd:String::from(""),
            deptid:0_i16,
            email:String::from(""),
            loginid:String::from(""),
        }))
         
    }

    pub async fn userinfo(loginid:String,pool :&PgPool)->Result<HomeUser>{
        let recs=sqlx::query(r#" SELECT displayname, loginid, rolex, roley, deptid, email FROM public.user_table where loginid =$1  "#)
        .bind(loginid.trim())
        .map(|row: PgRow| {
            HomeUser {
                displayname:row.get(0),
                rolex:row.get(2),
                roley:row.get(3),
                passwd:String::from("******"),
                deptid:row.get(4),
                email:row.get(5),
                loginid:row.get(1),
            }
         }).fetch_one(pool).await;

         Ok(recs.unwrap_or(HomeUser {
            displayname:String::from(""),
            rolex:0_i64,
            roley:0_i64,
            passwd:String::from(""),
            deptid:0_i16,
            email:String::from(""),
            loginid:String::from(""),
        }))
         
    }



}