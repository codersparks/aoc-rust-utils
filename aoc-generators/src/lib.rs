use crate::processors::line_processor_trait::LineProcessor;

pub mod processors;

pub fn apply_processor_to_input<P>(
    input: &str,
    processor: &P,
) -> Result<Vec<P::Item>, P::ProcessorError>
where
    P: LineProcessor,
{


    let mut result = Vec::new();

        for line in input.lines() {
            let line_result = processor.process(line);
            match line_result {
                Ok(l) => result.push(l),
                Err(p) => return Err(p),
            }
        }
        Ok(result)

}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::processors::regex_line_processor::{RegexLineProcessor, RegexLineProcessorMode};
    use super::*;



    #[test]
    fn test_apply_fn_to_lines() {
        let line_processor = RepeatingLineProcessor {};

        let input = fs::read_to_string("resources/aoc23_1.test").unwrap();
        if let Ok(lines) = apply_processor_to_input(&input, &line_processor) {
            assert_eq!(lines.len(), 5);

            assert_eq!(lines[0], "1abc2");
            assert_eq!(lines[1], "pqr3stu8vwx");
            assert_eq!(lines[2], "a1b2c3d4e5f");
            assert_eq!(lines[3], "treb7uchet");
            assert_eq!(lines[4], "this33is8898a1test78");
        } else { panic!("Failed to read file"); }
    }

    #[test]
    fn test_split_on_regex_striped() {
        let mut processor = RegexLineProcessor::new(r"(\d+)", RegexLineProcessorMode::Split(true));

        let input = fs::read_to_string("resources/aoc23_1.test").unwrap();
        if let Ok(lines) = apply_processor_to_input(&input, &processor) {
            assert_eq!(lines.len(), 5);
            assert_eq!(lines[0], vec!["abc"]);
            assert_eq!(lines[1], vec!["pqr", "stu", "vwx"]);
            assert_eq!(lines[2], vec!["a", "b", "c", "d", "e", "f"]);
            assert_eq!(lines[3], vec!["treb", "uchet"]);
            assert_eq!(lines[4], vec!["this", "is", "a", "test"]);
        } else {
            panic!("Failed to read file")
        }

        // Now we test without stripping the empty fields
        processor.update_mode(RegexLineProcessorMode::Split(false));

        if let Ok(lines2) = apply_processor_to_input(&input, &processor) {
            assert_eq!(lines2.len(), 5);
            assert_eq!(lines2[0], vec!["", "abc", ""]);
            assert_eq!(lines2[1], vec!["pqr", "stu", "vwx"]);
            assert_eq!(lines2[2], vec!["a", "b", "c", "d", "e", "f"]);
            assert_eq!(lines2[3], vec!["treb", "uchet"]);
            assert_eq!(lines2[4], vec!["this", "is", "a", "test", ""]);
        } else {
            panic!("Failed to read file");
        }
    }

    #[test]
    fn test_regex_matches() {
        let processor = RegexLineProcessor::new(r"(\d+)", RegexLineProcessorMode::Matches);

        let input = fs::read_to_string("resources/aoc23_1.test").unwrap();
        if let Ok(lines) = apply_processor_to_input(&input, &processor) {
            assert_eq!(lines.len(), 5);
            assert_eq!(lines[0], vec!["1", "2"]);
            assert_eq!(lines[1], vec!["3", "8"]);
            assert_eq!(lines[2], vec!["1", "2", "3", "4", "5"]);
            assert_eq!(lines[3], vec!["7"]);
            assert_eq!(lines[4], vec!["33", "8898", "1", "78"]);
        } else {
            panic!("Failed to read file");
        }
    }

    #[test]
    fn test_regex_first_last() {
        let processor = RegexLineProcessor::new(r"(\d+)", RegexLineProcessorMode::FirstLast);

        let input = fs::read_to_string("resources/aoc23_1.test").unwrap();
        if let Ok(lines) = apply_processor_to_input(&input, &processor) {
            assert_eq!(lines.len(), 5);
            assert_eq!(lines[0], vec!["1", "2"]);
            assert_eq!(lines[1], vec!["3", "8"]);
            assert_eq!(lines[2], vec!["1", "5"]);
            assert_eq!(lines[3], vec!["7", "7"]);
            assert_eq!(lines[4], vec!["33", "78"]);
        } else {
            panic!("Failed to read file");
        }
    }

    struct RepeatingLineProcessor {}

    impl LineProcessor for RepeatingLineProcessor {
        type Item = String;
        type ProcessorError = String;

        fn process(&self, line: &str) -> Result<Self::Item, Self::ProcessorError> {
            Ok(line.to_string())
        }
    }
}
