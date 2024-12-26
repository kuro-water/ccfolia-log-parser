use ccfolia_log_parser::error::Error;
use ccfolia_log_parser::get_logs;
use ccfolia_log_parser::log_summary::LogSummary;

fn main() {
    let filename = "data/log3.html";
    let logs = match get_logs(filename) {
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

    // for log in &logs {
    //     println!("{}", log);
    // }

    let log_summary = LogSummary::new(&logs);
    println!("{}", log_summary);
}
