use crate::log::Log;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

fn extract_skills_from_log_text(text: &str) -> Option<String> {
    match text.find('＞') { // Using full-width greater than sign
        Some(index) => {
            Some(text[..index].trim_end().to_string())
        }
        None => None,
    }
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

impl UserChoice {
    pub fn to_display_string(&self) -> &str {
        match self {
            UserChoice::Success => "成功",
            UserChoice::Failure => "失敗",
            UserChoice::Critical => "クリティカル",
            UserChoice::Fumble => "ファンブル",
        }
    }

    pub fn from_index(index: usize) -> Option<UserChoice> {
        match index {
            0 => Some(UserChoice::Success),
            1 => Some(UserChoice::Failure),
            2 => Some(UserChoice::Critical),
            3 => Some(UserChoice::Fumble),
            _ => None,
        }
    }
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

    pub fn format_chosen_skills_only(&self, chosen_result_index: usize) -> String {
        let user_choice = match UserChoice::from_index(chosen_result_index) {
            Some(choice) => choice,
            None => return String::new(), // Or some error string / specific handling
        };

        let logs_to_process = match user_choice {
            UserChoice::Success => &self.successes,
            UserChoice::Failure => &self.failures,
            UserChoice::Critical => &self.criticals,
            UserChoice::Fumble => &self.fumbles,
        };

        let skills_map = extract_skills_for_logs(logs_to_process);
        let result_type_display_string = user_choice.to_display_string();

        if skills_map.is_empty() {
            format!("  {}した技能: なし", result_type_display_string)
        } else {
            let mut sorted_skills: Vec<String> = skills_map
                .iter()
                .map(|(skill, count)| format!("《{}》（{}回）", skill, count))
                .collect();
            sorted_skills.sort(); // Sort for consistent output order
            let joined_skill_list_str = sorted_skills.join(", ");
            format!(
                "  {}した技能: {}",
                result_type_display_string, joined_skill_list_str
            )
        }
    }
}

impl Display for LogSummary<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format_with_skills(None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::log::Log; // Log is used in later tests
    use std::collections::HashMap; // HashMap is used in later tests

    #[test]
    fn test_extract_prefix_before_gt_simple() {
        assert_eq!(
            extract_skills_from_log_text("CCB<=51 【拳】 (1D100<=51) ＞ 85 ＞ 失敗"),
            Some("CCB<=51 【拳】 (1D100<=51)".to_string())
        );
    }

    #[test]
    fn test_extract_prefix_no_gt() {
        assert_eq!(extract_skills_from_log_text("普通のテキスト"), None);
        assert_eq!(extract_skills_from_log_text("CCB<=51 【拳】 (1D100<=51) "), None);
    }

    #[test]
    fn test_extract_prefix_empty_text() {
        assert_eq!(extract_skills_from_log_text(""), None);
    }

    #[test]
    fn test_extract_prefix_gt_at_start() {
        assert_eq!(extract_skills_from_log_text("＞ 後半のみ"), Some("".to_string()));
    }

    #[test]
    fn test_extract_prefix_with_trailing_whitespace() {
        assert_eq!(
            extract_skills_from_log_text("プレフィックスのみ   ＞ 後半"),
            Some("プレフィックスのみ".to_string())
        );
    }

    #[test]
    fn test_extract_prefix_multiple_gt() {
        assert_eq!(
            extract_skills_from_log_text("最初の部分 ＞ 2番目の部分 ＞ 最後の部分"),
            Some("最初の部分".to_string())
        );
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
            // These texts do not contain '＞'
            texts: vec!["イオリ (いおり) : 1d10 (1D10) = 8".to_string(), "CCB<=80 (1D100<=80) = 50 = 成功".to_string()],
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
            texts: vec!["CCB<=25 【目星】 (1D100<=25) ＞ 10 ＞ 成功".to_string()],
        };
        let logs_slice = vec![&log1];
        let skills = extract_skills_for_logs(&logs_slice);
        let mut expected = HashMap::new();
        expected.insert("CCB<=25 【目星】 (1D100<=25)".to_string(), 1);
        assert_eq!(skills, expected);
    }

    #[test]
    fn test_extract_skills_multiple_logs_counting() {
        let log1 = Log {
            tab: "メイン".to_string(),
            name: "PC1".to_string(),
            texts: vec!["CCB<=60 【攻撃】 (1D100<=60) ＞ 30 ＞ 成功".to_string()], // Skill: "CCB<=60 【攻撃】 (1D100<=60)"
        };
        let log2 = Log {
            tab: "メイン".to_string(),
            name: "PC1".to_string(),
            texts: vec![
                "CCB<=50 【回避】 (1D100<=50) ＞ 55 ＞ 失敗".to_string(), // Skill: "CCB<=50 【回避】 (1D100<=50)"
                "何か【攻撃】 (1D100<=60) ＞ 5 ＞ 決定的成功".to_string()  // Skill: "何か【攻撃】 (1D100<=60)"
            ],
        };
        let log3 = Log {
            tab: "メイン".to_string(),
            name: "PC2".to_string(),
            texts: vec![
                "CCB<=70 【応急手当】 (1D100<=70) ＞ 20 ＞ 成功".to_string(), // Skill: "CCB<=70 【応急手当】 (1D100<=70)"
                "そして【攻撃】 (1D100<=60) ＞ 98 ＞ 致命的失敗".to_string() // Skill: "そして【攻撃】 (1D100<=60)"
            ],
        };
        let logs_slice = vec![&log1, &log2, &log3];
        let skills = extract_skills_for_logs(&logs_slice);

        let mut expected = HashMap::new();
        expected.insert("CCB<=60 【攻撃】 (1D100<=60)".to_string(), 1);
        expected.insert("CCB<=50 【回避】 (1D100<=50)".to_string(), 1);
        expected.insert("何か【攻撃】 (1D100<=60)".to_string(), 1);
        expected.insert("CCB<=70 【応急手当】 (1D100<=70)".to_string(), 1);
        expected.insert("そして【攻撃】 (1D100<=60)".to_string(), 1);
        assert_eq!(skills, expected);
    }

    #[test]
    fn test_extract_skills_log_with_multiple_texts_same_skill() {
        // This test's name is now a bit misleading as skills are full dice strings.
        // If two dice strings are identical, they will be counted together.
        let log1 = Log {
            tab: "メイン".to_string(),
            name: "PC1".to_string(),
            texts: vec![
                "【知略】(1d100<=60) ＞ 30 ＞ 成功".to_string(),
                "【知略】(1d100<=60) ＞ 40 ＞ 成功".to_string()
            ],
        };
        let logs_slice = vec![&log1];
        let skills = extract_skills_for_logs(&logs_slice);
        let mut expected = HashMap::new();
        expected.insert("【知略】(1d100<=60)".to_string(), 2);
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
            texts: vec!["CCB<=5 【目星】 (1D100<=5) ＞ 3 ＞ 決定的成功".to_string()], // Skill: "CCB<=5 【目星】 (1D100<=5)"
        };
        let crit_log2 = Log {
            tab: "メイン".to_string(),
            name: "PC1".to_string(),
            texts: vec!["部屋の隅で【聞き耳】 (1D100<=71) ＞ 1 ＞ スペシャル！".to_string()], // Skill: "部屋の隅で【聞き耳】 (1D100<=71)"
        };
         let crit_log3 = Log { // Same skill as log1, but different dice string
            tab: "メイン".to_string(),
            name: "PC2".to_string(),
            texts: vec!["CCB<=10 【目星】 (1D100<=10) ＞ 1 ＞ 決定的成功！！！！".to_string()], // Skill: "CCB<=10 【目星】 (1D100<=10)"
        };
        let summary = create_test_summary(vec![], vec![], vec![&crit_log1, &crit_log2, &crit_log3], vec![]);
        let output = summary.format_with_skills(Some(UserChoice::Critical as usize));

        let expected_base = "通常成功：0\n通常失敗：0\nクリティカル：3\nファンブル：0\n";
        assert!(output.starts_with(expected_base));
        assert!(output.contains("クリティカルした技能: "));
        // Order can vary, check for individual skill strings
        assert!(output.contains("CCB<=5 【目星】 (1D100<=5)（1回）"));
        assert!(output.contains("部屋の隅で【聞き耳】 (1D100<=71)（1回）"));
        assert!(output.contains("CCB<=10 【目星】 (1D100<=10)（1回）"));
        assert!(output.ends_with("\n"));
    }

    #[test]
    fn test_format_successes_with_skills_and_no_skills_chosen() {
        let success_log1 = Log {
            tab: "メイン".to_string(),
            name: "PC1".to_string(),
            texts: vec!["1d100<=48 【SAN値チェック】 (1D100<=48) ＞ 22 ＞ 成功".to_string()], // Skill: "1d100<=48 【SAN値チェック】 (1D100<=48)"
        };
         let success_log2 = Log {
            tab: "メイン".to_string(),
            name: "PC2".to_string(),
            texts: vec!["CCB<=80 (1D100<=80) ＞ 50 ＞ 成功".to_string()], // Skill: "CCB<=80 (1D100<=80)"
        };
        let summary = create_test_summary(vec![&success_log1, &success_log2], vec![], vec![], vec![]);

        // Test with "Success" chosen
        let output_success_chosen = summary.format_with_skills(Some(UserChoice::Success as usize));
        let expected_base_success = "通常成功：2\n通常失敗：0\nクリティカル：0\nファンブル：0\n";
        assert!(output_success_chosen.starts_with(expected_base_success));
        assert!(output_success_chosen.contains("通常成功した技能: "));
        assert!(output_success_chosen.contains("1d100<=48 【SAN値チェック】 (1D100<=48)（1回）"));
        assert!(output_success_chosen.contains("CCB<=80 (1D100<=80)（1回）"));
        assert!(output_success_chosen.ends_with("\n"));

        // Test with None chosen (no specific skill list)
        let output_none_chosen = summary.format_with_skills(None);
        let expected_none = "通常成功：2\n通常失敗：0\nクリティカル：0\nファンブル：0\n";
        assert_eq!(output_none_chosen, expected_none);
    }

    #[test]
    fn test_format_fumbles_no_skills() { // Renaming to reflect it *can* have skills now
        let fumble_log = Log {
            tab: "メイン".to_string(),
            name: "PC1".to_string(),
            texts: vec!["CCB<=71 【聞き耳】 (1D100<=71) ＞ 96 ＞ 致命的失敗".to_string()], // Skill: "CCB<=71 【聞き耳】 (1D100<=71)"
        };
        let summary = create_test_summary(vec![], vec![], vec![], vec![&fumble_log]);
        let output = summary.format_with_skills(Some(UserChoice::Fumble as usize));
        let expected_base_fumble = "通常成功：0\n通常失敗：0\nクリティカル：0\nファンブル：1\n";
        assert!(output.starts_with(expected_base_fumble));
        assert!(output.contains("ファンブルした技能: CCB<=71 【聞き耳】 (1D100<=71)（1回）"));
        assert!(output.ends_with("\n"));
    }

     #[test]
    fn test_format_fumbles_log_without_gt() { // Previously "no_skill_text", now clarifies no GT means no skill extracted
        let fumble_log_no_skill = Log {
            tab: "メイン".to_string(),
            name: "PC1".to_string(),
            texts: vec!["CCB<=80 (1D100<=80) = 100 = 致命的失敗".to_string()], // No '＞'
        };
        let summary = create_test_summary(vec![], vec![], vec![], vec![&fumble_log_no_skill]);
        let output = summary.format_with_skills(Some(UserChoice::Fumble as usize));
        let expected = "通常成功：0\n通常失敗：0\nクリティカル：0\nファンブル：1\nファンブルした技能: なし\n"; // No skill extracted
        assert_eq!(output, expected);
    }

     #[test]
    fn test_format_invalid_choice_index() {
        let summary = create_test_summary(vec![], vec![], vec![], vec![]);
        let output = summary.format_with_skills(Some(99)); // Invalid index
        let expected = "通常成功：0\n通常失敗：0\nクリティカル：0\nファンブル：0\n";
        assert_eq!(output, expected);
    }

    // Tests for format_chosen_skills_only
    #[test]
    fn test_format_chosen_skills_criticals_present() {
        let crit_log1 = Log {
            tab: "メイン".to_string(),
            name: "PC1".to_string(),
            texts: vec!["CCB<=25 【目星】 (1D100<=25) ＞ 1 ＞ 決定的成功".to_string()],
        };
        let crit_log2 = Log {
            tab: "メイン".to_string(),
            name: "PC1".to_string(),
            texts: vec!["CCB<=5 【ブラフ】 (1D100<=5) ＞ 1 ＞ スペシャル".to_string()],
        };
        let summary = create_test_summary(vec![], vec![], vec![&crit_log1, &crit_log2], vec![]);
        let output = summary.format_chosen_skills_only(UserChoice::Critical as usize);
        // Skills are sorted: "CCB<=25 【目星】 (1D100<=25)" and "CCB<=5 【ブラフ】 (1D100<=5)"
        // "CCB<=25..." comes before "CCB<=5..." alphabetically
        let expected = "  クリティカルした技能: 《CCB<=25 【目星】 (1D100<=25)》（1回）, 《CCB<=5 【ブラフ】 (1D100<=5)》（1回）";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_format_chosen_skills_successes_none_extractable() {
        let success_log_no_gt = Log {
            tab: "メイン".to_string(),
            name: "PC1".to_string(),
            texts: vec!["CCB<=80 (1D100<=80) = 50 = 成功".to_string()], // No '＞'
        };
        let summary = create_test_summary(vec![&success_log_no_gt], vec![], vec![], vec![]);
        let output = summary.format_chosen_skills_only(UserChoice::Success as usize);
        assert_eq!(output, "  成功した技能: なし");
    }

    #[test]
    fn test_format_chosen_skills_fumbles_with_skills_sorted() {
        let fumble1 = Log {
            tab: "メイン".to_string(),
            name: "PC1".to_string(),
            texts: vec!["CCB<=71 【応急手当】 (1D100<=71) ＞ 96 ＞ 致命的失敗".to_string()],
        };
        let fumble2 = Log {
            tab: "メイン".to_string(),
            name: "PC1".to_string(),
            texts: vec!["CCB<=50 【回避】 (1D100<=50) ＞ 100 ＞ 致命的失敗".to_string()],
        };
         let fumble3 = Log { // Duplicate of fumble2's skill text to test count
            tab: "メイン".to_string(),
            name: "PC2".to_string(),
            texts: vec!["CCB<=50 【回避】 (1D100<=50) ＞ 99 ＞ 致命的失敗".to_string()],
        };
        let summary = create_test_summary(vec![], vec![], vec![], vec![&fumble1, &fumble2, &fumble3]);
        let output = summary.format_chosen_skills_only(UserChoice::Fumble as usize);
        // Expected sorted: 《CCB<=50 【回避】 (1D100<=50)》（2回）, 《CCB<=71 【応急手当】 (1D100<=71)》（1回）
        let expected = "  ファンブルした技能: 《CCB<=50 【回避】 (1D100<=50)》（2回）, 《CCB<=71 【応急手当】 (1D100<=71)》（1回）";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_format_chosen_skills_empty_category() {
        let summary = create_test_summary(vec![], vec![], vec![], vec![]); // No failure logs
        let output = summary.format_chosen_skills_only(UserChoice::Failure as usize);
        assert_eq!(output, "  失敗した技能: なし");
    }

    #[test]
    fn test_format_chosen_skills_invalid_index() {
        let summary = create_test_summary(vec![], vec![], vec![], vec![]);
        let output = summary.format_chosen_skills_only(99); // Invalid index
        assert_eq!(output, ""); // Expects empty string as per current implementation
    }
}
