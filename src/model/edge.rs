use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::repo::edge::EdgeRow;

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
