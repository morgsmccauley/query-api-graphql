use juniper::{EmptySubscription, RootNode};
use juniper::{FieldError, FieldResult};

use sqlx::{FromRow, Pool, Postgres};

pub struct Context {
    pub db: Pool<Postgres>,
}

impl juniper::Context for Context {}

#[derive(juniper::GraphQLScalarValue)]
#[graphql(transparent)]
struct JsonString(String);

#[derive(FromRow, juniper::GraphQLObject)]
#[allow(dead_code)]
struct IndexerStorage {
    function_name: String,
    key_name: String,
    value: String,
}

pub struct Query;

#[juniper::graphql_object(context = Context)]
impl Query {
    async fn get(
        context: &Context,
        function_name: String,
        key: String,
    ) -> FieldResult<Option<JsonString>> {
        match sqlx::query_as::<_, IndexerStorage>(
            "SELECT * FROM indexer_storage WHERE function_name = $1 AND key_name = $2",
        )
        .bind(function_name)
        .bind(key)
        .fetch_optional(&context.db)
        .await
        {
            Ok(Some(storage)) => Ok(Some(JsonString(storage.value))),
            Ok(None) => Ok(None),
            Err(err) => Err(FieldError::from(err)),
        }
    }
}

pub struct Mutation;

#[juniper::graphql_object(context = Context)]
impl Mutation {
    async fn set(
        context: &Context,
        function_name: String,
        key: String,
        data: JsonString,
    ) -> FieldResult<JsonString> {
        match sqlx::query_as::<_, IndexerStorage>(
            "INSERT INTO indexer_storage (function_name, key_name, value) VALUES ($1, $2, $3) RETURNING *"
        )
            .bind(function_name)
            .bind(key)
            .bind(data.0)
            .fetch_one(&context.db)
            .await {
                Ok(storage) => Ok(JsonString(storage.value)),
                Err(err) => Err(FieldError::from(err))
            }
    }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {}, EmptySubscription::new())
}

#[test]
fn schema_snapshot() {
    insta::assert_snapshot!(create_schema().as_schema_language())
}
