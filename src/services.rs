use actix_web::{get, route, web, HttpRequest, HttpResponse, Responder, Result};
use async_graphql::http::GraphiQLSource;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use crate::schema::Schema;

#[get("/graphiql")]
pub(crate) async fn graphql_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/graphql").finish()))
}

#[route("/graphql", method = "GET", method = "POST")]
pub(crate) async fn graphql(schema: web::Data<Schema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[route("/auth", method = "GET", method = "POST")]
pub(crate) async fn auth(req: HttpRequest) -> impl Responder {
    let role = match req.headers().get("X-Hasura-Role") {
        Some(role_header) => role_header.to_str().unwrap().to_string(),
        None => std::env::var("DEFAULT_HASURA_ROLE").unwrap(),
    };

    HttpResponse::Ok().json(serde_json::json!({
        "X-Hasura-Role": role,
    }))
}
