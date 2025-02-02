use hypergraphsql::*;
use serde::{Deserialize, Serialize};

static NODE_USER_URI: &str = "user";
static EDGE_FOLLOWS_URI: &str = "follows";

#[derive(Serialize, Deserialize)]
struct Follows;

#[derive(Serialize, Deserialize)]
struct UserInfo {
  name: String,
}

#[derive(Serialize, Deserialize)]
struct User {
  info: UserInfo,
}

impl User {
  fn new(name: impl Into<String>) -> Self {
    User {
      info: UserInfo { name: name.into() },
    }
  }
}

#[tokio::test]
async fn test_query() -> sqlx::Result<()> {
  let temp_path = tempfile::NamedTempFile::with_suffix("-test.db")?.into_temp_path();
  let filename = temp_path.as_os_str().to_string_lossy();

  println!("filename: {}", filename);

  let pool = create(&filename, false).await?;

  let user_a = create_node(&pool, NODE_USER_URI, User::new("a")).await?;
  let user_b = create_node(&pool, NODE_USER_URI, User::new("b")).await?;

  create_edge(&pool, &user_a, &user_b, EDGE_FOLLOWS_URI, None::<Follows>).await?;

  let query_json = serde_json::json!({
    "from_node.uri": {"eq": "user"},
    "from_node.data": {
      "info.name": {"eq": "a"}
    }
  });
  let query = serde_json::from_value::<Query>(query_json).expect("failed to parse Query JSON");
  println!("{}", query.sql());
  let related = query.node_edges::<User, User, Follows>(&pool).await?;

  assert_eq!(related.len(), 1);
  let node_edge = related.get(0).unwrap();
  assert_eq!(node_edge.from_node.data.info.name, "a");
  assert_eq!(node_edge.to_node.data.info.name, "b");

  let deleted = delete_nodes::<User>(&pool, &vec![user_a.id, user_b.id]).await?;
  assert_eq!(deleted.len(), 2);

  Ok(())
}
