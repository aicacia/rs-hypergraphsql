use std::{future::Future, pin::Pin};

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!();

pub async fn pragma(pool: &sqlx::SqlitePool) -> sqlx::Result<()> {
  sqlx::query("PRAGMA journal_mode = wal; PRAGMA synchronous = normal; PRAGMA foreign_keys = on;")
    .execute(pool)
    .await?;
  Ok(())
}

pub async fn run_transaction<T, F>(
  pool: &sqlx::SqlitePool,
  transaction_fn: F,
) -> Result<T, sqlx::Error>
where
  F: for<'a> FnOnce(
    &'a mut sqlx::Transaction<'_, sqlx::Sqlite>,
  ) -> Pin<Box<dyn Send + Future<Output = sqlx::Result<T>> + 'a>>,
{
  let mut transaction = pool.begin().await?;
  let result = match transaction_fn(&mut transaction).await {
    Ok(result) => result,
    Err(e) => match transaction.rollback().await {
      Ok(_) => return Err(e),
      Err(e2) => {
        // TODO: replace with logger
        println!("Failed to rollback transaction: {}", e2);
        return Err(e);
      }
    },
  };
  transaction.commit().await?;
  Ok(result)
}
