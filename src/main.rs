use ccfolia_log_parser::error::MyError;
use ccfolia_log_parser::log_summary::{LogSummary, UserChoice}; // Added UserChoice and LogSummary
use ccfolia_log_parser::{get_log_summary, get_logs, get_pc_summary};
use std::{env, io, io::Write}; // Added io::Write

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

    println!("どの結果の技能一覧を表示しますか？");
    println!("1: 成功");
    println!("2: 失敗");
    println!("3: クリティカル");
    println!("4: ファンブル");
    println!("5: 表示しない");
    print!("入力してください：");
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed before input

    let mut choice_str = String::new();
    io::stdin()
        .read_line(&mut choice_str)
        .expect("Failed to read line");

    let chosen_skill_display_index: Option<usize> = match choice_str.trim().parse::<u32>() {
        Ok(n) => match n {
            1 => Some(UserChoice::Success as usize), // 0
            2 => Some(UserChoice::Failure as usize), // 1
            3 => Some(UserChoice::Critical as usize), // 2
            4 => Some(UserChoice::Fumble as usize),   // 3
            5 => None,
            _ => {
                println!("無効な選択です。技能一覧は表示されません。");
                None
            }
        },
        Err(_) => {
            println!("無効な入力です。技能一覧は表示されません。");
            None
        }
    };

    for (name, log_summary) in log_summary_by_name {
        // Now using format_with_skills
        println!(
            "{}：\n{}",
            name,
            log_summary.format_with_skills(chosen_skill_display_index)
        );
    }

    println!("Enterキーで終了します");
    let mut a = "".to_string();
    io::stdin().read_line(&mut a).expect("Failed to read line");
}
