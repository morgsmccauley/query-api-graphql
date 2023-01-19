use actix_web::{get, route, web, HttpResponse, Responder};
use actix_web_lab::respond::Html;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use sqlx::{Pool, Postgres};

use crate::schema::{Context, Schema};

#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(
    schema: web::Data<Schema>,
    db: web::Data<Pool<Postgres>>,
    data: web::Json<GraphQLRequest>,
) -> impl Responder {
    let context = Context {
        db: db.as_ref().to_owned(),
    };
    let user = data.execute(&schema, &context).await;
    HttpResponse::Ok().json(user)
}
