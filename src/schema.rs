use crate::database::{self, Pool};

#[derive(serde::Serialize, serde::Deserialize)]
struct JsonString(String);

async_graphql::scalar!(JsonString);

pub struct Query;

#[async_graphql::Object]
impl Query {
    async fn get(
        &self,
        context: &async_graphql::Context<'_>,
        function_name: String,
        key: String,
    ) -> async_graphql::Result<Option<JsonString>> {
        match database::indexer_storage::get(context.data::<Pool>()?, function_name, key).await {
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
        match database::indexer_storage::create(context.data::<Pool>()?, function_name, key, data.0)
            .await
        {
            Ok(storage) => Ok(JsonString(storage.value)),
            Err(err) => Err(err.into()),
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
