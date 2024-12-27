use ccfolia_log_parser::error::MyError;
use ccfolia_log_parser::{get_log_summary, get_logs, get_pc_summary};
use std::{env, io};

fn main() {
    let original_logs = match get_logs(env::args()) {
        Ok(logs) => logs,
        Err(e) => match e {
            MyError::Io(e) => {
                eprintln!("ファイルが開けませんでした：{}", e);
                println!("Enterキーで終了します");
                let mut a = "".to_string();
                io::stdin().read_line(&mut a).expect("Failed to read line");
                return;
            }
            MyError::Parse(e) => {
                eprintln!("Parse error:{}", e);
                println!("Enterキーで終了します");
                let mut a = "".to_string();
                io::stdin().read_line(&mut a).expect("Failed to read line");
                return;
            }
        },
    };

    let _logs = get_log_summary(&original_logs);

    let log_summary_by_name = get_pc_summary(&original_logs);

    for (name, log_summary) in log_summary_by_name {
        println!("{}：\n{}", name, log_summary);
    }

    println!("Enterキーで終了します");
    let mut a = "".to_string();
    io::stdin().read_line(&mut a).expect("Failed to read line");
}
