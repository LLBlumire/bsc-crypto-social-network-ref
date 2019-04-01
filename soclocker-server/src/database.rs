//! This module contains the database connection structure.

use rocket_contrib::database;

use diesel::SqliteConnection;

/// Constructs the Database Connection from the supplied configuration. During
/// development this is an SqliteConnection, however it will be swapped for a
/// MySqlConnection closer to deployment.
#[database("core_db")]
pub struct CoreDbConn(SqliteConnection);
