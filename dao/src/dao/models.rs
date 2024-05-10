use itertools::Itertools;
use rusqlite::{Connection, Result};
use std::error::Error;
use std::path::PathBuf;
use tracing::debug;

const RUNS_TABLE: &str = "aurora_relayer_test_runs";

#[derive(Clone)]
pub struct TestData {
    pub db_id: i32,
    pub group: i32,
    pub name: String,
    pub content: String,
}

impl TestData {
    pub fn load(conn: &Connection, db_id: i32) -> Result<Vec<TestData>, rusqlite::Error> {
        let test_data_query = format!(
                "SELECT aurora_relayer_test_tasks_x_data.test_task_db_id, test_data_group, test_data_name, test_data_content FROM aurora_relayer_test_tasks_x_data LEFT JOIN aurora_relayer_test_data ON aurora_relayer_test_tasks_x_data.test_data_db_id = aurora_relayer_test_data.test_data_db_id WHERE aurora_relayer_test_tasks_x_data.test_task_db_id = {} ORDER BY aurora_relayer_test_data.test_data_db_id ASC",
                db_id
            ).clone();
        debug!("Selecting test data: {}", test_data_query);
        let mut data_stmt = conn.prepare(&test_data_query)?;
        let data_iter = data_stmt.query_map([], |row| {
            Ok(TestData {
                db_id: row.get::<usize, i32>(0)?,
                group: row.get(1)?,
                name: row.get(2)?,
                content: row.get(3)?,
            })
        });
        let data = Ok(data_iter?
            .filter(|res| res.is_ok())
            .map(|res| res.unwrap())
            .collect());
        data
    }
}

pub struct TestDataGroup {
    pub db_id: i32,
    pub data: Vec<TestData>,
}

impl TestDataGroup {
    pub fn load(conn: &Connection, db_id: i32) -> Result<Vec<TestDataGroup>, rusqlite::Error> {
        let test_data: Vec<TestData> = TestData::load(&conn, db_id).unwrap();
        Self::convert_test_data_to_data_groups(test_data)
    }

    fn convert_test_data_to_data_groups(
        test_data: Vec<TestData>,
    ) -> Result<Vec<TestDataGroup>, rusqlite::Error> {
        let test_data_groups: Vec<TestDataGroup> = test_data
            .iter()
            .group_by(|d| (d.group))
            .into_iter()
            .map(|(id, group)| TestDataGroup {
                db_id: id,
                data: group
                    .map(|t| TestData {
                        db_id: t.db_id,
                        group: t.group,
                        name: t.name.clone(),
                        content: t.content.clone(),
                    })
                    .collect(),
            })
            .collect();
        Ok(test_data_groups)
    }
}

pub struct TestTask {
    pub db_id: i32,
    pub task_type: String,
    pub parameters: String,
    pub begin: String,
    pub end: String,
    pub data_groups: Vec<TestDataGroup>,
}

impl TestTask {
    pub fn load(conn: &Connection, db_id: i32) -> Result<Vec<TestTask>, rusqlite::Error> {
        let test_tasks_query = format!(
                "SELECT aurora_relayer_test_tasks.test_task_db_id, test_task_type, test_task_parameters, test_task_begin, test_task_end FROM aurora_relayer_test_runs_x_tasks LEFT JOIN aurora_relayer_test_tasks ON aurora_relayer_test_runs_x_tasks.test_task_db_id = aurora_relayer_test_tasks.test_task_db_id WHERE aurora_relayer_test_runs_x_tasks.test_run_db_id = {} ORDER BY aurora_relayer_test_tasks.test_task_db_id ASC",
                db_id
            ).clone();
        debug!("Select test tasks: {}", test_tasks_query);
        let mut tasks_stmt = conn.prepare(&test_tasks_query)?;
        let tasks_iter = tasks_stmt.query_map([], |row| {
            let db_id: i32 = row.get::<usize, i32>(0)?;
            Ok(TestTask {
                db_id: row.get::<usize, i32>(0)?,
                task_type: row.get(1)?,
                parameters: row.get(2)?,
                begin: row.get(3)?,
                end: row.get(4)?,
                data_groups: TestDataGroup::load(&conn, db_id).unwrap(),
            })
        })?;
        let mut tasks: Vec<TestTask> = Vec::new();
        for task in tasks_iter {
            tasks.push(task?);
        }
        Ok(tasks)
    }

    pub fn get_test_data_content_by_group_index(
        &self,
        data_group_index: usize,
        test_data_name: String,
    ) -> Result<String, Box<dyn Error>> {
        let test_grp: &Vec<TestData> = &self.data_groups[data_group_index].data;
        let test_data: Vec<String> = test_grp
            .into_iter()
            .filter(|d| d.name == test_data_name)
            .map(|t| t.content.clone())
            .collect();
        Ok(test_data.iter().next().unwrap().to_string())
    }

    pub fn get_test_data_content_array(
        &self,
        test_data_name: String,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        let test_grps: &Vec<TestDataGroup> = &self.data_groups;
        let test_data: Vec<String> = test_grps
            .into_iter()
            .flat_map(|grp| {
                grp.data
                    .iter()
                    .filter(|d| d.name == test_data_name)
                    .map(|c| c.content.clone())
            })
            .collect();
        Ok(test_data)
    }
}

pub struct TestRun {
    pub db_id: i32,
    pub id: String,
    pub network: String,
    pub tasks: Vec<TestTask>,
}

impl TestRun {
    pub fn new(conn: &Connection, network_name: String) -> Result<TestRun, rusqlite::Error> {
        //TODO: use https://github.com/SeaQL/sea-query/ for more user friendly queries formatting
        let test_run_query = format!(
            "SELECT * FROM {} WHERE test_run_network = '{}' ORDER BY test_run_db_id DESC LIMIT 1",
            RUNS_TABLE,
            network_name.as_str()
        )
        .clone();
        debug!("Selecting test runs: {}", test_run_query);
        let mut stmt = conn.prepare(&test_run_query)?;
        let mut test_run_iter = stmt.query_map([], |row| {
            let db_id: i32 = row.get::<usize, i32>(0)?;
            Ok(TestRun {
                db_id: row.get::<usize, i32>(0)?,
                id: row.get::<usize, String>(1)?,
                network: row.get::<usize, String>(2)?,
                tasks: TestTask::load(conn, db_id).unwrap(),
            })
        })?;
        test_run_iter.next().unwrap()
    }

    pub fn filter_tasks_with_limit_one(
        self,
        task_type: String,
    ) -> Result<TestTask, Box<dyn Error>> {
        let test_task: Vec<TestTask> = self
            .tasks
            .into_iter()
            .filter(|task| task.task_type == task_type)
            .map(|task| task)
            .collect();
        Ok(test_task
            .into_iter()
            .next()
            .ok_or("Error: empty tasks".to_string())?)
    }
}

pub fn get_db_connection(db_path: &PathBuf) -> Result<Connection, rusqlite::Error> {
    Ok(Connection::open(db_path)?)
}
