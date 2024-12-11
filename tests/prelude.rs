use hypergraphsql::prelude::*;
use serde::{Deserialize, Serialize};

static NODE_USER_URI: &str = "user";
static EDGE_FOLLOWS_URI: &str = "follows";

#[derive(Serialize, Deserialize)]
struct Follows;

#[derive(Serialize, Deserialize)]
struct User {
  name: String,
}

#[sqlx::test(migrations = "./migrations")]
async fn test_simple_graph(pool: sqlx::SqlitePool) -> sqlx::Result<()> {
  pragma(&pool).await?;

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

  let related = get_related_by_from_node_id_and_edge_uri::<User, User, Follows>(
    &pool,
    user_a.id,
    EDGE_FOLLOWS_URI,
  )
  .await?;
  assert_eq!(related.len(), 1);
  let node_edge = related.get(0).unwrap();
  let user_a = node_edge.from_node();
  let user_b = node_edge.to_node();
  assert_eq!(user_a.data.name, "a");
  assert_eq!(user_b.data.name, "b");

  Ok(())
}
