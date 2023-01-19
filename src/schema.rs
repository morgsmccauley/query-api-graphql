use juniper::{EmptySubscription, RootNode};
use juniper::{FieldError, FieldResult};

use juniper::{GraphQLInputObject, GraphQLObject};
use sqlx::{FromRow, Pool, Postgres};

pub struct Context {
    pub db: Pool<Postgres>,
}

impl juniper::Context for Context {}

#[derive(GraphQLObject, FromRow)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Human {
    id: i32,
    name: String,
    home_planet: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewHuman {
    name: String,
    home_planet: String,
}

pub struct QueryRoot;

#[juniper::graphql_object(context = Context)]
impl QueryRoot {
    async fn humans(context: &Context) -> FieldResult<Vec<Human>> {
        sqlx::query_as::<_, Human>("SELECT * from humans")
            .fetch_all(&context.db)
            .await
            .map_err(FieldError::from)
    }

    async fn human(context: &Context, id: i32) -> FieldResult<Human> {
        sqlx::query_as::<_, Human>("SELECT * from humans where id = $1")
            .bind(id)
            .fetch_one(&context.db)
            .await
            .map_err(FieldError::from)
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(context = Context)]
impl MutationRoot {
    async fn create_human(context: &Context, new_human: NewHuman) -> FieldResult<Human> {
        sqlx::query_as::<_, Human>(
            "INSERT INTO humans (name, home_planet) VALUES ($1, $2) RETURNING *",
        )
        .bind(new_human.name)
        .bind(new_human.home_planet)
        .fetch_one(&context.db)
        .await
        .map_err(FieldError::from)
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
