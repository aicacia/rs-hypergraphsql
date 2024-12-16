use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::repo::{self, node::NodeRow};

#[derive(Serialize, Deserialize)]
pub struct Node<T> {
  pub id: i64,
  pub uri: String,
  pub data: T,
  pub updated_at: DateTime<Utc>,
  pub created_at: DateTime<Utc>,
}

impl<T> TryFrom<NodeRow> for Node<T>
where
  T: DeserializeOwned,
{
  type Error = serde_json::Error;

  fn try_from(row: NodeRow) -> Result<Self, Self::Error> {
    Ok(Self {
      id: row.id,
      uri: row.uri,
      data: serde_json::from_str::<T>(&row.data)?,
      updated_at: DateTime::<Utc>::from_timestamp(row.updated_at, 0).unwrap_or_default(),
      created_at: DateTime::<Utc>::from_timestamp(row.created_at, 0).unwrap_or_default(),
    })
  }
}

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

pub async fn delete_node<T>(pool: &sqlx::SqlitePool, node_id: i64) -> sqlx::Result<Option<Node<T>>>
where
  T: Serialize + DeserializeOwned,
{
  let node = repo::node::delete_node(pool, node_id).await?;
  if let Some(row) = node {
    match Node::try_from(row) {
      Ok(node) => Ok(Some(node)),
      Err(e) => Err(sqlx::Error::Decode(Box::new(e))),
    }
  } else {
    Ok(None)
  }
}

pub async fn delete_nodes<E>(
  pool: &sqlx::SqlitePool,
  node_ids: &[i64],
) -> sqlx::Result<Vec<Node<E>>>
where
  E: Serialize + DeserializeOwned,
{
  let rows: Vec<NodeRow> = repo::node::delete_nodes(pool, node_ids).await?;
  let mut nodes = Vec::with_capacity(rows.len());
  for row in rows {
    match Node::try_from(row) {
      Ok(node) => nodes.push(node),
      Err(e) => return Err(sqlx::Error::Decode(Box::new(e))),
    }
  }
  Ok(nodes)
}

pub async fn delete_nodes_by_uri<T>(
  pool: &sqlx::SqlitePool,
  uri: &str,
) -> sqlx::Result<Vec<Node<T>>>
where
  T: Serialize + DeserializeOwned,
{
  let rows: Vec<NodeRow> = repo::node::delete_nodes_by_uri(pool, uri).await?;
  let mut nodes = Vec::with_capacity(rows.len());
  for row in rows {
    match Node::try_from(row) {
      Ok(node) => nodes.push(node),
      Err(e) => return Err(sqlx::Error::Decode(Box::new(e))),
    }
  }
  Ok(nodes)
}
