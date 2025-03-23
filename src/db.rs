use std::path::PathBuf;
use surrealdb::{
    Surreal,
    engine::any::{Any, connect},
};

#[allow(dead_code)]
struct Database {
    client: Surreal<Any>,
}

impl Database {
    /// Creates a new instance of the `Database`.
    ///
    /// This function initializes a new `Database` object and connects it to the
    /// appropriate SurrealDB database. If the application is in test mode, it connects
    /// to an in-memory database; otherwise, it connects to a file-based SurrealKV database.
    ///
    /// # Returns
    ///
    /// Returns a `Database` instance with an active connection to SurrealDB.
    #[allow(dead_code)]
    pub async fn new() -> Self {
        Self {
            client: Self::connect().await,
        }
    }

    /// Connects to the database and sets namespace to "tap_ns" and database to "tap_db"
    ///
    /// If the `test` config flag is set, an in-memory database is used.
    /// Otherwise, a SurrealKV file-based database is used.
    #[allow(dead_code)]
    async fn connect() -> Surreal<Any> {
        let client: Surreal<Any> = connect(Self::url())
            .await
            .expect("Could not connect to database");
        client
            .use_ns("tap_ns")
            .use_db("tap_db")
            .await
            .expect("Could not set namespace and database");
        client
    }

    /// Returns the path to the SurrealKV file
    ///
    /// The path is in the same directory as the executable
    #[allow(dead_code)]
    fn get_db_file_path() -> PathBuf {
        let exe_path = std::env::current_exe().expect("Could not get executable path");
        let dir_path = exe_path
            .parent()
            .expect("Could not get executable directory");
        dir_path.join(".tap_db")
    }

    /// Connects to the database
    ///
    /// If the application is in test mode (`test` config flag is set), an in-memory database is used.
    /// Otherwise, a SurrealKV file-based database is used.
    #[allow(dead_code)]
    fn url() -> String {
        if cfg!(test) {
            // For testing, use an in-memory database
            "mem://".to_string()
        } else {
            // Use file-based database
            format!("surrealkv://{}", Self::get_db_file_path().display())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn url_returns_mem_connection_for_test() {
        let url = Database::url();
        assert_eq!(url, "mem://");
    }

    #[tokio::test]
    #[ignore]
    async fn use_file_based_db_in_non_test() {
        // TODO: Fix this, need to read how to make a specific test act as if it was not in test environment so cfg!(test) returns false
        // Set test flag to false
        let url = Database::url();
        assert!(url.starts_with("surrealkv://"));
    }

    #[test]
    fn db_file_path_ends_with_tap_db() {
        let db_path = Database::get_db_file_path();
        assert!(
            db_path.ends_with(".tap_db"),
            "Database path does not end with .tap_db"
        );
    }
}
