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

pub async fn get_edge_by_id(pool: &sqlx::SqlitePool, id: i64) -> sqlx::Result<Option<EdgeRow>> {
  sqlx::query_as!(
    EdgeRow,
    "SELECT edge.* FROM edges edge WHERE edge.id = $1;",
    id
  )
  .fetch_optional(pool)
  .await
}

pub async fn get_edges_by_from_node_id(
  pool: &sqlx::SqlitePool,
  from_node_id: i64,
) -> sqlx::Result<Vec<EdgeRow>> {
  sqlx::query_as!(
    EdgeRow,
    "SELECT edge.* FROM edges edge WHERE edge.from_node_id = $1;",
    from_node_id
  )
  .fetch_all(pool)
  .await
}

pub async fn get_edges_by_to_node_id(
  pool: &sqlx::SqlitePool,
  to_node_id: i64,
) -> sqlx::Result<Vec<EdgeRow>> {
  sqlx::query_as!(
    EdgeRow,
    "SELECT edge.* FROM edges edge WHERE edge.to_node_id = $1;",
    to_node_id
  )
  .fetch_all(pool)
  .await
}

pub async fn create_edge(
  pool: &sqlx::SqlitePool,
  from_node_id: i64,
  to_node_id: i64,
  uri: &str,
  data: Option<&str>,
) -> sqlx::Result<EdgeRow> {
  sqlx::query_as!(
    EdgeRow,
    "INSERT INTO edges (from_node_id, to_node_id, uri, data) VALUES ($1, $2, $3, $4) RETURNING *;",
    from_node_id,
    to_node_id,
    uri,
    data
  )
  .fetch_one(pool)
  .await
}

pub async fn delete_edge(pool: &sqlx::SqlitePool, edge_id: i64) -> sqlx::Result<Option<EdgeRow>> {
  sqlx::query_as!(
    EdgeRow,
    "DELETE FROM edges WHERE id = $1 RETURNING *;",
    edge_id
  )
  .fetch_optional(pool)
  .await
}

pub async fn delete_edges_by_uri(pool: &sqlx::SqlitePool, uri: &str) -> sqlx::Result<Vec<EdgeRow>> {
  sqlx::query_as!(
    EdgeRow,
    "DELETE FROM edges WHERE uri = $1 RETURNING *;",
    uri
  )
  .fetch_all(pool)
  .await
}
