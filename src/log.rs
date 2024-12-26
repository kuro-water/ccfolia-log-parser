use anyhow::bail;
use scraper::element_ref::Select;
use scraper::ElementRef;
use std::fmt;
use unicode_segmentation::UnicodeSegmentation;

pub struct Log {
    pub tab: String,
    pub name: String,
    pub texts: Vec<String>,
}

impl Log {
    pub fn new(mut span_tags: Select) -> anyhow::Result<Log> {
        // タブ
        let tab = match Log::get_tab(span_tags.next()) {
            Ok(tab) => tab,
            Err(e) => bail!("{}", e),
        };

        // 名前
        let name = match Log::get_name(span_tags.next()) {
            Ok(name) => name,
            Err(e) => bail!("{}", e),
        };

        // テキスト
        let texts = match Log::get_texts(span_tags.next()) {
            Ok(texts) => texts,
            Err(e) => bail!("{}", e),
        };

        Ok(Log { tab, name, texts })
    }

    fn validate_tab(tab: String) -> anyhow::Result<String> {
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

    fn get_tab(span_tag: Option<ElementRef>) -> anyhow::Result<String> {
        let Some(span_tag) = span_tag else {
            // span_tags.next()がNoneの場合ここに入る
            bail!("タブを格納するspanタグが見つかりません");
        };
        let texts = span_tag.text().collect::<Vec<_>>();
        if texts.len() != 1 {
            bail!("タブ名が1行ではなく{}行あります", texts.len());
        }
        match Log::validate_tab(texts[0].to_string()) {
            Ok(tab) => Ok(tab),
            Err(e) => {
                bail!("{}タブの解析中にエラーが発生しました：{}", texts[0], e);
            }
        }
    }

    fn get_name(span_tag: Option<ElementRef>) -> anyhow::Result<String> {
        let Some(name) = span_tag else {
            bail!("名前を格納するspanタグが見つかりません");
        };
        let texts = name.text().collect::<Vec<_>>();
        if texts.len() != 1 {
            bail!("名前が1行ではなく{}行あります", texts.len());
        }
        Ok(texts[0].trim().replace("\n", ""))
    }

    fn get_texts(span_tag: Option<ElementRef>) -> anyhow::Result<Vec<String>> {
        let Some(texts) = span_tag else {
            bail!("テキストを格納するspanタグが見つかりません");
        };
        let texts = texts.text().collect::<Vec<_>>();
        let mut vec = Vec::new();
        for text in texts.iter() {
            let text = text.trim().replace("\n", "");
            vec.push(text);
        }
        Ok(vec)
    }
}

impl fmt::Display for Log {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        s.push_str(&format!("tab:{}\n", self.tab));
        s.push_str(&format!("name:{}\n", self.name));
        for (i, text) in self.texts.iter().enumerate() {
            s.push_str(&format!("{}:{}\n", i, text));
        }

        write!(f, "{s}")
    }
}
