# HypergraphSQL

[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-MIT)
[![Test Status](https://github.com/aicacia/rs-auth/workflows/Tests/badge.svg?event=push)](https://github.com/nathanfaucett/rs-auth/actions)

a hypergraph in sqlite

```rust
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

#[tokio::main]
async fn main(pool: sqlx::SqlitePool) -> Result<(), sqlx::Error> {
  // run migrations if not already run
  MIGRATOR.run(&pool).await?;
  // sets sqlite to WAL mode, enables foreign keys, and sets synchronous to normal
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

```
