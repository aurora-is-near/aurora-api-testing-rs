pub mod db_client {
    use std::{path::PathBuf };
    use rusqlite::{Connection, Result};
    use tracing::{debug, Level};
    use tracing_subscriber::FmtSubscriber;

    pub struct TestData {
        pub db_id: i32,
        pub group: i32,
        pub name: String,
        pub content: String,
    }

    pub struct TestDataGroup {
        pub db_id: i32,
        pub data: Vec<TestData>
    }

    pub struct TestTask {
        pub db_id: i32,
        pub task_type: String,
        pub parameters: String,
        pub begin: String,
        pub end: String,
        pub data_groups: Vec<TestDataGroup>
    }

    pub struct TestRun {
        pub db_id: i32,
        pub id: String,
        pub network: String,
        pub tasks: Vec<TestTask>,
    }

    impl TestRun {
        pub fn new(
            conn: &Connection,
            network_name: String,
            runs_table: String
        ) -> Result<TestRun, rusqlite::Error> {
            let subscriber = FmtSubscriber::builder()
                .with_max_level(Level::TRACE)
                .finish();
            tracing::subscriber::set_global_default(subscriber)
                .expect("setting default subscriber failed");
            debug!(
                "network name is : {}, runs table name is : {}", &network_name, &runs_table
            );

            let query = format!(
                "SELECT * FROM {} WHERE test_run_network = '{}' ORDER BY test_run_db_id DESC LIMIT 1",
                runs_table.as_str(),
                network_name.as_str()
            ).clone();
            debug!(
                "{}", &query
            );

            let mut stmt = conn.prepare(&query)?;
            let mut task_run_iter = stmt.query_map([], |row| {
                Ok(
                    TestRun {
                        db_id: row.get(0)?,
                        id: row.get(1)?,
                        network: row.get(2)?,
                        tasks: vec![]
                    }
                )
            })?;
            //TODO: add tasks, data groups and test data
            task_run_iter.next().unwrap()
        }
    }

    pub fn get_db_connection(db_path: &PathBuf) -> Result<Connection, rusqlite::Error> {
        Ok(Connection::open(db_path)?)
    }
}
