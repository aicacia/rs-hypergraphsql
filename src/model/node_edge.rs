use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::repo::node_edge::NodeEdgeRow;

use super::{edge::Edge, node::Node};

#[derive(Serialize, Deserialize)]
pub struct NodeEdge<FN, TN, E> {
  pub from_node: Node<FN>,
  pub to_node: Node<TN>,
  pub edge: Edge<E>,
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
      from_node: Node {
        id: row.from_node_id,
        uri: row.from_node_uri,
        data: serde_json::from_str::<FN>(&row.from_node_data)?,
        updated_at: DateTime::<Utc>::from_timestamp(row.from_node_updated_at, 0)
          .unwrap_or_default(),
        created_at: DateTime::<Utc>::from_timestamp(row.from_node_created_at, 0)
          .unwrap_or_default(),
      },
      to_node: Node {
        id: row.to_node_id,
        uri: row.to_node_uri,
        data: serde_json::from_str::<TN>(&row.to_node_data)?,
        updated_at: DateTime::<Utc>::from_timestamp(row.to_node_updated_at, 0).unwrap_or_default(),
        created_at: DateTime::<Utc>::from_timestamp(row.to_node_created_at, 0).unwrap_or_default(),
      },
      edge: Edge {
        id: row.edge_id,
        from_node_id: row.from_node_id,
        to_node_id: row.to_node_id,
        uri: row.edge_uri,
        data: if let Some(data) = row.edge_data {
          Some(serde_json::from_str::<E>(&data)?)
        } else {
          None
        },
        updated_at: DateTime::<Utc>::from_timestamp(row.edge_updated_at, 0).unwrap_or_default(),
        created_at: DateTime::<Utc>::from_timestamp(row.edge_created_at, 0).unwrap_or_default(),
      },
    })
  }
}
