#[derive(sqlx::FromRow)]
pub struct EdgeRow {
  pub id: i64,
  pub from_node_id: i64,
  pub to_node_id: i64,
  pub uri: String,
  pub data: Option<String>,
  pub updated_at: i64,
  pub created_at: i64,
}

pub async fn create_edge(
  pool: &sqlx::SqlitePool,
  from_node_id: i64,
  to_node_id: i64,
  uri: &str,
  data: Option<&str>,
) -> sqlx::Result<EdgeRow> {
  sqlx::query_as(
    "INSERT INTO edges (from_node_id, to_node_id, uri, data) VALUES ($1, $2, $3, $4) RETURNING *;",
  )
  .bind(from_node_id)
  .bind(to_node_id)
  .bind(uri)
  .bind(data)
  .fetch_one(pool)
  .await
}

pub async fn delete_edge(pool: &sqlx::SqlitePool, edge_id: i64) -> sqlx::Result<Option<EdgeRow>> {
  sqlx::query_as("DELETE FROM edges WHERE id = $1 RETURNING *;")
    .bind(edge_id)
    .fetch_optional(pool)
    .await
}

pub async fn delete_edges(pool: &sqlx::SqlitePool, edge_ids: &[i64]) -> sqlx::Result<Vec<EdgeRow>> {
  let ids = edge_ids
    .into_iter()
    .map(ToString::to_string)
    .collect::<Vec<String>>()
    .join(",");
  sqlx::query_as(&format!(
    "DELETE FROM edges WHERE id in ({ids}) RETURNING *;"
  ))
  .fetch_all(pool)
  .await
}

pub async fn delete_edges_by_uri(pool: &sqlx::SqlitePool, uri: &str) -> sqlx::Result<Vec<EdgeRow>> {
  sqlx::query_as("DELETE FROM edges WHERE uri = $1 RETURNING *;")
    .bind(uri)
    .fetch_all(pool)
    .await
}
