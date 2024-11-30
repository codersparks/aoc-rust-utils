use regex::Regex;

pub trait LineProcessor {
    type Item;
    fn process(&self, line: &str) -> Self::Item;
}

pub enum RegexLineProcessorMode {
    Split(bool),
    Matches,
    FirstLast
}

pub struct RegexLineProcessor {
    regex: Regex,
    mode: RegexLineProcessorMode
}
impl RegexLineProcessor {
    pub fn new(regex: &str, mode: RegexLineProcessorMode) -> Self {
        Self {
            regex: Regex::new(regex).expect("Invalid regex"),
            mode
        }
    }

    pub fn update_mode(&mut self, mode: RegexLineProcessorMode) {
        self.mode = mode;
    }

    fn split_on_regex(&self, line: &str, strip_empty: bool) -> Vec<String> {
        let initial_split: Vec<String> = self.regex.split(line).map(|s| s.to_string()).collect();
        if strip_empty {
            initial_split.into_iter().filter(|s| !s.is_empty()).collect()
        } else {
            initial_split
        }
    }

    fn regex_matches(&self, line: &str) -> Vec<String> {
        let matches = self.regex.find_iter(line);
        let mut result = Vec::new();
        for match_result in matches {
            result.push(match_result.as_str().to_string());
        }
        result
    }

    fn regex_first_last(&self, line: &str) -> Vec<String> {
        let regex_matches = self.regex_matches(line);

        if regex_matches.len() == 0 {
            Vec::new()
        } else if regex_matches.len() == 1 {
            return vec![regex_matches[0].as_str().to_string(), regex_matches[0].as_str().to_string()]
        } else {
            return vec![regex_matches[0].as_str().to_string(), regex_matches[regex_matches.len() - 1].as_str().to_string()]
        }
    }
}
impl LineProcessor for RegexLineProcessor {
    type Item = Vec<String>;

    fn process(&self, line: &str) -> Self::Item {
        match  self.mode{
            RegexLineProcessorMode::Split(strip_empty) => self.split_on_regex(line, strip_empty),
            RegexLineProcessorMode::Matches => self.regex_matches(line),
            RegexLineProcessorMode::FirstLast => self.regex_first_last(line)
        }

    }
}

pub fn read_lines(filename: &str) -> Vec<String> {
    std::fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|l| l.to_string())
        .collect()
}

pub fn apply_processor_to_file_lines<P>(filename: &str, processor: &P) -> Vec<P::Item>
    where P: LineProcessor
{
    std::fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines().into_iter()
        .map(|l| processor.process(&l))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_to_vec_string() {
        let lines = read_lines("resources/aoc23_1.test");
        assert_eq!(lines.len(), 5);

        assert_eq!(lines[0], "1abc2");
        assert_eq!(lines[1], "pqr3stu8vwx");
        assert_eq!(lines[2], "a1b2c3d4e5f");
        assert_eq!(lines[3], "treb7uchet");
        assert_eq!(lines[4], "this33is8898a1test78");
    }

    #[test]
    fn test_apply_fn_to_lines() {
        let line_processor = RepeatingLineProcessor {};
        let lines = apply_processor_to_file_lines("resources/aoc23_1.test", &line_processor);
        assert_eq!(lines.len(), 5);

        assert_eq!(lines[0], "1abc2");
        assert_eq!(lines[1], "pqr3stu8vwx");
        assert_eq!(lines[2], "a1b2c3d4e5f");
        assert_eq!(lines[3], "treb7uchet");
        assert_eq!(lines[4], "this33is8898a1test78");
    }

    #[test]
    fn test_split_on_regex_striped() {
        let mut processor = RegexLineProcessor::new(r"(\d+)", RegexLineProcessorMode::Split(true));

        let lines = apply_processor_to_file_lines("resources/aoc23_1.test", &processor);

        assert_eq!(lines.len(), 5);
        assert_eq!(lines[0], vec!["abc"]);
        assert_eq!(lines[1], vec!["pqr", "stu", "vwx"]);
        assert_eq!(lines[2], vec!["a", "b", "c", "d", "e", "f"]);
        assert_eq!(lines[3], vec!["treb", "uchet"]);
        assert_eq!(lines[4], vec!["this", "is", "a", "test"]);

        // Now we test without stripping the empty fields
        processor.update_mode(RegexLineProcessorMode::Split(false));

        let lines2 = apply_processor_to_file_lines("resources/aoc23_1.test", &processor);

        assert_eq!(lines2.len(), 5);
        assert_eq!(lines2[0], vec!["", "abc", ""]);
        assert_eq!(lines2[1], vec!["pqr", "stu", "vwx"]);
        assert_eq!(lines2[2], vec!["a", "b", "c", "d", "e", "f"]);
        assert_eq!(lines2[3], vec!["treb", "uchet"]);
        assert_eq!(lines2[4], vec!["this", "is", "a", "test", ""]);
    }

    #[test]
    fn test_regex_matches() {
        let processor = RegexLineProcessor::new(r"(\d+)", RegexLineProcessorMode::Matches);

        let lines = apply_processor_to_file_lines("resources/aoc23_1.test", &processor);

        assert_eq!(lines.len(), 5);
        assert_eq!(lines[0], vec!["1", "2"]);
        assert_eq!(lines[1], vec!["3", "8"]);
        assert_eq!(lines[2], vec!["1","2", "3", "4", "5"]);
        assert_eq!(lines[3], vec!["7"]);
        assert_eq!(lines[4], vec!["33", "8898", "1", "78"]);
    }

    #[test]
    fn test_regex_first_last() {
        let processor = RegexLineProcessor::new(r"(\d+)", RegexLineProcessorMode::FirstLast);

        let lines = apply_processor_to_file_lines("resources/aoc23_1.test", &processor);

        assert_eq!(lines.len(), 5);
        assert_eq!(lines[0], vec!["1", "2"]);
        assert_eq!(lines[1], vec!["3", "8"]);
        assert_eq!(lines[2], vec!["1", "5"]);
        assert_eq!(lines[3], vec!["7", "7"]);
        assert_eq!(lines[4], vec!["33", "78"]);
    }

    struct RepeatingLineProcessor {}

    impl LineProcessor for RepeatingLineProcessor {
        type Item = String;

        fn process(&self, line: &str) -> Self::Item {
            line.to_string()
        }
    }
}
