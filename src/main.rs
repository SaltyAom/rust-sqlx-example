mod models;
mod routes;

use std::env;
use dotenv::dotenv;
use anyhow::Result;

use actix_web::{ HttpServer, App, middleware };

use sqlx::postgres::PgPoolOptions;

use crate::routes::routes;

#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Database URL");
    let connection_pool = PgPoolOptions::new().connect(&database_url).await?;

    HttpServer::new(move || {
        App::new()
            .wrap(
                middleware::Compress::default()
            )
            .data(connection_pool.clone())
            .configure(routes)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
        .ok();
    
    Ok(())
}