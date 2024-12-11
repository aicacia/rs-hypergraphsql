use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::repo::node::NodeRow;

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
