use ccfolia_log_parser::error::Error;
use ccfolia_log_parser::{get_log_summary, get_logs, get_pc_summary};

fn main() {
    let filename = "data/log3.html";
    let original_logs = match get_logs(filename) {
        Ok(logs) => logs,
        Err(e) => match e {
            Error::Io(e) => {
                panic!("ファイルが開けませんでした：{}", e);
            }
            Error::Anyhow(e) => {
                panic!("error:{}", e);
            }
        },
    };

    let _logs = get_log_summary(&original_logs);

    let log_summary_by_name = get_pc_summary(&original_logs);

    for (name, log_summary) in log_summary_by_name {
        println!("{}:\n{}", name, log_summary);
    }
}
