use ndarray::{Array, Array2};

pub fn generate_2d_board_char(input: &str) -> Array2<char>{
    let input = input.trim();
    let row_count = input.lines().count();
    let chars = input.lines().map(|l| l.chars()).flatten().collect::<Vec<_>>();
    let row_length = (chars.len()) / row_count;

    let a = Array::from_iter(chars);

    let board = a.into_shape_with_order((row_count, row_length)).unwrap();
    board

}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_generate_2d_board_char() {
        let input = fs::read_to_string("resources/aoc24_4_test.txt").unwrap();

        let board = generate_2d_board_char(&input);
        println!("Board: {:#?}", board);
    }
}