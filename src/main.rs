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

    // Initial Display Loop (Counts Only)
    println!("--- 各プレイヤーの集計結果 ---");
    for (name, log_summary) in &log_summary_by_name {
        println!("{}：\n{}", name, log_summary.to_string()); // Uses Display trait
    }
    println!("---------------------------\n");

    // User Input Section
    println!("どの結果の技能一覧を詳しく見ますか？");
    println!("1: 成功");
    println!("2: 失敗");
    println!("3: クリティカル");
    println!("4: ファンブル");
    println!("5: 詳細を見ない");
    print!("入力してください：");
    io::stdout().flush().unwrap();

    let mut choice_str = String::new();
    io::stdin()
        .read_line(&mut choice_str)
        .expect("Failed to read line");

    let chosen_skill_display_index: Option<usize> = match choice_str.trim().parse::<u32>() {
        Ok(n) => match n {
            1 => Some(0), // Success
            2 => Some(1), // Failure
            3 => Some(2), // Critical
            4 => Some(3), // Fumble
            5 => None,
            _ => {
                println!("無効な選択です。詳細表示をスキップします。");
                None
            }
        },
        Err(_) => {
            println!("無効な入力です。詳細表示をスキップします。");
            None
        }
    };

    // Conditional Skill Display Loop
    if let Some(index) = chosen_skill_display_index {
        if let Some(user_choice) = UserChoice::from_index(index) {
            let category_name = user_choice.to_display_string();
            println!("\n--- {}の技能詳細 ---", category_name);
            for (name, log_summary) in &log_summary_by_name {
                let skills_output = log_summary.format_chosen_skills_only(index);
                println!("{}：\n{}", name, skills_output);
            }
            println!("---------------------------");
        } else {
            // This case should ideally not be reached if parsing logic is correct
            println!("内部エラー: 無効なインデックスが処理されました。");
        }
    } else {
        println!("詳細表示をスキップします。");
    }

    println!("\nEnterキーで終了します");
    let mut a = "".to_string();
    io::stdin().read_line(&mut a).expect("Failed to read line");
}
