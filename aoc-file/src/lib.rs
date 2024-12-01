use crate::processors::line_processor_trait::LineProcessor;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
pub mod processors;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn apply_processor_to_file_lines<P>(
    filename: &str,
    processor: &P,
) -> Result<Vec<P::Item>, P::ProcessorError>
where
    P: LineProcessor,
{
    let file_lines = read_lines(filename);

    let mut result = Vec::new();
    if let Ok(lines) = file_lines {
        for line in lines {
            let line_result = processor.process(line.unwrap().as_str());
            match line_result {
                Ok(l) => result.push(l),
                Err(p) => return Err(p),
            }
        }
        Ok(result)
    } else {
        Err(file_lines.err().unwrap().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::processors::regex_line_processor::{RegexLineProcessor, RegexLineProcessorMode};

    #[test]
    fn test_read_file_to_vec_string() {
        if let Ok(buffered_lines) = read_lines("resources/aoc23_1.test") {
            let lines: Vec<String> = buffered_lines.map(|l| l.unwrap()).collect();
            assert_eq!(lines.len(), 5);

            assert_eq!(lines[0], "1abc2");
            assert_eq!(lines[1], "pqr3stu8vwx");
            assert_eq!(lines[2], "a1b2c3d4e5f");
            assert_eq!(lines[3], "treb7uchet");
            assert_eq!(lines[4], "this33is8898a1test78");
        }
    }

    #[test]
    fn test_apply_fn_to_lines() {
        let line_processor = RepeatingLineProcessor {};
        if let Ok(lines) = apply_processor_to_file_lines("resources/aoc23_1.test", &line_processor)
        {
            assert_eq!(lines.len(), 5);

            assert_eq!(lines[0], "1abc2");
            assert_eq!(lines[1], "pqr3stu8vwx");
            assert_eq!(lines[2], "a1b2c3d4e5f");
            assert_eq!(lines[3], "treb7uchet");
            assert_eq!(lines[4], "this33is8898a1test78");
        } else {
            panic!("Failed to read file");
        }
    }

    #[test]
    fn test_split_on_regex_striped() {
        let mut processor = RegexLineProcessor::new(r"(\d+)", RegexLineProcessorMode::Split(true));

        if let Ok(lines) = apply_processor_to_file_lines("resources/aoc23_1.test", &processor) {
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

        if let Ok(lines2) = apply_processor_to_file_lines("resources/aoc23_1.test", &processor) {
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

        if let Ok(lines) = apply_processor_to_file_lines("resources/aoc23_1.test", &processor) {
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

        if let Ok(lines) = apply_processor_to_file_lines("resources/aoc23_1.test", &processor) {
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
