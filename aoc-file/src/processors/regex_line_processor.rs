use regex::Regex;
use crate::processors::line_processor_trait::LineProcessor;

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

    fn split_on_regex(&self, line: &str, strip_empty: bool) -> Result<Vec<String>,String> {
        let split_strings: Vec<String> = self.regex.split(line).map(|s| s.to_string()).collect();
        if strip_empty {
            Ok(split_strings.into_iter().filter(|s| !s.is_empty()).collect())
        } else {
            Ok(split_strings)
        }
    }

    fn regex_matches(&self, line: &str) -> Result<Vec<String>,String> {
        let matches = self.regex.find_iter(line);
        let mut result = Vec::new();
        for match_result in matches {
            result.push(match_result.as_str().to_string());
        }
        Ok(result)
    }

    fn regex_first_last(&self, line: &str) -> Result<Vec<String>,String> {
        let regex_matches = self.regex_matches(line).unwrap();

        if regex_matches.len() == 0 {
            Ok(Vec::new())
        } else if regex_matches.len() == 1 {
            return Ok(vec![regex_matches[0].as_str().to_string(), regex_matches[0].as_str().to_string()])
        } else {
            return Ok(vec![regex_matches[0].as_str().to_string(), regex_matches[regex_matches.len() - 1].as_str().to_string()])
        }
    }
}
impl LineProcessor for RegexLineProcessor {
    type Item = Vec<String>;
    type ProcessorError = String;
    fn process(&self, line: &str) -> Result<Self::Item, Self::ProcessorError> {
        match  self.mode{
            RegexLineProcessorMode::Split(strip_empty) => self.split_on_regex(line, strip_empty),
            RegexLineProcessorMode::Matches => self.regex_matches(line),
            RegexLineProcessorMode::FirstLast => self.regex_first_last(line)
        }

    }
}