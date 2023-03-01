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
    HttpResponse::Ok().json(serde_json::json!({
        "X-Hasura-Role": std::env::var("HASURA_ROLE").unwrap(),
    }))
}
