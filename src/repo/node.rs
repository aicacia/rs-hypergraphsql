#[derive(sqlx::FromRow)]
pub struct NodeRow {
  pub id: i64,
  pub uri: String,
  pub data: String,
  pub updated_at: i64,
  pub created_at: i64,
}

pub async fn create_node(pool: &sqlx::SqlitePool, uri: &str, data: &str) -> sqlx::Result<NodeRow> {
  sqlx::query_as("INSERT INTO nodes (uri, data) VALUES ($1, $2) RETURNING *;")
    .bind(uri)
    .bind(data)
    .fetch_one(pool)
    .await
}

pub async fn delete_node(pool: &sqlx::SqlitePool, id: i64) -> sqlx::Result<Option<NodeRow>> {
  sqlx::query_as("DELETE FROM nodes WHERE id = $1 RETURNING *;")
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn delete_nodes(pool: &sqlx::SqlitePool, node_ids: &[i64]) -> sqlx::Result<Vec<NodeRow>> {
  let ids = node_ids
    .into_iter()
    .map(ToString::to_string)
    .collect::<Vec<String>>()
    .join(",");
  sqlx::query_as(&format!(
    "DELETE FROM nodes WHERE id in ({ids}) RETURNING *;"
  ))
  .fetch_all(pool)
  .await
}

pub async fn delete_nodes_by_uri(pool: &sqlx::SqlitePool, uri: &str) -> sqlx::Result<Vec<NodeRow>> {
  sqlx::query_as("DELETE FROM nodes WHERE uri = $1 RETURNING *;")
    .bind(uri)
    .fetch_all(pool)
    .await
}
