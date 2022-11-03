pub mod dao;
pub mod utils;
use crate::dao::data_object::{get_db_connection, TestRun, TestTask};
use crate::utils::utils::{get_env_var, get_full_db_path, load_env_file};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_loads_env_file() {
        let vars = load_env_file();
        assert_eq!(
            vars.get(&"RPC_URL".to_string()),
            Some(&"https://mainnet.aurora.dev:443/".to_string())
        );
        assert_eq!(
            vars.get(&"NETWORK_NAME".to_string()),
            Some(&"mainnet_aurora_plus".to_string())
        )
    }

    #[test]
    fn it_loads_test_runs_and_filter_tasks() {
        use std::cmp::Ordering;
        use tracing::{debug, Level};
        use tracing_subscriber::FmtSubscriber;
        let mut network_name =
            get_env_var(&"NETWORK_NAME".to_string()).unwrap_or("mainnet_aurora_plus".to_string());
        let full_db_path = get_full_db_path().unwrap();
        let conn = get_db_connection(&full_db_path).unwrap();
        let runs_table = get_env_var(&"RUNS_TABLE".to_string())
            .unwrap_or("aurora_relayer_test_runs".to_string());
        let test_run = TestRun::new(&conn, network_name, runs_table).unwrap();
        assert_eq!(test_run.db_id, 20);
        network_name = get_env_var(&"NETWORK_NAME".to_string()).unwrap();
        assert_eq!(test_run.network, network_name);
        let number_of_data_groups = test_run.tasks[0].data_groups.len();
        let result = number_of_data_groups.cmp(&0);
        assert_eq!(Ordering::Greater, result);
        let task_type = String::from("transferNtimes");
        let task: TestTask = test_run
            .filter_tasks_with_limit_one(task_type.clone())
            .unwrap();
        assert_eq!(task.task_type, task_type);
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .finish();
        debug!(
            "{}, {}, {}, {}, {}",
            task.db_id, task.task_type, task.parameters, task.begin, task.end
        );
        conn.close();
    }
}
