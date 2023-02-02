pub type Pool = sqlx::PgPool;

pub async fn create_pool() -> sqlx::PgPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool")
}

#[derive(sqlx::FromRow, async_graphql::SimpleObject)]
#[allow(dead_code)]
pub struct IndexerStorage {
    function_name: String,
    key_name: String,
    pub value: String,
}

pub mod indexer_storage {
    pub async fn get(
        pool: &super::Pool,
        function_name: String,
        key: String,
    ) -> Result<Option<super::IndexerStorage>, sqlx::Error> {
        sqlx::query_as::<_, super::IndexerStorage>(
            "SELECT * FROM indexer_storage WHERE function_name = $1 AND key_name = $2",
        )
        .bind(function_name)
        .bind(key)
        .fetch_optional(pool)
        .await
    }

    pub async fn create(
        pool: &super::Pool,
        function_name: String,
        key: String,
        data: String,
    ) -> Result<super::IndexerStorage, sqlx::Error> {
        sqlx::query_as::<_, super::IndexerStorage>(
            "INSERT INTO indexer_storage (function_name, key_name, value) VALUES ($1, $2, $3) ON CONFLICT ON CONSTRAINT indexer_storage_pkey  DO UPDATE SET value = EXCLUDED.value RETURNING *"
        )
            .bind(function_name)
            .bind(key)
            .bind(data)
            .fetch_one(pool)
            .await
    }
}
