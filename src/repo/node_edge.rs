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
        fn.id as from_node_id, 
        fn.uri as from_node_uri,
        fn.data as from_node_data,
        fn.created_at as from_node_created_at,
        fn.updated_at as from_node_updated_at,

        tn.id as to_node_id, 
        tn.uri as to_node_uri,
        tn.data as to_node_data,
        tn.created_at as to_node_created_at,
        tn.updated_at as to_node_updated_at,

        e.id as edge_id, 
        e.uri as edge_uri,
        e.data as edge_data,
        e.created_at as edge_created_at,
        e.updated_at as edge_updated_at
       FROM edges e
       JOIN nodes fn ON fn.id = e.from_node_id
       JOIN nodes tn ON tn.id = e.to_node_id
       WHERE fn.id = $1 OR tn.id = $1;"#,
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
        fn.id as from_node_id, 
        fn.uri as from_node_uri,
        fn.data as from_node_data,
        fn.created_at as from_node_created_at,
        fn.updated_at as from_node_updated_at,

        tn.id as to_node_id, 
        tn.uri as to_node_uri,
        tn.data as to_node_data,
        tn.created_at as to_node_created_at,
        tn.updated_at as to_node_updated_at,
        
        e.id as edge_id, 
        e.uri as edge_uri,
        e.data as edge_data,
        e.created_at as edge_created_at,
        e.updated_at as edge_updated_at
       FROM edges e
       JOIN nodes fn ON fn.id = e.from_node_id
       JOIN nodes tn ON tn.id = e.to_node_id
       WHERE fn.uri = $1 OR tn.uri = $1;"#,
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
        fn.id as from_node_id, 
        fn.uri as from_node_uri,
        fn.data as from_node_data,
        fn.created_at as from_node_created_at,
        fn.updated_at as from_node_updated_at,

        tn.id as to_node_id, 
        tn.uri as to_node_uri,
        tn.data as to_node_data,
        tn.created_at as to_node_created_at,
        tn.updated_at as to_node_updated_at,
        
        e.id as edge_id, 
        e.uri as edge_uri,
        e.data as edge_data,
        e.created_at as edge_created_at,
        e.updated_at as edge_updated_at
       FROM edges e
       JOIN nodes fn ON fn.id = e.from_node_id
       JOIN nodes tn ON tn.id = e.to_node_id
       WHERE fn.id = $1;"#,
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
        fn.id as from_node_id, 
        fn.uri as from_node_uri,
        fn.data as from_node_data,
        fn.created_at as from_node_created_at,
        fn.updated_at as from_node_updated_at,

        tn.id as to_node_id, 
        tn.uri as to_node_uri,
        tn.data as to_node_data,
        tn.created_at as to_node_created_at,
        tn.updated_at as to_node_updated_at,
        
        e.id as edge_id, 
        e.uri as edge_uri,
        e.data as edge_data,
        e.created_at as edge_created_at,
        e.updated_at as edge_updated_at
       FROM edges e
       JOIN nodes fn ON fn.id = e.from_node_id
       JOIN nodes tn ON tn.id = e.to_node_id
       WHERE fn.uri = $1;"#,
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
        fn.id as from_node_id, 
        fn.uri as from_node_uri,
        fn.data as from_node_data,
        fn.created_at as from_node_created_at,
        fn.updated_at as from_node_updated_at,

        tn.id as to_node_id, 
        tn.uri as to_node_uri,
        tn.data as to_node_data,
        tn.created_at as to_node_created_at,
        tn.updated_at as to_node_updated_at,
        
        e.id as edge_id, 
        e.uri as edge_uri,
        e.data as edge_data,
        e.created_at as edge_created_at,
        e.updated_at as edge_updated_at
       FROM edges e
       JOIN nodes fn ON fn.id = e.from_node_id
       JOIN nodes tn ON tn.id = e.to_node_id
       WHERE tn.id = $1;"#,
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
        fn.id as from_node_id, 
        fn.uri as from_node_uri,
        fn.data as from_node_data,
        fn.created_at as from_node_created_at,
        fn.updated_at as from_node_updated_at,

        tn.id as to_node_id, 
        tn.uri as to_node_uri,
        tn.data as to_node_data,
        tn.created_at as to_node_created_at,
        tn.updated_at as to_node_updated_at,
        
        e.id as edge_id, 
        e.uri as edge_uri,
        e.data as edge_data,
        e.created_at as edge_created_at,
        e.updated_at as edge_updated_at
       FROM edges e
       JOIN nodes fn ON fn.id = e.from_node_id
       JOIN nodes tn ON tn.id = e.to_node_id
       WHERE tn.uri = $1;"#,
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
        fn.id as from_node_id, 
        fn.uri as from_node_uri,
        fn.data as from_node_data,
        fn.created_at as from_node_created_at,
        fn.updated_at as from_node_updated_at,

        tn.id as to_node_id, 
        tn.uri as to_node_uri,
        tn.data as to_node_data,
        tn.created_at as to_node_created_at,
        tn.updated_at as to_node_updated_at,
        
        e.id as edge_id, 
        e.uri as edge_uri,
        e.data as edge_data,
        e.created_at as edge_created_at,
        e.updated_at as edge_updated_at
       FROM edges e
       JOIN nodes fn ON fn.id = e.from_node_id
       JOIN nodes tn ON tn.id = e.to_node_id
       WHERE e.id = $1;"#,
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
        fn.id as from_node_id, 
        fn.uri as from_node_uri,
        fn.data as from_node_data,
        fn.created_at as from_node_created_at,
        fn.updated_at as from_node_updated_at,

        tn.id as to_node_id, 
        tn.uri as to_node_uri,
        tn.data as to_node_data,
        tn.created_at as to_node_created_at,
        tn.updated_at as to_node_updated_at,
        
        e.id as edge_id, 
        e.uri as edge_uri,
        e.data as edge_data,
        e.created_at as edge_created_at,
        e.updated_at as edge_updated_at
       FROM edges e
       JOIN nodes fn ON fn.id = e.from_node_id
       JOIN nodes tn ON tn.id = e.to_node_id
       WHERE e.uri = $1;"#,
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
        fn.id as from_node_id, 
        fn.uri as from_node_uri,
        fn.data as from_node_data,
        fn.created_at as from_node_created_at,
        fn.updated_at as from_node_updated_at,

        tn.id as to_node_id, 
        tn.uri as to_node_uri,
        tn.data as to_node_data,
        tn.created_at as to_node_created_at,
        tn.updated_at as to_node_updated_at,
        
        e.id as edge_id, 
        e.uri as edge_uri,
        e.data as edge_data,
        e.created_at as edge_created_at,
        e.updated_at as edge_updated_at
       FROM edges e
       JOIN nodes fn ON fn.id = e.from_node_id
       JOIN nodes tn ON tn.id = e.to_node_id
       WHERE (fn.id = $1 OR tn.id = $1) AND e.uri = $2;"#,
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
        fn.id as from_node_id, 
        fn.uri as from_node_uri,
        fn.data as from_node_data,
        fn.created_at as from_node_created_at,
        fn.updated_at as from_node_updated_at,

        tn.id as to_node_id, 
        tn.uri as to_node_uri,
        tn.data as to_node_data,
        tn.created_at as to_node_created_at,
        tn.updated_at as to_node_updated_at,
        
        e.id as edge_id, 
        e.uri as edge_uri,
        e.data as edge_data,
        e.created_at as edge_created_at,
        e.updated_at as edge_updated_at
       FROM edges e
       JOIN nodes fn ON fn.id = e.from_node_id
       JOIN nodes tn ON tn.id = e.to_node_id
       WHERE fn.id = $1 AND e.uri = $2;"#,
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
        fn.id as from_node_id, 
        fn.uri as from_node_uri,
        fn.data as from_node_data,
        fn.created_at as from_node_created_at,
        fn.updated_at as from_node_updated_at,

        tn.id as to_node_id, 
        tn.uri as to_node_uri,
        tn.data as to_node_data,
        tn.created_at as to_node_created_at,
        tn.updated_at as to_node_updated_at,
        
        e.id as edge_id, 
        e.uri as edge_uri,
        e.data as edge_data,
        e.created_at as edge_created_at,
        e.updated_at as edge_updated_at
       FROM edges e
       JOIN nodes fn ON fn.id = e.from_node_id
       JOIN nodes tn ON tn.id = e.to_node_id
       WHERE tn.id = $1 AND e.uri = $2;"#,
    to_node_id,
    edge_uri
  )
  .fetch_all(pool)
  .await
}
