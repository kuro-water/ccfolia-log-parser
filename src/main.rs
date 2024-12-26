use std::fs::File;
use std::io::Read;
use unicode_segmentation::UnicodeSegmentation;

mod parse_html;

fn main() {
    let filename = "data/log.html";
    // let mut file = File::open(config.filename)?;
    let mut file = File::open(filename).expect(&format!("{} not found", filename));
    let mut html = String::new();
    file.read_to_string(&mut html)
        .expect(&format!("{} is not HTML.", filename));

    // HTMLをパース
    let document = scraper::Html::parse_document(&html);

    // セレクターをパース
    let p_selector = scraper::Selector::parse("p").unwrap();
    let span_selector = scraper::Selector::parse("span").unwrap();

    // セレクターを用いて要素を取得
    let p_tags = document.select(&p_selector);

    // 一つのpタグに一つのチャットが入っている
    for p_tag in p_tags {
        // let texts = p_tag.text().collect::<Vec<_>>();
        // for text in texts.iter() {
        //     println!("a. {}", text.trim().replace("\n", ""));
        // }
        //
        // println!("{}",texts.len());

        let mut span_tags = p_tag.select(&span_selector);

        // タブ名
        if let Some(tab) = span_tags.next() {
            let texts = tab.text().collect::<Vec<_>>();
            for text in texts.iter() {
                // 改行を削除
                let text = text.trim().replace("\n", "");

                // 書記素クラスタに分解
                let graphemes = text.graphemes(true).collect::<Vec<&str>>();
                let first = graphemes.first();
                let last = graphemes.last();
                // タブ名であれば[]で囲まれているはず
                if let (Some(&first), Some(&last)) = (first, last) {
                    if first != "[" || last != "]" {
                        panic!("{}はタブ名ではありません", text);
                    }
                }
                let new_text = graphemes[1..graphemes.len() - 1].concat();

                println!("tab:{}", new_text);
            }
        }

        // PC名
        if let Some(name) = span_tags.next() {
            let texts = name.text().collect::<Vec<_>>();
            for text in texts.iter() {
                let text = text.trim().replace("\n", "");
                println!("name:{}", text);
            }
        }

        let mut num = 1;
        for span_tag in span_tags {
            let texts = span_tag.text().collect::<Vec<_>>();
            for text in texts.iter() {
                let text = text.trim().replace("\n", "");
                println!("{}:{}", num, text);
                num += 1;
            }
        }

        println!();
    }
}
