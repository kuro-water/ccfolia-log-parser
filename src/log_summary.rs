use crate::log::Log;
use std::fmt::{Display, Formatter};

pub struct LogSummary<'a> {
    pub successes: Vec<&'a Log>,
    pub failures: Vec<&'a Log>,
    pub criticals: Vec<&'a Log>,
    pub fumbles: Vec<&'a Log>,
}

impl<'a> LogSummary<'a> {
    pub fn new(logs: Vec<&'a Log>) -> LogSummary<'a> {
        let successes: Vec<_> = logs
            .iter()
            .filter(|log| {
                log.texts.iter().any(|text| {
                    (text.contains("成功") || text.contains("スペシャル"))
                        && !text.contains("決定的成功")
                })
            })
            .map(|log| log.clone())
            .collect();

        let failures: Vec<_> = logs
            .iter()
            .filter(|log| {
                log.texts
                    .iter()
                    .any(|text| text.contains("失敗") && !text.contains("致命的失敗"))
            })
            .map(|log| log.clone())
            .collect();

        let criticals: Vec<_> = logs
            .iter()
            .filter(|log| log.texts.iter().any(|text| text.contains("決定的成功")))
            .map(|log| log.clone())
            .collect();

        let fumbles: Vec<_> = logs
            .iter()
            .filter(|log| log.texts.iter().any(|text| text.contains("致命的失敗")))
            .map(|log| log.clone())
            .collect();

        LogSummary {
            successes,
            failures,
            criticals,
            fumbles,
        }
    }

    pub fn print_log(&self) -> String {
        let mut s = String::new();
        s.push_str("----- 通常成功 -----\n");
        for success in &self.successes {
            s.push_str(&format!("{}", success));
        }
        s.push_str("----- 通常失敗 -----\n");
        for failure in &self.failures {
            s.push_str(&format!("{}", failure));
        }
        s.push_str("----- クリティカル -----\n");
        for critical in &self.criticals {
            s.push_str(&format!("{}", critical));
        }
        s.push_str("----- ファンブル -----\n");
        for fumble in &self.fumbles {
            s.push_str(&format!("{}", fumble));
        }
        s
    }
}

impl Display for LogSummary<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str("----- 総計 -----\n");
        s.push_str(&format!("通常成功：{}\n", self.successes.iter().count()));
        s.push_str(&format!("通常失敗：{}\n", self.failures.iter().count()));
        s.push_str(&format!(
            "クリティカル：{}\n",
            self.criticals.iter().count()
        ));
        s.push_str(&format!("ファンブル：{}\n", self.fumbles.iter().count()));

        write!(f, "{}", s)
    }
}
