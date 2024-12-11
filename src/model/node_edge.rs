use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::repo::node_edge::NodeEdgeRow;

use super::{edge::Edge, node::Node};

#[derive(Serialize, Deserialize)]
pub struct NodeEdge<FN, TN, E> {
  pub from_node_id: i64,
  pub from_node_uri: String,
  pub from_node_data: FN,
  pub from_node_updated_at: DateTime<Utc>,
  pub from_node_created_at: DateTime<Utc>,

  pub to_node_id: i64,
  pub to_node_uri: String,
  pub to_node_data: TN,
  pub to_node_updated_at: DateTime<Utc>,
  pub to_node_created_at: DateTime<Utc>,

  pub edge_id: i64,
  pub edge_uri: String,
  pub edge_data: Option<E>,
  pub edge_updated_at: DateTime<Utc>,
  pub edge_created_at: DateTime<Utc>,
}

impl<FN, TN, E> NodeEdge<FN, TN, E> {
  pub fn from_node(&self) -> Node<&FN> {
    Node {
      id: self.from_node_id,
      uri: self.from_node_uri.clone(),
      data: &self.from_node_data,
      updated_at: self.from_node_updated_at,
      created_at: self.from_node_created_at,
    }
  }

  pub fn to_node(&self) -> Node<&TN> {
    Node {
      id: self.to_node_id,
      uri: self.to_node_uri.clone(),
      data: &self.to_node_data,
      updated_at: self.to_node_updated_at,
      created_at: self.to_node_created_at,
    }
  }

  pub fn to_edge(&self) -> Edge<&E> {
    Edge {
      id: self.edge_id,
      from_node_id: self.from_node_id,
      to_node_id: self.to_node_id,
      uri: self.edge_uri.clone(),
      data: self.edge_data.as_ref(),
      updated_at: self.edge_updated_at,
      created_at: self.edge_created_at,
    }
  }
}

impl<FN, TN, E> NodeEdge<FN, TN, E>
where
  FN: Clone,
{
  pub fn from_node_owned(&self) -> Node<FN> {
    Node {
      id: self.from_node_id,
      uri: self.from_node_uri.clone(),
      data: self.from_node_data.clone(),
      updated_at: self.from_node_updated_at,
      created_at: self.from_node_created_at,
    }
  }
}

impl<FN, TN, E> NodeEdge<FN, TN, E>
where
  TN: Clone,
{
  pub fn to_node_owned(&self) -> Node<TN> {
    Node {
      id: self.to_node_id,
      uri: self.to_node_uri.clone(),
      data: self.to_node_data.clone(),
      updated_at: self.to_node_updated_at,
      created_at: self.to_node_created_at,
    }
  }
}

impl<FN, TN, E> NodeEdge<FN, TN, E>
where
  E: Clone,
{
  pub fn to_edge_owned(&self) -> Edge<E> {
    Edge {
      id: self.edge_id,
      from_node_id: self.from_node_id,
      to_node_id: self.to_node_id,
      uri: self.edge_uri.clone(),
      data: self.edge_data.clone(),
      updated_at: self.edge_updated_at,
      created_at: self.edge_created_at,
    }
  }
}

impl<FN, TN, E> TryFrom<NodeEdgeRow> for NodeEdge<FN, TN, E>
where
  FN: DeserializeOwned,
  TN: DeserializeOwned,
  E: DeserializeOwned,
{
  type Error = serde_json::Error;

  fn try_from(row: NodeEdgeRow) -> Result<Self, Self::Error> {
    Ok(Self {
      from_node_id: row.from_node_id,
      from_node_uri: row.from_node_uri,
      from_node_data: serde_json::from_str::<FN>(&row.from_node_data)?,
      from_node_updated_at: DateTime::<Utc>::from_timestamp(row.from_node_updated_at, 0)
        .unwrap_or_default(),
      from_node_created_at: DateTime::<Utc>::from_timestamp(row.from_node_created_at, 0)
        .unwrap_or_default(),

      to_node_id: row.to_node_id,
      to_node_uri: row.to_node_uri,
      to_node_data: serde_json::from_str::<TN>(&row.to_node_data)?,
      to_node_updated_at: DateTime::<Utc>::from_timestamp(row.to_node_updated_at, 0)
        .unwrap_or_default(),
      to_node_created_at: DateTime::<Utc>::from_timestamp(row.to_node_created_at, 0)
        .unwrap_or_default(),

      edge_id: row.edge_id,
      edge_uri: row.edge_uri,
      edge_data: if let Some(data) = row.edge_data {
        Some(serde_json::from_str::<E>(&data)?)
      } else {
        None
      },
      edge_updated_at: DateTime::<Utc>::from_timestamp(row.edge_updated_at, 0).unwrap_or_default(),
      edge_created_at: DateTime::<Utc>::from_timestamp(row.edge_created_at, 0).unwrap_or_default(),
    })
  }
}
