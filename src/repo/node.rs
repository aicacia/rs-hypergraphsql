#[derive(sqlx::FromRow)]
pub struct NodeRow {
  pub id: i64,
  pub uri: String,
  pub data: String,
  pub updated_at: i64,
  pub created_at: i64,
}

pub async fn get_node_by_id(pool: &sqlx::SqlitePool, id: i64) -> sqlx::Result<Option<NodeRow>> {
  sqlx::query_as!(
    NodeRow,
    "SELECT node.* FROM nodes node WHERE node.id = $1;",
    id
  )
  .fetch_optional(pool)
  .await
}

pub async fn get_nodes_by_uri(pool: &sqlx::SqlitePool, uri: &str) -> sqlx::Result<Vec<NodeRow>> {
  sqlx::query_as!(
    NodeRow,
    "SELECT node.* FROM nodes node WHERE node.uri = $1;",
    uri
  )
  .fetch_all(pool)
  .await
}

pub async fn create_node(pool: &sqlx::SqlitePool, uri: &str, data: &str) -> sqlx::Result<NodeRow> {
  sqlx::query_as!(
    NodeRow,
    "INSERT INTO nodes (uri, data) VALUES ($1, $2) RETURNING *;",
    uri,
    data
  )
  .fetch_one(pool)
  .await
}

pub async fn delete_node(pool: &sqlx::SqlitePool, id: i64) -> sqlx::Result<Option<NodeRow>> {
  sqlx::query_as!(NodeRow, "DELETE FROM nodes WHERE id = $1 RETURNING *;", id)
    .fetch_optional(pool)
    .await
}

pub async fn delete_nodes_by_uri(pool: &sqlx::SqlitePool, uri: &str) -> sqlx::Result<Vec<NodeRow>> {
  sqlx::query_as!(
    NodeRow,
    "DELETE FROM nodes WHERE uri = $1 RETURNING *;",
    uri
  )
  .fetch_all(pool)
  .await
}
