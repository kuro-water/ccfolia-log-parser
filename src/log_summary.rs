use crate::log::Log;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

fn extract_skills_from_log_text(text: &str) -> Option<String> {
    if let Some(start_index) = text.find('【') {
        if let Some(end_index) = text.find('】') {
            if start_index < end_index {
                return Some(text[start_index + 3..end_index].to_string());
            }
        }
    }
    None
}

fn extract_skills_for_logs<'a>(logs: &[&'a Log]) -> HashMap<String, usize> {
    let mut skills_map: HashMap<String, usize> = HashMap::new();
    for log in logs {
        for text in &log.texts {
            if let Some(skill_name) = extract_skills_from_log_text(text) {
                *skills_map.entry(skill_name).or_insert(0) += 1;
            }
        }
    }
    skills_map
}

pub enum UserChoice {
    Success,
    Failure,
    Critical,
    Fumble,
}

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
            .map(|log| *log)
            .collect();

        let failures: Vec<_> = logs
            .iter()
            .filter(|log| {
                log.texts
                    .iter()
                    .any(|text| text.contains("失敗") && !text.contains("致命的失敗"))
            })
            .map(|log| *log)
            .collect();

        let criticals: Vec<_> = logs
            .iter()
            .filter(|log| log.texts.iter().any(|text| text.contains("決定的成功")))
            .map(|log| *log)
            .collect();

        let fumbles: Vec<_> = logs
            .iter()
            .filter(|log| log.texts.iter().any(|text| text.contains("致命的失敗")))
            .map(|log| *log)
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

    pub fn format_with_skills(&self, chosen_result_index: Option<usize>) -> String {
        let mut s = String::new();
        s.push_str(&format!("通常成功：{}\n", self.successes.len()));
        s.push_str(&format!("通常失敗：{}\n", self.failures.len()));
        s.push_str(&format!("クリティカル：{}\n", self.criticals.len()));
        s.push_str(&format!("ファンブル：{}\n", self.fumbles.len()));

        if let Some(index) = chosen_result_index {
            let (log_type_name, logs_to_process) = match index {
                0 => ("通常成功", &self.successes),
                1 => ("通常失敗", &self.failures),
                2 => ("クリティカル", &self.criticals),
                3 => ("ファンブル", &self.fumbles),
                _ => return s, // Invalid index, return current summary
            };

            let skills_map = extract_skills_for_logs(logs_to_process);

            if skills_map.is_empty() {
                s.push_str(&format!("{}した技能: なし\n", log_type_name));
            } else {
                let skill_list_str = skills_map
                    .iter()
                    .map(|(skill, count)| format!("{}（{}回）", skill, count))
                    .collect::<Vec<String>>()
                    .join(", ");
                s.push_str(&format!("{}した技能: {}\n", log_type_name, skill_list_str));
            }
        }
        s
    }
}

