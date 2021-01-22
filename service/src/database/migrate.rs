use rust_embed::RustEmbed;
use tokio_postgres::IsolationLevel;

use super::Database;

/// The embedded migrations files to apply
#[derive(RustEmbed)]
#[folder = "migrations/"]
struct Migrations;

/// Migrate the database pointed to by the provided database connection to the latest version of the
/// schema
///
/// # Parameters
/// - `d` - The database to migrate
#[tracing::instrument(skip(d))]
pub async fn migrate(d: &dyn Database) {
    let mut conn = d.checkout().await;
    let tx = conn
        .build_transaction()
        .isolation_level(IsolationLevel::Serializable)
        .read_only(false)
        .deferrable(false)
        .start()
        .await
        .expect("Failed to start transaction");

    create_migrations_table(&tx).await;
    apply_migrations(&tx).await;

    tx.commit().await.expect("Failed to commit transaction");
}

/// Ensure that the migrations table exists to record the migrations that have been applied.
///
/// # Parameters
/// - `tx` An open database transaction in which to execute the SQL.
async fn create_migrations_table(tx: &tokio_postgres::Transaction<'_>) {
    tracing::trace!("Ensuring the migrations table exists");
    tx.execute(
        "CREATE TABLE IF NOT EXISTS __migrations(
          migration_file TEXT PRIMARY KEY,
          sequence SERIAL NOT NULL,
          executed TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
          executed_from TEXT NOT NULL DEFAULT inet_client_addr()
        )",
        &[],
    )
    .await
    .expect("Failed to create __migrations table");

    tracing::trace!("Locking the migrations table");
    tx.execute("LOCK TABLE __migrations IN EXCLUSIVE MODE", &[])
        .await
        .expect("Failed to lock __migrations table");
}

/// Generate a list of all the migrations that are known.
///
/// These are loaded from the embedded SQL files, and indicate everything that can be applied
///
/// # Returns
/// The list of migration files to apply
fn list_all_migrations() -> Vec<String> {
    tracing::trace!("Listing all migrations that can be applied");
    let mut migrations: Vec<String> = Migrations::iter().map(|f| f.to_string()).collect();
    migrations.sort();
    tracing::debug!(migrations = ?migrations, "All known migrations");

    migrations
}

/// Get a list of the migrations that have been previously applied
///
/// This list is loaded from the database and indicates which of the migrations have been applied
/// before
///
/// # Parameters
/// - `transaction` - The database transaction we're working in
///
/// # Returns
/// The list of migrations that have been applied before
///
/// # Errors
/// If an error occurs executing the SQL then return an error
async fn list_applied_migrations(tx: &tokio_postgres::Transaction<'_>) -> Vec<String> {
    tracing::trace!("Listing the applied migrations");

    let migrations = tx
        .query("SELECT migration_file FROM __migrations", &[])
        .await
        .expect("Failed to list applied migrations")
        .iter()
        .map(|row| row.get::<&str, String>("migration_file"))
        .collect::<Vec<String>>();
    tracing::debug!(migrations = ?migrations, "Migrations already applied");

    migrations
}

/// Actually apply the migrations that are outstanding
///
/// # Parameters
/// - `transaction` - The database transaction we're working in
///
/// # Returns
/// On successfully applying the migrations, return a void value
///
/// # Errors
/// If an error occurs applying the migrations then return an error
async fn apply_migrations(tx: &tokio_postgres::Transaction<'_>) {
    let all_migrations = list_all_migrations();
    let applied_migrations = list_applied_migrations(tx).await;

    let mut count: u32 = 0;
    for migration in &all_migrations {
        if applied_migrations.contains(migration) {
            tracing::debug!(migration = ?migration, "Migration already applied");
        } else {
            tracing::debug!(migration = ?migration, "Applying migration");
            let contents = Migrations::get(migration).expect("Failed to load migration");

            tx.batch_execute(std::str::from_utf8(&contents).expect("Failed to load migration"))
                .await
                .expect("Failed to apply migration");
            tx.execute(
                "INSERT INTO __migrations(migration_file) VALUES ($1)",
                &[migration],
            )
            .await
            .expect("Failed to record applied migration");
            count += 1;
        }
    }
    tracing::info!(count = ?count, total = ?(all_migrations.len()), "Applied migrations");
}
