use anyhow::{bail, Result};
use scraper::element_ref::Select;
use scraper::ElementRef;
use std::fs::File;
use std::io::Read;
use unicode_segmentation::UnicodeSegmentation;

mod parse_html;

struct Log {
    tab: String,
    name: String,
    texts: Vec<String>,
}

impl Log {
    fn get_tab(span_tag: Option<ElementRef>) -> Result<String> {
        let Some(span_tag) = span_tag else {
            // span_tags.next()がNoneの場合ここに入る
            panic!("タブを格納するspanタグが見つかりません");
        };
        let texts = span_tag.text().collect::<Vec<_>>();
        if texts.len() != 1 {
            panic!("タブ名が1行ではなく{}行あります", texts.len());
        }
        match Log::validate_tab(texts[0].to_string()) {
            Ok(tab) => Ok(tab),
            Err(e) => {
                panic!("{}タブの解析中にエラーが発生しました：{}", texts[0], e);
            }
        }
    }

    fn get_name(span_tag: Option<ElementRef>) -> Result<String> {
        let Some(name) = span_tag else {
            panic!("名前を格納するspanタグが見つかりません");
        };
        let texts = name.text().collect::<Vec<_>>();
        if texts.len() != 1 {
            panic!("名前が1行ではなく{}行あります", texts.len());
        }
        Ok(texts[0].trim().replace("\n", ""))
    }

    fn get_texts(span_tag: Option<ElementRef>) -> Result<Vec<String>> {
        let Some(texts) = span_tag else {
            panic!("テキストを格納するspanタグが見つかりません");
        };
        let texts = texts.text().collect::<Vec<_>>();
        let mut vec = Vec::new();
        for text in texts.iter() {
            let text = text.trim().replace("\n", "");
            vec.push(text);
        }
        Ok(vec)
    }

    fn new(mut span_tags: Select) -> Log {
        // タブ
        let tab = match Log::get_tab(span_tags.next()) {
            Ok(tab) => tab,
            Err(e) => panic!("{e}"),
        };

        // 名前
        let name = match Log::get_name(span_tags.next()) {
            Ok(name) => name,
            Err(e) => panic!("{e}"),
        };

        // テキスト
        let texts = match Log::get_texts(span_tags.next()) {
            Ok(texts) => texts,
            Err(e) => panic!("{e}"),
        };

        Log { tab, name, texts }
    }

    fn validate_tab(tab: String) -> Result<String> {
        // 改行を削除
        let tab = tab.trim().replace("\n", "");

        // 書記素クラスタに分解
        let graphemes = tab.graphemes(true).collect::<Vec<&str>>();

        // []で囲まれていることを確認
        let first = graphemes.first();
        let last = graphemes.last();
        let (Some(&first), Some(&last)) = (first, last) else {
            // first, lastが取得できなかった場合ここに入る
            // bail!:簡易的なエラー
            // 参考：https://zenn.dev/shimopino/articles/understand-rust-error-handling#%E7%B0%A1%E6%98%93%E7%9A%84%E3%81%AA%E3%82%A8%E3%83%A9%E3%83%BC%E3%81%AE%E5%AE%9A%E7%BE%A9
            bail!("タブ名の最初または最後の文字が取得できません");
        };
        if first != "[" || last != "]" {
            bail!("不正なタブ名です");
        }

        // []を削除
        Ok(graphemes[1..graphemes.len() - 1].concat())
    }
}

fn main() {
    let filename = "data/Log.html";
    // let mut file = File::open(config.filename)?;
    let mut file = File::open(filename).expect(&format!("{} not found", filename));
    let mut html = String::new();
    file.read_to_string(&mut html)
        .expect(&format!("{filename} is not HTML."));

    // HTMLをパース
    let document = scraper::Html::parse_document(&html);

    // セレクターをパース
    let p_selector = scraper::Selector::parse("p").unwrap();
    let span_selector = scraper::Selector::parse("span").unwrap();

    // セレクターを用いて要素を取得
    let p_tags = document.select(&p_selector);

    // 一つのpタグに一つのチャットが入っている
    for p_tag in p_tags {
        let span_tags = p_tag.select(&span_selector);

        let log = Log::new(span_tags);

        println!("tab:{}", log.tab);
        println!("name:{}", log.name);
        for (i, text) in log.texts.iter().enumerate() {
            println!("{}:{}", i, text);
        }

        println!();
    }
}
