use sqlx::{FromRow, Pool, Postgres};

#[derive(serde::Serialize, serde::Deserialize)]
struct JsonString(String);

async_graphql::scalar!(JsonString);

#[derive(FromRow, async_graphql::SimpleObject)]
#[allow(dead_code)]
struct IndexerStorage {
    function_name: String,
    key_name: String,
    value: String,
}

pub struct Query;

#[async_graphql::Object]
impl Query {
    async fn get(
        &self,
        context: &async_graphql::Context<'_>,
        function_name: String,
        key: String,
    ) -> async_graphql::Result<Option<JsonString>> {
        let pool = context.data::<Pool<Postgres>>()?;

        match sqlx::query_as::<_, IndexerStorage>(
            "SELECT * FROM indexer_storage WHERE function_name = $1 AND key_name = $2",
        )
        .bind(function_name)
        .bind(key)
        .fetch_optional(pool)
        .await
        {
            Ok(Some(storage)) => Ok(Some(JsonString(storage.value))),
            Ok(None) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }
}

pub struct Mutation;

#[async_graphql::Object]
impl Mutation {
    async fn set(
        &self,
        context: &async_graphql::Context<'_>,
        function_name: String,
        key: String,
        data: JsonString,
    ) -> async_graphql::Result<JsonString> {
        let pool = context.data::<Pool<Postgres>>()?;

        match sqlx::query_as::<_, IndexerStorage>(
            "INSERT INTO indexer_storage (function_name, key_name, value) VALUES ($1, $2, $3) RETURNING *"
        )
            .bind(function_name)
            .bind(key)
            .bind(data.0)
            .fetch_one(pool)
            .await {
                Ok(storage) => Ok(JsonString(storage.value)),
                Err(err) => Err(err.into())
            }
    }
}

pub type Schema = async_graphql::Schema<Query, Mutation, async_graphql::EmptySubscription>;

pub fn create_schema(
) -> async_graphql::SchemaBuilder<Query, Mutation, async_graphql::EmptySubscription> {
    async_graphql::Schema::build(Query, Mutation, async_graphql::EmptySubscription)
}

#[test]
fn schema_snapshot() {
    insta::assert_snapshot!(create_schema().finish().sdl())
}
