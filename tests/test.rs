use hypergraphsql::*;
use serde::{Deserialize, Serialize};

static NODE_USER_URI: &str = "user";
static EDGE_FOLLOWS_URI: &str = "follows";

#[derive(Serialize, Deserialize)]
struct Follows;

#[derive(Serialize, Deserialize)]
struct User {
  name: String,
}

#[sqlx::test]
async fn test_query() -> sqlx::Result<()> {
  let temp_path = tempfile::NamedTempFile::new()?.into_temp_path();
  let filename = temp_path.as_os_str().to_string_lossy();

  let pool = create_pool(&filename).await?;

  let user_a = create_node(
    &pool,
    NODE_USER_URI,
    User {
      name: "a".to_string(),
    },
  )
  .await?;
  let user_b = create_node(
    &pool,
    NODE_USER_URI,
    User {
      name: "b".to_string(),
    },
  )
  .await?;

  create_edge(&pool, &user_a, &user_b, EDGE_FOLLOWS_URI, None::<Follows>).await?;

  let query_json = serde_json::json!({
    "type": "node_edge",
    "from_node.uri": {"eq": "user"},
    "from_node.data": {
      "name": {"eq": "a"}
    }
  });
  let query = serde_json::from_value::<Query>(query_json).expect("Query failed to parse JSON");
  println!("{}", query.query_builder().sql());
  let related = query.node_edges::<User, User, Follows>(&pool).await?;

  assert_eq!(related.len(), 1);
  let node_edge = related.get(0).unwrap();
  let user_a = node_edge.from_node();
  let user_b = node_edge.to_node();
  assert_eq!(user_a.data.name, "a");
  assert_eq!(user_b.data.name, "b");

  Ok(())
}
