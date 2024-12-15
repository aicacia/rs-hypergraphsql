use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::repo::{self, edge::EdgeRow};

use super::node::Node;

#[derive(Serialize, Deserialize)]
pub struct Edge<T> {
  pub id: i64,
  pub from_node_id: i64,
  pub to_node_id: i64,
  pub uri: String,
  pub data: Option<T>,
  pub updated_at: DateTime<Utc>,
  pub created_at: DateTime<Utc>,
}

impl<T> TryFrom<EdgeRow> for Edge<T>
where
  T: DeserializeOwned,
{
  type Error = serde_json::Error;

  fn try_from(row: EdgeRow) -> Result<Self, Self::Error> {
    Ok(Self {
      id: row.id,
      from_node_id: row.from_node_id,
      to_node_id: row.to_node_id,
      uri: row.uri,
      data: if let Some(data) = row.data {
        Some(serde_json::from_str::<T>(&data)?)
      } else {
        None
      },
      updated_at: DateTime::<Utc>::from_timestamp(row.updated_at, 0).unwrap_or_default(),
      created_at: DateTime::<Utc>::from_timestamp(row.created_at, 0).unwrap_or_default(),
    })
  }
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
  create_edge_with_ids(pool, from_node.id, to_node.id, uri, data).await
}

pub async fn create_edge_with_ids<E>(
  pool: &sqlx::SqlitePool,
  from_node_id: i64,
  to_node_id: i64,
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
    from_node_id,
    to_node_id,
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
