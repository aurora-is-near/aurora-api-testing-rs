pub mod dao {
    use itertools::Itertools;
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
            runs_table: String,
        ) -> Result<TestRun, rusqlite::Error> {
            //TODO: use https://github.com/SeaQL/sea-query/ for more user friendly queries formatting
            let query = format!(
                "SELECT * FROM {} WHERE test_run_network = '{}' ORDER BY test_run_db_id DESC LIMIT 1",
                runs_table.as_str(),
                network_name.as_str()
            ).clone();
            let subscriber = FmtSubscriber::builder()
                .with_max_level(Level::TRACE)
                .finish();
            tracing::subscriber::set_global_default(subscriber)
                .expect("setting default subscriber failed");
            debug!("{}", query);
            let mut stmt = conn.prepare(&query)?;
            let mut test_run_iter = stmt.query_map([], |row| {
                Ok(
                    TestRun {
                        db_id: row.get(0)?,
                        id: row.get(1)?,
                        network: row.get(2)?,
                        tasks: load_test_tasks(conn, row.get(0)?).unwrap()
                    }
                )
            })?;
            test_run_iter.next().unwrap()
        }
    }

    pub fn load_test_tasks(
        conn: &Connection,
        db_id: String
    ) -> Result<Vec<TestTask>, rusqlite::Error> {
        let test_tasks_query = format!(
            "SELECT aurora_relayer_test_tasks.test_task_db_id, test_task_type, test_task_parameters, test_task_begin, test_task_end FROM aurora_relayer_test_runs_x_tasks LEFT JOIN aurora_relayer_test_tasks ON aurora_relayer_test_runs_x_tasks.test_task_db_id = aurora_relayer_test_tasks.test_task_db_id WHERE aurora_relayer_test_runs_x_tasks.test_run_db_id = {} ORDER BY aurora_relayer_test_tasks.test_task_db_id ASC",
            db_id.parse::<i32>().unwrap()
        ).clone();
        let mut tasks_stmt = conn.prepare(&test_tasks_query)?;
        let tasks_iter = tasks_stmt.query_map([], |row| {
            Ok(
                TestTask {
                    db_id: row.get(0)?,
                    task_type: row.get(1)?,
                    parameters: row.get(2)?,
                    begin: row.get(3)?,
                    end: row.get(4)?,
                    data_groups: load_test_data_groups(&conn, row.get(0)?).unwrap()
                }
            )
        });
        let mut tasks: Vec<TestTask> = Vec::new();
        for task in tasks_iter?.next() {
            tasks.push(task.unwrap());
        }
        Ok(tasks)
    }

    pub fn load_test_data_groups(
        conn: &Connection,
        db_id: String
    ) -> Result<Vec<TestDataGroup>, rusqlite::Error> {
        let test_data = load_test_data(&conn, db_id).unwrap();
        // TODO: https://github.com/rxRust/rxRust
        load_data_groups(test_data)
    }

    pub fn load_test_data(
        conn: &Connection,
        db_id: String
    ) -> Result<Vec<TestData>, rusqlite::Error> {
        let test_data_query = format!(
            "SELECT aurora_relayer_test_tasks_x_data.test_task_db_id, test_data_group, test_data_name, test_data_content FROM aurora_relayer_test_tasks_x_data LEFT JOIN aurora_relayer_test_data ON aurora_relayer_test_tasks_x_data.test_data_db_id = aurora_relayer_test_data.test_data_db_id WHERE aurora_relayer_test_tasks_x_data.test_task_db_id = {} ORDER BY aurora_relayer_test_data.test_data_db_id ASC",
            db_id.parse::<i32>().unwrap()
        ).clone();
        let mut data_stmt = conn.prepare(&test_data_query)?;
        let data_iter = data_stmt.query_map([], |row| {
            Ok(
                TestData {
                    db_id: row.get(0)?,
                    group: row.get(1)?,
                    name: row.get(2)?,
                    content: row.get(3)?,
                }
            )
        });
        let mut data: Vec<TestData> = Vec::new();
        for element in data_iter?.next() {
            data.push(element.unwrap());
        }
        Ok(data)
    }

    pub fn load_data_groups(
        test_data: Vec<TestData>
    ) -> Result<Vec<TestDataGroup>, rusqlite::Error> {
        let test_data_groups: Vec<TestDataGroup> = test_data
            .iter()
            .group_by(|d| (d.group))
            .into_iter()
            .map(|(id, group)| TestDataGroup {
                db_id: id,
                data: group.map(|t| TestData {
                    db_id: t.db_id,
                    group: t.group,
                    name: t.name.clone(),
                    content: t.content.clone(),
                }).collect()
            }).collect();
        Ok(test_data_groups)
    }

    pub fn get_db_connection(db_path: &PathBuf) -> Result<Connection, rusqlite::Error> {
        Ok(Connection::open(db_path)?)
    }
}
