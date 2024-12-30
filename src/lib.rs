use crate::log::Log;
use crate::log_summary::LogSummary;
use error::MyError;
use scraper::{Html, Selector};
use std::collections::{HashMap, HashSet};
use std::env::Args;
use std::fs::File;
use std::io::Read;

pub mod error;
pub mod log;
pub mod log_summary;

pub fn get_logs(mut args: Args) -> Result<Vec<Log>, MyError> {
    let default = "data/log5.html".to_string();

    // argsのチェック
    let filename = if args.len() == 2 {
        args.next();
        args.next().unwrap_or_else(|| default)
    } else {
        default
    };

    let mut file = File::open(&filename)?;
    let mut html = String::new();
    file.read_to_string(&mut html)?;

    // HTMLをパース
    let document = Html::parse_document(&html);
    // セレクターをパース
    // エラーのはずがないのでunwrapでよい
    let p_selector = Selector::parse("p").unwrap();
    let span_selector = Selector::parse("span").unwrap();

    // セレクターを用いて要素を取得
    let p_tags = document.select(&p_selector);
    let mut logs = Vec::new();

    // 一つのpタグに一つのチャットが入っている
    for p_tag in p_tags {
        let span_tags = p_tag.select(&span_selector);
        let log = Log::new(span_tags)?;

        // ---start---以前は無視する
        if log.texts.len() == 1 && log.texts[0] == "---start---" {
            logs = Vec::new();
        }

        logs.push(log)
    }
    Ok(logs)
}

pub fn get_log_summary(logs: &Vec<Log>) -> LogSummary {
    LogSummary::new(logs.iter().collect())
}

pub fn get_pc_summary(logs: &Vec<Log>) -> HashMap<String, LogSummary> {
    let names: HashSet<_> = logs.iter().map(|log| log.name.clone()).collect();
    let mut map = HashMap::new();
    for name in names {
        let logs: Vec<&Log> = logs.iter().filter(|log| log.name == name).collect();
        let log_summary = LogSummary::new(logs);

        let mut count = 0;
        count += log_summary.successes.iter().count();
        count += log_summary.failures.iter().count();
        count += log_summary.criticals.iter().count();
        count += log_summary.fumbles.iter().count();
        if count == 0 {
            continue;
        }
        map.insert(name, log_summary);
    }
    map
}
