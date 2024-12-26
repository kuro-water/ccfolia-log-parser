use scraper::{Html, Selector};
use std::fs::File;
use std::io::Read;

mod log;
use log::Log;

fn main() {
    let filename = "data/Log.html";
    // let mut file = File::open(config.filename)?;
    let mut file = File::open(filename).expect(&format!("{} not found", filename));
    let mut html = String::new();
    file.read_to_string(&mut html)
        .expect(&format!("{} is not HTML.", filename));

    // HTMLをパース
    let document = Html::parse_document(&html);

    // セレクターをパース
    let p_selector = Selector::parse("p").unwrap();
    let span_selector = Selector::parse("span").unwrap();

    // セレクターを用いて要素を取得
    let p_tags = document.select(&p_selector);

    let mut logs = Vec::new();

    // 一つのpタグに一つのチャットが入っている
    for p_tag in p_tags {
        let span_tags = p_tag.select(&span_selector);

        let log = match Log::new(span_tags) {
            Ok(log) => log,
            Err(e) => panic!("{}", e),
        };
        println!("{}", log);

        logs.push(log)
    }
}