impl Display for LogSummary<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // let mut s = String::new();
        // // s.push_str("----- 総計 -----\n");
        // s.push_str(&format!("通常成功：{}\n", self.successes.iter().count()));
        // s.push_str(&format!("通常失敗：{}\n", self.failures.iter().count()));
        // s.push_str(&format!(
        //     "クリティカル：{}\n",
        //     self.criticals.iter().count()
        // ));
        // s.push_str(&format!("ファンブル：{}\n", self.fumbles.iter().count()));
        //
        // write!(f, "{}", s)
        write!(f, "{}", self.format_with_skills(None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::log::Log; // Log is used in later tests
    use std::collections::HashMap; // HashMap is used in later tests

    #[test]
    fn test_extract_skill_simple() {
        assert_eq!(
            extract_skills_from_log_text("CCB<=80 【目星】 result"),
            Some("目星".to_string())
        );
    }

    #[test]
    fn test_extract_skill_no_skill() {
        assert_eq!(extract_skills_from_log_text(" обычный текст "), None);
    }

    #[test]
    fn test_extract_skill_empty_text() {
        assert_eq!(extract_skills_from_log_text(""), None);
    }

    #[test]
    fn test_extract_skill_multiple_brackets() {
        assert_eq!(
            extract_skills_from_log_text("【交渉】 then 【心理学】"),
            Some("交渉".to_string())
        );
    }

    #[test]
    fn test_extract_skill_malformed_brackets() {
        assert_eq!(extract_skills_from_log_text("【目星 result"), None);
        assert_eq!(extract_skills_from_log_text("目星】 result"), None);
        assert_eq!(extract_skills_from_log_text("【】 result"), Some("".to_string())); // Extracts empty string
    }

    #[test]
    fn test_extract_skills_empty_list() {
        let logs: Vec<&Log> = Vec::new();
        let skills = extract_skills_for_logs(&logs);
        assert!(skills.is_empty());
    }

    #[test]
    fn test_extract_skills_no_skills_in_logs() {
        let log1 = Log {
            tab: "メイン".to_string(),
            name: "PC1".to_string(),
            texts: vec!["イオリ (いおり) : 1d10 (1D10) > 8".to_string(), "CCB<=80 (1D100<=80) > 50 > 成功".to_string()],
        };
        let logs_slice = vec![&log1];
        let skills = extract_skills_for_logs(&logs_slice);
        assert!(skills.is_empty());
    }

    #[test]
    fn test_extract_skills_single_log_single_skill() {
        let log1 = Log {
            tab: "メイン".to_string(),
            name: "PC1".to_string(),
            texts: vec!["CCB<=25 【目星】 (1D100<=25) > 10 > 成功".to_string()],
        };
        let logs_slice = vec![&log1];
        let skills = extract_skills_for_logs(&logs_slice);
        let mut expected = HashMap::new();
        expected.insert("目星".to_string(), 1);
        assert_eq!(skills, expected);
    }

    #[test]
    fn test_extract_skills_multiple_logs_counting() {
        let log1 = Log {
            tab: "メイン".to_string(),
            name: "PC1".to_string(),
            texts: vec!["CCB<=60 【攻撃】 (1D100<=60) > 30 > 成功".to_string()],
        };
        let log2 = Log {
            tab: "メイン".to_string(),
            name: "PC1".to_string(),
            texts: vec!["CCB<=50 【回避】 (1D100<=50) > 55 > 失敗".to_string(), "何か【攻撃】 (1D100<=60) > 5 > 決定的成功".to_string()],
        };
        let log3 = Log {
            tab: "メイン".to_string(),
            name: "PC2".to_string(),
            texts: vec!["CCB<=70 【応急手当】 (1D100<=70) > 20 > 成功".to_string(), "そして【攻撃】 (1D100<=60) > 98 > 致命的失敗".to_string()],
        };
        let logs_slice = vec![&log1, &log2, &log3];
        let skills = extract_skills_for_logs(&logs_slice);

        let mut expected = HashMap::new();
        expected.insert("攻撃".to_string(), 3);
        expected.insert("回避".to_string(), 1);
        expected.insert("応急手当".to_string(), 1);
        assert_eq!(skills, expected);
    }

    #[test]
    fn test_extract_skills_log_with_multiple_texts_same_skill() {
        let log1 = Log {
            tab: "メイン".to_string(),
            name: "PC1".to_string(),
            texts: vec!["【知略】を使い、さらに【知略】を重ねる".to_string()],
        };
        let logs_slice = vec![&log1];
        let skills = extract_skills_for_logs(&logs_slice);
        let mut expected = HashMap::new();
        expected.insert("知略".to_string(), 1); // Current function only extracts first skill per text
        assert_eq!(skills, expected);
    }

    // Helper to create basic LogSummary for format_with_skills tests
    fn create_test_summary<'a>(
        successes: Vec<&'a Log>,
        failures: Vec<&'a Log>,
        criticals: Vec<&'a Log>,
        fumbles: Vec<&'a Log>,
    ) -> LogSummary<'a> {
        LogSummary {
            successes,
            failures,
            criticals,
            fumbles,
        }
    }

    #[test]
    fn test_format_no_skills_chosen_or_present() {
        let summary = create_test_summary(vec![], vec![], vec![], vec![]);
        let output = summary.format_with_skills(None);
        let expected = "通常成功：0\n通常失敗：0\nクリティカル：0\nファンブル：0\n";
        assert_eq!(output, expected);

        let output_critical_chosen = summary.format_with_skills(Some(UserChoice::Critical as usize));
        let expected_critical = "通常成功：0\n通常失敗：0\nクリティカル：0\nファンブル：0\nクリティカルした技能: なし\n";
        assert_eq!(output_critical_chosen, expected_critical);
    }

    #[test]
    fn test_format_criticals_with_skills() {
        let crit_log1 = Log {
            tab: "メイン".to_string(),
            name: "PC1".to_string(),
            texts: vec!["CCB<=5 【目星】 (1D100<=5) > 3 > 決定的成功".to_string()],
        };
        let crit_log2 = Log {
            tab: "メイン".to_string(),
            name: "PC1".to_string(),
            texts: vec!["部屋の隅で【聞き耳】 (1D100<=71) > 1 > スペシャル！".to_string()], // Using スペシャル for variety
        };
         let crit_log3 = Log { // Same skill as log1
            tab: "メイン".to_string(),
            name: "PC2".to_string(),
            texts: vec!["CCB<=10 【目星】 (1D100<=10) > 1 > 決定的成功！！！！".to_string()],
        };
        // This log should not be picked up by LogSummary::new() as critical due to "成功" text, but extract_skills_for_logs doesn't care
        // For format_with_skills, the LogSummary instance is manually created, so this log will be in its criticals list.
        let summary = create_test_summary(vec![], vec![], vec![&crit_log1, &crit_log2, &crit_log3], vec![]);
        let output = summary.format_with_skills(Some(UserChoice::Critical as usize));

        let expected_base = "通常成功：0\n通常失敗：0\nクリティカル：3\nファンブル：0\n";
        // HashMap iteration order is not guaranteed, so check for parts
        assert!(output.starts_with(expected_base));
        assert!(output.contains("クリティカルした技能: "));
        assert!(output.contains("目星（2回）"));
        assert!(output.contains("聞き耳（1回）"));
        assert!(output.ends_with("\n"));
    }

    #[test]
    fn test_format_successes_with_skills_and_no_skills_chosen() {
        let success_log1 = Log {
            tab: "メイン".to_string(),
            name: "PC1".to_string(),
            texts: vec!["1d100<=48 【SAN値チェック】 (1D100<=48) > 22 > 成功".to_string()],
        };
         let success_log2 = Log { // This log would be categorized as success by LogSummary::new
            tab: "メイン".to_string(),
            name: "PC2".to_string(),
            texts: vec!["CCB<=80 (1D100<=80) > 50 > 成功".to_string()], // No skill
        };
        let summary = create_test_summary(vec![&success_log1, &success_log2], vec![], vec![], vec![]);

        // Test with "Success" chosen
        let output_success_chosen = summary.format_with_skills(Some(UserChoice::Success as usize));
        let expected_base_success = "通常成功：2\n通常失敗：0\nクリティカル：0\nファンブル：0\n";
        assert!(output_success_chosen.starts_with(expected_base_success));
        assert!(output_success_chosen.contains("通常成功した技能: SAN値チェック（1回）"));
        assert!(output_success_chosen.ends_with("\n"));

        // Test with None chosen (no specific skill list)
        let output_none_chosen = summary.format_with_skills(None);
        let expected_none = "通常成功：2\n通常失敗：0\nクリティカル：0\nファンブル：0\n";
        assert_eq!(output_none_chosen, expected_none);
    }

    #[test]
    fn test_format_fumbles_no_skills() {
        let fumble_log = Log {
            tab: "メイン".to_string(),
            name: "PC1".to_string(),
            texts: vec!["CCB<=71 【聞き耳】 (1D100<=71) > 96 > 致命的失敗".to_string()],
        };
        // LogSummary::new would categorize this as a fumble.
        // format_with_skills will extract "聞き耳" from it if fumbles are chosen.
        let summary = create_test_summary(vec![], vec![], vec![], vec![&fumble_log]);
        let output = summary.format_with_skills(Some(UserChoice::Fumble as usize));
        let expected_base_fumble = "通常成功：0\n通常失敗：0\nクリティカル：0\nファンブル：1\n";
        assert!(output.starts_with(expected_base_fumble));
        assert!(output.contains("ファンブルした技能: 聞き耳（1回）"));
        assert!(output.ends_with("\n"));
    }

     #[test]
    fn test_format_fumbles_no_skill_text() { // New test for fumble without skill
        let fumble_log_no_skill = Log {
            tab: "メイン".to_string(),
            name: "PC1".to_string(),
            texts: vec!["CCB<=80 (1D100<=80) > 100 > 致命的失敗".to_string()],
        };
        let summary = create_test_summary(vec![], vec![], vec![], vec![&fumble_log_no_skill]);
        let output = summary.format_with_skills(Some(UserChoice::Fumble as usize));
        let expected = "通常成功：0\n通常失敗：0\nクリティカル：0\nファンブル：1\nファンブルした技能: なし\n";
        assert_eq!(output, expected);
    }

     #[test]
    fn test_format_invalid_choice_index() {
        let summary = create_test_summary(vec![], vec![], vec![], vec![]);
        let output = summary.format_with_skills(Some(99)); // Invalid index
        let expected = "通常成功：0\n通常失敗：0\nクリティカル：0\nファンブル：0\n";
        assert_eq!(output, expected);
    }
}
