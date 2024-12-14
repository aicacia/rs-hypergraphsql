pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!();

pub async fn create_pool(filename: &str) -> sqlx::Result<sqlx::SqlitePool> {
  let pool = sqlx::sqlite::SqlitePoolOptions::new()
    .connect_with(
      sqlx::sqlite::SqliteConnectOptions::new()
        .filename(filename)
        .create_if_missing(true)
        .foreign_keys(true)
        .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal),
    )
    .await?;
  MIGRATOR.run(&pool).await?;
  Ok(pool)
}

pub async fn pragma(pool: &sqlx::SqlitePool) -> sqlx::Result<()> {
  sqlx::query("PRAGMA journal_mode = wal; PRAGMA synchronous = normal; PRAGMA foreign_keys = on;")
    .execute(pool)
    .await?;
  Ok(())
}
