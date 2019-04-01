use rocket_contrib::database;

use diesel::SqliteConnection;

#[database("core_db")]
pub struct CoreDbConn(SqliteConnection);
