use std::collections::HashMap;

use super::query::{Query, QueryExpr, QueryField, QueryOp};

pub fn query_builder<'args, DB: sqlx::Database>(query: &Query) -> sqlx::QueryBuilder<'args, DB> {
  match query {
    Query::Node(inner) => query_inner_builder(
      sqlx::QueryBuilder::new("SELECT node.* FROM nodes node"),
      inner,
    ),
    Query::Edge(inner) => query_inner_builder(
      sqlx::QueryBuilder::new("SELECT edge.* FROM nodes edge"),
      inner,
    ),
    Query::NodeEdge(inner) => query_inner_builder(
      sqlx::QueryBuilder::new(
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
      inner,
    ),
  }
}

fn query_inner_builder<'args, DB: sqlx::Database>(
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
    qb = query_inner_builder_expr(qb, &field.to_string(), expr);
  }
  qb
}

fn query_inner_builder_expr<'args, DB: sqlx::Database>(
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
        qb = query_inner_builder_expr(
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
        qb = query_inner_builder_expr(qb, field, &*sub_expr);
      }
      QueryOp::Neq(sub_expr) => {
        qb.push(field);
        qb.push(" != ");
        qb = query_inner_builder_expr(qb, field, &*sub_expr);
      }
      QueryOp::Gt(sub_expr) => {
        qb.push(field);
        qb.push(" > ");
        qb = query_inner_builder_expr(qb, field, &*sub_expr);
      }
      QueryOp::Lt(sub_expr) => {
        qb.push(field);
        qb.push(" < ");
        qb = query_inner_builder_expr(qb, field, &*sub_expr);
      }
      QueryOp::Gte(sub_expr) => {
        qb.push(field);
        qb.push(" >= ");
        qb = query_inner_builder_expr(qb, field, &*sub_expr);
      }
      QueryOp::Lte(sub_expr) => {
        qb.push(field);
        qb.push(" <= ");
        qb = query_inner_builder_expr(qb, field, &*sub_expr);
      }
      QueryOp::Like(sub_expr) => {
        qb.push(field);
        qb.push(" LIKE ");
        qb = query_inner_builder_expr(qb, field, &*sub_expr);
      }
      QueryOp::In(sub_exprs) => {
        qb.push(field);
        qb.push(" IN (");
        for (i, sub_expr) in sub_exprs.into_iter().enumerate() {
          if i > 0 {
            qb.push(", ");
          }
          qb = query_inner_builder_expr(qb, field, sub_expr);
        }
        qb.push(")");
      }
      QueryOp::And(sub_exprs) => {
        qb.push(" (");
        for (i, sub_expr) in sub_exprs.into_iter().enumerate() {
          if i > 0 {
            qb.push(" AND ");
          }
          qb = query_inner_builder_expr(qb, field, sub_expr);
        }
        qb.push(")");
      }
      QueryOp::Or(sub_exprs) => {
        qb.push(" (");
        for (i, sub_expr) in sub_exprs.into_iter().enumerate() {
          if i > 0 {
            qb.push(" OR ");
          }
          qb = query_inner_builder_expr(qb, field, sub_expr);
        }
        qb.push(")");
      }
      QueryOp::Not(sub_expr) => {
        qb.push(" NOT ");
        qb = query_inner_builder_expr(qb, field, &*sub_expr);
      }
    },
  }
  qb
}
