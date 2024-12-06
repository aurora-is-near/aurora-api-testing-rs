#[macro_use]
pub mod helpers;
pub mod models;
pub mod utils;

// local tests for this lib
#[cfg(test)]
mod tests {
    use crate::helpers::TransactionReceipt;
    use crate::models::{get_db_connection, TestRun, TestTask};
    use crate::utils::{get_env_var, get_full_db_path, load_env_file};
    use std::cmp::Ordering;
    // use tracing::{debug, Level};
    // use tracing_subscriber::FmtSubscriber;

    #[test]
    fn it_loads_env_file() {
        let vars = load_env_file();
        // assert_eq!(
        //     vars.get(&"RPC_URL".to_string()),
        //     Some(&"http://142.132.133.160:8545/".to_string())
        // );
        assert_eq!(
            vars.get("NETWORK_NAME"),
            Some(&"mainnet_aurora_plus".to_string())
        )
    }

    #[test]
    fn it_loads_test_runs_and_filter_tasks() {
        let mut network_name =
            get_env_var("NETWORK_NAME").unwrap_or("mainnet_aurora_plus".to_string());
        let full_db_path = get_full_db_path().unwrap();
        let conn = get_db_connection(&full_db_path).unwrap();
        let test_run = TestRun::new(&conn, network_name).unwrap();
        network_name = get_env_var("NETWORK_NAME").unwrap();
        assert_eq!(test_run.network, network_name);
        let number_of_data_groups = test_run.tasks[0].data_groups.len();
        let result = number_of_data_groups.cmp(&0);
        assert_eq!(Ordering::Greater, result);
        let task_type = String::from("transferNtimes");
        let task: TestTask = test_run
            .filter_tasks_with_limit_one(task_type.clone())
            .unwrap();
        assert_eq!(task.task_type, task_type);
        // debug!(
        //     "{}, {}, {}, {}, {}",
        //     task.db_id, task.task_type, task.parameters, task.begin, task.end
        // );
        let data_contents: Vec<String> = task
            .get_test_data_content_array("receipt".to_string())
            .unwrap();
        (0..task.data_groups.len()).for_each(|i| {
            let data_content_by_group_index = task
                .get_test_data_content_by_group_index(i, "receipt".to_string())
                .unwrap();
            assert_eq!(data_contents[i], data_content_by_group_index);
        });
    }

    #[test]
    fn it_loads_transaction_receipts() {
        use std::cmp::Ordering;
        let network_name = get_env_var("NETWORK_NAME").unwrap_or("mainnet_aurora_plus".to_string());
        let full_db_path = get_full_db_path().unwrap();
        let conn = get_db_connection(&full_db_path).unwrap();
        let test_run = TestRun::new(&conn, network_name).unwrap();
        let task = test_run
            .filter_tasks_with_limit_one("transferNtimes".to_string())
            .unwrap();
        let data_contents: Vec<String> = task
            .get_test_data_content_array("receipt".to_string())
            .unwrap();
        let number_of_data_contents = data_contents.len();
        // let subscriber = FmtSubscriber::builder()
        //     .with_max_level(Level::TRACE)
        //     .finish();
        // tracing::subscriber::set_global_default(subscriber)
        //     .expect("setting default subscriber failed");
        // debug!("{}", data_contents[0]);
        let receipts = TransactionReceipt::load(data_contents).unwrap();
        // debug!("{:?}", receipts[0].events[0].args);
        // debug!("Number of receipts: {:?}", receipts.len());
        let number_of_receipts = receipts.len();
        let result = number_of_receipts.cmp(&0);
        assert_eq!(Ordering::Greater, result);
        assert_eq!(number_of_receipts, number_of_data_contents);
    }
}
