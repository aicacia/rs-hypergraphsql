#[derive(sqlx::FromRow)]
pub struct NodeEdgeRow {
  pub from_node_id: i64,
  pub from_node_uri: String,
  pub from_node_data: String,
  pub from_node_updated_at: i64,
  pub from_node_created_at: i64,

  pub to_node_id: i64,
  pub to_node_uri: String,
  pub to_node_data: String,
  pub to_node_updated_at: i64,
  pub to_node_created_at: i64,

  pub edge_id: i64,
  pub edge_uri: String,
  pub edge_data: Option<String>,
  pub edge_updated_at: i64,
  pub edge_created_at: i64,
}

pub async fn get_related_by_node_id(
  pool: &sqlx::SqlitePool,
  node_id: i64,
) -> sqlx::Result<Vec<NodeEdgeRow>> {
  sqlx::query_as!(
    NodeEdgeRow,
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
       JOIN nodes to_node ON to_node.id = edge.to_node_id
       WHERE from_node.id = $1 OR to_node.id = $1;"#,
    node_id
  )
  .fetch_all(pool)
  .await
}

pub async fn get_related_by_node_uri(
  pool: &sqlx::SqlitePool,
  node_uri: &str,
) -> sqlx::Result<Vec<NodeEdgeRow>> {
  sqlx::query_as!(
    NodeEdgeRow,
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
       JOIN nodes to_node ON to_node.id = edge.to_node_id
       WHERE from_node.uri = $1 OR to_node.uri = $1;"#,
    node_uri
  )
  .fetch_all(pool)
  .await
}

pub async fn get_related_by_from_node_id(
  pool: &sqlx::SqlitePool,
  from_node_id: i64,
) -> sqlx::Result<Vec<NodeEdgeRow>> {
  sqlx::query_as!(
    NodeEdgeRow,
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
       JOIN nodes to_node ON to_node.id = edge.to_node_id
       WHERE from_node.id = $1;"#,
    from_node_id
  )
  .fetch_all(pool)
  .await
}

pub async fn get_related_by_from_node_uri(
  pool: &sqlx::SqlitePool,
  from_node_uri: &str,
) -> sqlx::Result<Vec<NodeEdgeRow>> {
  sqlx::query_as!(
    NodeEdgeRow,
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
       JOIN nodes to_node ON to_node.id = edge.to_node_id
       WHERE from_node.uri = $1;"#,
    from_node_uri
  )
  .fetch_all(pool)
  .await
}

pub async fn get_related_by_to_node_id(
  pool: &sqlx::SqlitePool,
  to_node_id: i64,
) -> sqlx::Result<Vec<NodeEdgeRow>> {
  sqlx::query_as!(
    NodeEdgeRow,
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
       JOIN nodes to_node ON to_node.id = edge.to_node_id
       WHERE to_node.id = $1;"#,
    to_node_id
  )
  .fetch_all(pool)
  .await
}

pub async fn get_related_by_to_node_uri(
  pool: &sqlx::SqlitePool,
  to_node_uri: &str,
) -> sqlx::Result<Vec<NodeEdgeRow>> {
  sqlx::query_as!(
    NodeEdgeRow,
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
       JOIN nodes to_node ON to_node.id = edge.to_node_id
       WHERE to_node.uri = $1;"#,
    to_node_uri
  )
  .fetch_all(pool)
  .await
}

pub async fn get_related_by_edge_id(
  pool: &sqlx::SqlitePool,
  edge_id: i64,
) -> sqlx::Result<Vec<NodeEdgeRow>> {
  sqlx::query_as!(
    NodeEdgeRow,
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
       JOIN nodes to_node ON to_node.id = edge.to_node_id
       WHERE edge.id = $1;"#,
    edge_id
  )
  .fetch_all(pool)
  .await
}

pub async fn get_related_by_edge_uri(
  pool: &sqlx::SqlitePool,
  edge_uri: &str,
) -> sqlx::Result<Vec<NodeEdgeRow>> {
  sqlx::query_as!(
    NodeEdgeRow,
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
       JOIN nodes to_node ON to_node.id = edge.to_node_id
       WHERE edge.uri = $1;"#,
    edge_uri
  )
  .fetch_all(pool)
  .await
}

pub async fn get_related_by_node_id_and_edge_uri(
  pool: &sqlx::SqlitePool,
  node_id: i64,
  edge_uri: &str,
) -> sqlx::Result<Vec<NodeEdgeRow>> {
  sqlx::query_as!(
    NodeEdgeRow,
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
       JOIN nodes to_node ON to_node.id = edge.to_node_id
       WHERE (from_node.id = $1 OR to_node.id = $1) AND edge.uri = $2;"#,
    node_id,
    edge_uri
  )
  .fetch_all(pool)
  .await
}

pub async fn get_related_by_from_node_id_and_edge_uri(
  pool: &sqlx::SqlitePool,
  from_node_id: i64,
  edge_uri: &str,
) -> sqlx::Result<Vec<NodeEdgeRow>> {
  sqlx::query_as!(
    NodeEdgeRow,
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
       JOIN nodes to_node ON to_node.id = edge.to_node_id
       WHERE from_node.id = $1 AND edge.uri = $2;"#,
    from_node_id,
    edge_uri
  )
  .fetch_all(pool)
  .await
}

pub async fn get_related_by_to_node_id_and_edge_uri(
  pool: &sqlx::SqlitePool,
  to_node_id: i64,
  edge_uri: &str,
) -> sqlx::Result<Vec<NodeEdgeRow>> {
  sqlx::query_as!(
    NodeEdgeRow,
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
       JOIN nodes to_node ON to_node.id = edge.to_node_id
       WHERE to_node.id = $1 AND edge.uri = $2;"#,
    to_node_id,
    edge_uri
  )
  .fetch_all(pool)
  .await
}
