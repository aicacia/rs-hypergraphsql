use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
  model::{edge::Edge, node::Node, node_edge::NodeEdge},
  repo::{self, node_edge::NodeEdgeRow},
};

pub async fn create_node<T>(pool: &sqlx::SqlitePool, uri: &str, data: T) -> sqlx::Result<Node<T>>
where
  T: Serialize + DeserializeOwned,
{
  let data_json = match serde_json::to_string(&data) {
    Ok(json) => json,
    Err(e) => return Err(sqlx::Error::Encode(Box::new(e))),
  };
  let row = repo::node::create_node(pool, uri, &data_json).await?;
  Ok(Node {
    id: row.id,
    uri: row.uri,
    data,
    updated_at: DateTime::<Utc>::from_timestamp(row.updated_at, 0).unwrap_or_default(),
    created_at: DateTime::<Utc>::from_timestamp(row.created_at, 0).unwrap_or_default(),
  })
}

pub async fn create_edge<E, FromNode, ToNode>(
  pool: &sqlx::SqlitePool,
  from_node: &Node<FromNode>,
  to_node: &Node<ToNode>,
  uri: &str,
  data: Option<E>,
) -> sqlx::Result<Edge<E>>
where
  E: Serialize + DeserializeOwned,
{
  let data_json = if let Some(d) = &data {
    match serde_json::to_string(d) {
      Ok(json) => Some(json),
      Err(e) => return Err(sqlx::Error::Encode(Box::new(e))),
    }
  } else {
    None
  };
  let row = repo::edge::create_edge(
    pool,
    from_node.id,
    to_node.id,
    uri,
    data_json.as_ref().map(String::as_ref),
  )
  .await?;
  Ok(Edge {
    id: row.id,
    from_node_id: row.from_node_id,
    to_node_id: row.to_node_id,
    uri: row.uri,
    data,
    updated_at: DateTime::<Utc>::from_timestamp(row.updated_at, 0).unwrap_or_default(),
    created_at: DateTime::<Utc>::from_timestamp(row.created_at, 0).unwrap_or_default(),
  })
}

pub async fn get_related_by_edge_uri<FN, TN, E>(
  pool: &sqlx::SqlitePool,
  edge_uri: &str,
) -> sqlx::Result<Vec<NodeEdge<FN, TN, E>>>
where
  FN: DeserializeOwned,
  TN: DeserializeOwned,
  E: DeserializeOwned,
{
  node_edge_rows_to_model(repo::node_edge::get_related_by_edge_uri(pool, edge_uri).await?)
}

pub async fn get_related_by_node_id<FN, TN, E>(
  pool: &sqlx::SqlitePool,
  node_id: i64,
) -> sqlx::Result<Vec<NodeEdge<FN, TN, E>>>
where
  FN: DeserializeOwned,
  TN: DeserializeOwned,
  E: DeserializeOwned,
{
  node_edge_rows_to_model(repo::node_edge::get_related_by_node_id(pool, node_id).await?)
}

pub async fn get_related_by_from_node_id<FN, TN, E>(
  pool: &sqlx::SqlitePool,
  from_node_id: i64,
) -> sqlx::Result<Vec<NodeEdge<FN, TN, E>>>
where
  FN: DeserializeOwned,
  TN: DeserializeOwned,
  E: DeserializeOwned,
{
  node_edge_rows_to_model(repo::node_edge::get_related_by_from_node_id(pool, from_node_id).await?)
}

pub async fn get_related_by_to_node_id<FN, TN, E>(
  pool: &sqlx::SqlitePool,
  to_node_id: i64,
) -> sqlx::Result<Vec<NodeEdge<FN, TN, E>>>
where
  FN: DeserializeOwned,
  TN: DeserializeOwned,
  E: DeserializeOwned,
{
  node_edge_rows_to_model(repo::node_edge::get_related_by_to_node_id(pool, to_node_id).await?)
}

pub async fn get_related_by_node_id_and_edge_uri<FN, TN, E>(
  pool: &sqlx::SqlitePool,
  node_id: i64,
  edge_uri: &str,
) -> sqlx::Result<Vec<NodeEdge<FN, TN, E>>>
where
  FN: DeserializeOwned,
  TN: DeserializeOwned,
  E: DeserializeOwned,
{
  node_edge_rows_to_model(
    repo::node_edge::get_related_by_node_id_and_edge_uri(pool, node_id, edge_uri).await?,
  )
}

pub async fn get_related_by_from_node_id_and_edge_uri<FN, TN, E>(
  pool: &sqlx::SqlitePool,
  from_node_id: i64,
  edge_uri: &str,
) -> sqlx::Result<Vec<NodeEdge<FN, TN, E>>>
where
  FN: DeserializeOwned,
  TN: DeserializeOwned,
  E: DeserializeOwned,
{
  node_edge_rows_to_model(
    repo::node_edge::get_related_by_from_node_id_and_edge_uri(pool, from_node_id, edge_uri).await?,
  )
}

pub async fn get_related_by_to_node_id_and_edge_uri<FN, TN, E>(
  pool: &sqlx::SqlitePool,
  to_node_id: i64,
  edge_uri: &str,
) -> sqlx::Result<Vec<NodeEdge<FN, TN, E>>>
where
  FN: DeserializeOwned,
  TN: DeserializeOwned,
  E: DeserializeOwned,
{
  node_edge_rows_to_model(
    repo::node_edge::get_related_by_to_node_id_and_edge_uri(pool, to_node_id, edge_uri).await?,
  )
}

pub fn node_edge_rows_to_model<FN, TN, E>(
  rows: Vec<NodeEdgeRow>,
) -> sqlx::Result<Vec<NodeEdge<FN, TN, E>>>
where
  FN: DeserializeOwned,
  TN: DeserializeOwned,
  E: DeserializeOwned,
{
  let mut node_edges = Vec::with_capacity(rows.len());
  for row in rows {
    match row.try_into() {
      Ok(node_edge) => node_edges.push(node_edge),
      Err(e) => return Err(sqlx::Error::Decode(Box::new(e))),
    }
  }
  Ok(node_edges)
}
