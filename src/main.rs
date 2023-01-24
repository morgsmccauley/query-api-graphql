use std::io;

use actix_cors::Cors;
use actix_web::{middleware, web::Data, App, HttpServer};
use dotenv::dotenv;

mod database;
mod schema;
mod services;

use crate::database::create_pool;
use crate::schema::create_schema;
use crate::services::{graphql, graphql_playground};

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port = std::env::var("PORT")
        .expect("PORT must be set")
        .parse::<u16>()
        .expect("PORT must be numeric");

    let schema = create_schema().data(create_pool().await).finish();

    log::info!("starting HTTP server on port {}", port);
    log::info!("GraphiQL playground: http://localhost:{}/graphiql", port);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(graphql)
            .service(graphql_playground)
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
