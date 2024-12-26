use crate::log::Log;
use error::Error;
use scraper::{Html, Selector};
use std::fs::File;
use std::io::Read;

pub mod error;
pub mod log;

pub fn get_logs(filename: &str) -> Result<Vec<Log>, Error> {
    let mut file = File::open(filename)?;
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
        logs.push(log)
    }
    Ok(logs)
}
