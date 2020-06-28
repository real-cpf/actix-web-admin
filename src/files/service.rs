use actix_web::{HttpRequest,delete, get, post, put, web, HttpResponse, Responder};
use sqlx::PgPool;
use chrono::Utc;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use uuid::Uuid;

use actix_multipart::Multipart;
use sanitize_filename;
use futures::{StreamExt, TryStreamExt};
use crate::middlewares::authorize::UserIdentity;
use crate::files::{Files,FileRequest};





// need auth
#[get("/list")]
pub async fn list( db_pool: web::Data<PgPool>) -> impl Responder {

    let result = Files::filelist(0_i64,db_pool.get_ref()).await;
    match result {
        Ok(res) => {
            HttpResponse::Ok().json(res)
           
        },
        _ => HttpResponse::BadRequest().body("*")
    }
}


#[post("/upload")]
async fn upload(mut payload: Multipart,db_pool: web::Data<PgPool>) -> impl Responder {

    // iterate over multipart stream
    let mut path=String::new();
    let mut fname=String::new();
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        fname=String::from(filename);
        let filepath = format!("./upload/{}-{}", Uuid::new_v4(),sanitize_filename::sanitize(&filename));
        path=filepath.clone();
        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await.unwrap();
        }
        
    }
    // HttpResponse::Ok().json(fname)

    let db_file=Files{
        id :0,
        filename :fname,
        filepath :path,
        filemark :String::from("*"), 
        filepower :0_i64,
    };

    let result =Files::fileupload(db_file, db_pool.get_ref()).await;
    match result {
        Ok(res) => {
            HttpResponse::Ok().json(res)
        },
        Err(e)=>{
            println!("{}",e);
            HttpResponse::BadRequest().body("*")
        },
        _ => HttpResponse::BadRequest().body("*")
    }    
}

// need auth
#[post("/download")]
pub async fn download( file: web::Json<FileRequest>,db_pool: web::Data<PgPool>) -> impl Responder {

    let result = Files::filepath(file.into_inner().id,0_i64,db_pool.get_ref()).await;
    match result {
        Ok(res) => {
            let f_stream=fs::read(res.filepath).unwrap();
            HttpResponse::Ok().content_type("application/octet-stream").body(f_stream)
           
        },
        _ => HttpResponse::BadRequest().body("*")
    }
}