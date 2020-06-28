use serde::{Serialize, Deserialize};
use actix_web::{HttpResponse, HttpRequest, Responder, Error};
use futures::future::{ready, Ready};
use sqlx::{PgPool, FromRow, Row};
use sqlx::postgres::PgRow;
use anyhow::Result;


#[derive(Serialize, Deserialize)]
pub struct FileRequest{
    pub id :i64,
}

#[derive(Serialize, Deserialize,FromRow)]
pub struct Files{
    pub id :i32,
	pub filename :String,
	pub filepath :String,
    pub filemark :String, 
    pub filepower :i64,
}

impl Responder for Files{
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

impl Files{

    pub async fn filelist(power:i64,pool :&PgPool)->Result<Vec<Files>>{
        let recs=sqlx::query(r#" SELECT id, filename, filemark FROM public.file_table "#)
        .map(|row: PgRow| {
            Files {
                 id :row.get(0),
                 filename :row.get(1),
                 filepath :String::from("*"),
                 filemark :row.get(2),
                 filepower:0_i64,
            }
         }).fetch_all(pool).await;

        Ok(recs.unwrap())
         
    }

    pub async fn fileupload(file:Files,pool :&PgPool)->Result<FileRequest>{
        let mut tx=pool.begin().await?;
        let res=sqlx::query(r#"
            INSERT INTO public.file_table (filename, filepath, filemark, filepower) VALUES($1, $2, $3, $4) 
            returning currval ('file_table_id_seq'::regclass)
        "#)
        .bind(file.filename)
        .bind(file.filepath)
        .bind(file.filemark)
        .bind(file.filepower)
        .map(|row: PgRow| {
            FileRequest{
                id:row.get(0)
            }
        })
        .fetch_one(&mut tx).await?;
        tx.commit().await?;
        Ok(res)
    }


    pub async fn filepath(id:i64,power:i64,pool :&PgPool)->Result<Files>{
        let recs=sqlx::query(r#" SELECT filepath FROM public.file_table where id=$1 "#)
        .bind(id)
        .map(|row: PgRow| {
            Files {
                 id :0,
                 filename :String::from("*"),
                 filepath :row.get(0),
                 filemark :String::from("*"),
                 filepower:0_i64,
            }
         }).fetch_one(pool).await;

        Ok(recs.unwrap())
         
    }

}