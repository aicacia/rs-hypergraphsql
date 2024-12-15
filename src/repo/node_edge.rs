#[derive(sqlx::FromRow)]
pub struct NodeEdgeRow {
  pub from_node_id: i64,
  pub from_node_uri: String,
  pub from_node_data: String,
  pub from_node_updated_at: i64,
  pub from_node_created_at: i64,

  pub to_node_id: i64,
  pub to_node_uri: String,
  pub to_node_data: String,
  pub to_node_updated_at: i64,
  pub to_node_created_at: i64,

  pub edge_id: i64,
  pub edge_uri: String,
  pub edge_data: Option<String>,
  pub edge_updated_at: i64,
  pub edge_created_at: i64,
}
