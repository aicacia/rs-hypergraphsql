use std::{collections::HashMap, fmt};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
  model::{edge::Edge, node::Node, node_edge::NodeEdge},
  repo::{edge::EdgeRow, node::NodeRow, node_edge::NodeEdgeRow},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Query(pub HashMap<QueryField, QueryExpr>);

impl Query {
  pub fn sql(&self) -> String {
    query_condition_builder(sqlx::QueryBuilder::<'_, sqlx::Sqlite>::new(""), &self.0).into_sql()
  }

  pub fn nodes_sql(&self) -> String {
    query_condition_builder(
      sqlx::QueryBuilder::<'_, sqlx::Sqlite>::new("SELECT node.* FROM nodes node"),
      &self.0,
    )
    .into_sql()
  }

  pub fn edges_sql(&self) -> String {
    query_condition_builder(
      sqlx::QueryBuilder::<'_, sqlx::Sqlite>::new("SELECT edge.* FROM nodes edge"),
      &self.0,
    )
    .into_sql()
  }

  pub fn node_edges_sql(&self) -> String {
    query_condition_builder(
      sqlx::QueryBuilder::<'_, sqlx::Sqlite>::new(
        r#"SELECT 
        from_node.id as from_node_id, 
        from_node.uri as from_node_uri,
        from_node.data as from_node_data,
        from_node.created_at as from_node_created_at,
        from_node.updated_at as from_node_updated_at,

        to_node.id as to_node_id, 
        to_node.uri as to_node_uri,
        to_node.data as to_node_data,
        to_node.created_at as to_node_created_at,
        to_node.updated_at as to_node_updated_at,

        edge.id as edge_id, 
        edge.uri as edge_uri,
        edge.data as edge_data,
        edge.created_at as edge_created_at,
        edge.updated_at as edge_updated_at
       FROM edges edge
       JOIN nodes from_node ON from_node.id = edge.from_node_id
       JOIN nodes to_node ON to_node.id = edge.to_node_id"#,
      ),
      &self.0,
    )
    .into_sql()
  }

  pub async fn node_rows(&self, pool: &sqlx::SqlitePool) -> sqlx::Result<Vec<NodeRow>> {
    sqlx::query_as(&self.nodes_sql()).fetch_all(pool).await
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
    sqlx::query_as(&self.edges_sql()).fetch_all(pool).await
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
    sqlx::query_as(&self.node_edges_sql()).fetch_all(pool).await
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

pub fn query_condition_builder<'args, DB: sqlx::Database>(
  mut qb: sqlx::QueryBuilder<'args, DB>,
  query: &HashMap<QueryField, QueryExpr>,
) -> sqlx::QueryBuilder<'args, DB> {
  for (i, (field, expr)) in query.into_iter().enumerate() {
    if i == 0 {
      qb.push(" WHERE ");
    }
    if i > 0 {
      qb.push(" AND ");
    }
    qb = query_condition_builder_expr(qb, &field.to_string(), expr);
  }
  qb
}

fn query_condition_builder_expr<'args, DB: sqlx::Database>(
  mut qb: sqlx::QueryBuilder<'args, DB>,
  field: &str,
  expr: &QueryExpr,
) -> sqlx::QueryBuilder<'args, DB> {
  match expr {
    QueryExpr::Value(value) => {
      qb.push(value.into_sql());
    }
    QueryExpr::Field(sub_field) => {
      qb.push(sub_field.to_string());
    }
    QueryExpr::Data(data) => {
      for (i, (data_field, sub_expr)) in data.into_iter().enumerate() {
        if i > 0 {
          qb.push(" AND ");
        }
        qb = query_condition_builder_expr(
          qb,
          &format!("json_extract({field},'$.{data_field}')"),
          sub_expr,
        );
      }
    }
    QueryExpr::Op(op) => match op {
      QueryOp::Eq(sub_expr) => {
        qb.push(field);
        qb.push(" = ");
        qb = query_condition_builder_expr(qb, field, &*sub_expr);
      }
      QueryOp::Neq(sub_expr) => {
        qb.push(field);
        qb.push(" != ");
        qb = query_condition_builder_expr(qb, field, &*sub_expr);
      }
      QueryOp::Gt(sub_expr) => {
        qb.push(field);
        qb.push(" > ");
        qb = query_condition_builder_expr(qb, field, &*sub_expr);
      }
      QueryOp::Lt(sub_expr) => {
        qb.push(field);
        qb.push(" < ");
        qb = query_condition_builder_expr(qb, field, &*sub_expr);
      }
      QueryOp::Gte(sub_expr) => {
        qb.push(field);
        qb.push(" >= ");
        qb = query_condition_builder_expr(qb, field, &*sub_expr);
      }
      QueryOp::Lte(sub_expr) => {
        qb.push(field);
        qb.push(" <= ");
        qb = query_condition_builder_expr(qb, field, &*sub_expr);
      }
      QueryOp::Like(sub_expr) => {
        qb.push(field);
        qb.push(" LIKE ");
        qb = query_condition_builder_expr(qb, field, &*sub_expr);
      }
      QueryOp::In(sub_exprs) => {
        qb.push(field);
        qb.push(" IN (");
        for (i, sub_expr) in sub_exprs.into_iter().enumerate() {
          if i > 0 {
            qb.push(", ");
          }
          qb = query_condition_builder_expr(qb, field, sub_expr);
        }
        qb.push(")");
      }
      QueryOp::And(sub_exprs) => {
        qb.push(" (");
        for (i, sub_expr) in sub_exprs.into_iter().enumerate() {
          if i > 0 {
            qb.push(" AND ");
          }
          qb = query_condition_builder_expr(qb, field, sub_expr);
        }
        qb.push(")");
      }
      QueryOp::Or(sub_exprs) => {
        qb.push(" (");
        for (i, sub_expr) in sub_exprs.into_iter().enumerate() {
          if i > 0 {
            qb.push(" OR ");
          }
          qb = query_condition_builder_expr(qb, field, sub_expr);
        }
        qb.push(")");
      }
      QueryOp::Not(sub_expr) => {
        qb.push(" NOT ");
        qb = query_condition_builder_expr(qb, field, &*sub_expr);
      }
    },
  }
  qb
}
