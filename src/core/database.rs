pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!();

pub async fn create(filename: &str, create_if_missing: bool) -> sqlx::Result<sqlx::SqlitePool> {
  let pool = sqlx::sqlite::SqlitePoolOptions::new()
    .connect_with(
      sqlx::sqlite::SqliteConnectOptions::new()
        .filename(filename)
        .create_if_missing(create_if_missing)
        .foreign_keys(true)
        .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal),
    )
    .await?;
  MIGRATOR.run(&pool).await?;
  Ok(pool)
}

pub async fn pragma<'e, E, DB>(executor: E) -> sqlx::Result<()>
where
  DB: sqlx::Database,
  <DB as sqlx::Database>::Arguments<'e>: sqlx::IntoArguments<'e, DB>,
  E: sqlx::Executor<'e, Database = DB>,
{
  sqlx::query("PRAGMA journal_mode = wal; PRAGMA synchronous = normal; PRAGMA foreign_keys = on;")
    .execute(executor)
    .await?;
  Ok(())
}
