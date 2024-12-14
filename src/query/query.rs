use std::{collections::HashMap, fmt};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
  model::{edge::Edge, node::Node, node_edge::NodeEdge},
  repo::{edge::EdgeRow, node::NodeRow, node_edge::NodeEdgeRow},
};

use super::builder::query_builder;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Query {
  #[serde(rename = "node")]
  Node(HashMap<QueryField, QueryExpr>),
  #[serde(rename = "edge")]
  Edge(HashMap<QueryField, QueryExpr>),
  #[serde(rename = "node_edge")]
  NodeEdge(HashMap<QueryField, QueryExpr>),
}

impl Query {
  pub fn query_builder(&self) -> sqlx::QueryBuilder<'_, sqlx::Sqlite> {
    query_builder::<sqlx::Sqlite>(self)
  }

  pub fn into_sql(&self) -> String {
    self.query_builder().into_sql()
  }

  pub fn is_node_query(&self) -> bool {
    match self {
      Query::Node(_) => true,
      _ => false,
    }
  }

  pub fn is_edge_query(&self) -> bool {
    match self {
      Query::Edge(_) => true,
      _ => false,
    }
  }

  pub fn is_node_edge_query(&self) -> bool {
    match self {
      Query::NodeEdge(_) => true,
      _ => false,
    }
  }

  pub async fn node_rows(&self, pool: &sqlx::SqlitePool) -> sqlx::Result<Vec<NodeRow>> {
    if self.is_node_query() {
      return sqlx::query_as(query_builder::<sqlx::Sqlite>(self).sql())
        .fetch_all(pool)
        .await;
    }
    Err(sqlx::Error::Encode(Box::new(QueryError::InvalidQueryType)))
  }

  pub async fn nodes<N>(&self, pool: &sqlx::SqlitePool) -> sqlx::Result<Vec<Node<N>>>
  where
    N: DeserializeOwned,
  {
    let rows = self.node_rows(pool).await?;
    let mut edges = Vec::with_capacity(rows.len());
    for row in rows {
      match row.try_into() {
        Ok(edge) => edges.push(edge),
        Err(e) => return Err(sqlx::Error::Decode(Box::new(e))),
      }
    }
    Ok(edges)
  }

  pub async fn edge_rows(&self, pool: &sqlx::SqlitePool) -> sqlx::Result<Vec<EdgeRow>> {
    if self.is_edge_query() {
      return sqlx::query_as(query_builder::<sqlx::Sqlite>(self).sql())
        .fetch_all(pool)
        .await;
    }
    Err(sqlx::Error::Encode(Box::new(QueryError::InvalidQueryType)))
  }

  pub async fn edges<E>(&self, pool: &sqlx::SqlitePool) -> sqlx::Result<Vec<Edge<E>>>
  where
    E: DeserializeOwned,
  {
    let rows = self.edge_rows(pool).await?;
    let mut edges = Vec::with_capacity(rows.len());
    for row in rows {
      match row.try_into() {
        Ok(edge) => edges.push(edge),
        Err(e) => return Err(sqlx::Error::Decode(Box::new(e))),
      }
    }
    Ok(edges)
  }

  pub async fn node_edge_rows(&self, pool: &sqlx::SqlitePool) -> sqlx::Result<Vec<NodeEdgeRow>> {
    if self.is_node_edge_query() {
      return sqlx::query_as(query_builder::<sqlx::Sqlite>(self).sql())
        .fetch_all(pool)
        .await;
    }
    Err(sqlx::Error::Encode(Box::new(QueryError::InvalidQueryType)))
  }

  pub async fn node_edges<FN, TN, E>(
    &self,
    pool: &sqlx::SqlitePool,
  ) -> sqlx::Result<Vec<NodeEdge<FN, TN, E>>>
  where
    FN: DeserializeOwned,
    TN: DeserializeOwned,
    E: DeserializeOwned,
  {
    let rows = self.node_edge_rows(pool).await?;
    let mut node_edges = Vec::with_capacity(rows.len());
    for row in rows {
      match row.try_into() {
        Ok(node_edge) => node_edges.push(node_edge),
        Err(e) => return Err(sqlx::Error::Decode(Box::new(e))),
      }
    }
    Ok(node_edges)
  }
}

#[derive(Debug)]
pub enum QueryError {
  InvalidQueryType,
}

impl std::error::Error for QueryError {}

impl fmt::Display for QueryError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      QueryError::InvalidQueryType => write!(f, "Invalid query type"),
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub enum QueryField {
  // Node
  #[serde(rename = "node.id")]
  NodeId,
  #[serde(rename = "node.uri")]
  NodeURI,
  #[serde(rename = "node.data")]
  NodeData,
  #[serde(rename = "node.updated_at")]
  NodeUpdatedAt,
  #[serde(rename = "node.created_at")]
  NodeCreatedAt,
  // Edge
  #[serde(rename = "edge.id")]
  EdgeId,
  #[serde(rename = "edge.from_node_id")]
  EdgeFromNodeId,
  #[serde(rename = "edge.to_node_id")]
  EdgeToNodeId,
  #[serde(rename = "edge.uri")]
  EdgeURI,
  #[serde(rename = "edge.data")]
  EdgeData,
  #[serde(rename = "edge.updated_at")]
  EdgeUpdatedAt,
  #[serde(rename = "edge.created_at")]
  EdgeCreatedAt,
  // FromNode
  #[serde(rename = "from_node.id")]
  NodeEdgeFromNodeId,
  #[serde(rename = "from_node.uri")]
  NodeEdgeFromNodeURI,
  #[serde(rename = "from_node.data")]
  NodeEdgeFromNodeData,
  #[serde(rename = "from_node.updated_at")]
  NodeEdgeFromNodeUpdatedAt,
  #[serde(rename = "from_node.created_at")]
  NodeEdgeFromNodeCreatedAt,
  // ToNode
  #[serde(rename = "to_node.id")]
  NodeEdgeToNodeId,
  #[serde(rename = "to_node.uri")]
  NodeEdgeToNodeURI,
  #[serde(rename = "to_node.data")]
  NodeEdgeToNodeData,
  #[serde(rename = "to_node.updated_at")]
  NodeEdgeToNodeUpdatedAt,
  #[serde(rename = "to_node.created_at")]
  NodeEdgeToNodeCreatedAt,
}

impl fmt::Display for QueryField {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match serde_json::to_string(self) {
      Ok(s) => write!(f, "{}", &s[1..(s.len() - 1)]),
      Err(_) => Err(std::fmt::Error),
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum QueryExpr {
  Value(QueryValue),
  Field(QueryField),
  Op(QueryOp),
  Data(HashMap<String, QueryExpr>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum QueryValue {
  Null,
  Bool(bool),
  String(String),
  Number(serde_json::Number),
}

impl QueryValue {
  pub fn into_sql(&self) -> String {
    match self {
      QueryValue::Null => "NULL".to_string(),
      QueryValue::Bool(b) => b.to_string(),
      QueryValue::String(s) => format!("'{}'", s),
      QueryValue::Number(n) => n.to_string(),
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum QueryOp {
  Eq(Box<QueryExpr>),
  Neq(Box<QueryExpr>),
  Gt(Box<QueryExpr>),
  Lt(Box<QueryExpr>),
  Gte(Box<QueryExpr>),
  Lte(Box<QueryExpr>),
  Like(Box<QueryExpr>),
  In(Vec<QueryExpr>),
  And(Vec<QueryExpr>),
  Or(Vec<QueryExpr>),
  Not(Box<QueryExpr>),
}
