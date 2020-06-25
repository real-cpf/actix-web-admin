#[macro_use]
extern crate log;
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;
use actix_web::{web, App, HttpResponse, HttpServer, Responder,http};
use actix_cors::Cors;
use sqlx::PgPool;
use anyhow::Result;

mod home;

// default / handler
async fn index() -> impl Responder {
    HttpResponse::Ok().body(r#"
        Welcome to Index.
    "#
    )
}



#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    // this will enable us to keep application running during recompile: systemfd --no-pid -s http::5000 -- cargo watch -x run
    let mut listenfd = ListenFd::from_env();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    println!("{}",&database_url);
    // PgPool::builder()
    //     .max_size(5) // maximum number of connections in the pool
    //     .build(env::var("DATABASE_URL")?).await?;
    let db_pool = PgPool::new(&database_url).await?;
    // .wrap(todo::redirect::CheckLogin)
    let mut server = HttpServer::new(move || {
        App::new()
                .wrap(Cors::new() // <- Construct CORS middleware builder
                .allowed_origin("http://localhost:9528")
                .allowed_origin("http://localhost:9000")
                .allowed_methods(vec!["GET", "POST","PUT"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600)
                .finish()
            )
            .data(db_pool.clone()) // pass database pool to application so we can access it inside handlers
            .route("/", web::get().to(index))
            .service(web::scope("/home").configure(home::init))
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("HOST is not set in .env file");
            let port = env::var("PORT").expect("PORT is not set in .env file");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    info!("Starting server");
    server.run().await?;

    Ok(())
}
