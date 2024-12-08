
/// Defines operation if length of collection is divisible by 2 (i.e. cannot determine exact middle element)
pub enum FindMiddleElementMode {
    /// find_middle_element will return as Err
    Error,
    /// find_middle_element will return the value to the left of the middle point
    Left,
    /// find_middle_element will return the value to the right of the middle port
    Right,
}

pub fn find_middle_element<T>(collection: &Vec<T>, mode: FindMiddleElementMode) -> Result<&T, &str>
where T: std::hash::Hash + std::cmp::Eq {
    let midpoint = collection.len() / 2;
    if collection.len() % 2 == 0 {
        match mode {
            FindMiddleElementMode::Error => Err("Collection has even number of elements and error mode used"),
            FindMiddleElementMode::Left => Ok(&collection[midpoint - 1]),
            FindMiddleElementMode::Right => Ok(&collection[midpoint]),
        }
    } else {
        Ok(&collection[midpoint])
    }
}

/// FindMode defines if the next element should be found ascending or descending index
pub enum FindMode {
    Ascending,
    Descending,
}


pub fn find_next_element<T>(collection: &Vec<T>, starting_index: usize, element: &T, mode: FindMode) -> Option<usize>
where
    T: PartialEq<T>,
{
    match mode {
        FindMode::Ascending => collection[starting_index + 1..].iter().position(|x| x == element).map(|i| i + starting_index + 1),
        FindMode::Descending => (0..starting_index).rev().find(|&i| &collection[i] == element),
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_middle_element_odd_count() {
        let input = vec![1, 2, 3, 4, 5];
        let result = find_middle_element(&input, FindMiddleElementMode::Error);
        assert_eq!(*result.unwrap(), 3);
    }

    #[test]
    fn find_middle_element_even_count_error_mode() {
        let input = vec![1, 2, 3, 4];
        let result = find_middle_element(&input, FindMiddleElementMode::Error);
        assert_eq!(result.unwrap_err(), "Collection has even number of elements and error mode used");
    }

    #[test]
    fn find_middle_element_even_count_left_mode() {
        let input = vec![1, 2, 3, 4];
        let result = find_middle_element(&input, FindMiddleElementMode::Left);
        assert_eq!(*result.unwrap(), 2);
    }

    #[test]
    fn find_middle_element_even_count_right_mode() {
        let input = vec![1, 2, 3, 4];
        let result = find_middle_element(&input, FindMiddleElementMode::Right);
        assert_eq!(*result.unwrap(), 3);
    }

    #[test]
    fn find_next_element_ascending() {
        let input = vec![1, 2, 3, 4, 2, 5, 6];
        let result = find_next_element(&input, 0, &2, FindMode::Ascending);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn find_next_element_ascending_no_match() {
        let input = vec![1, 2, 3, 4, 5, 6];
        let result = find_next_element(&input, 5, &1, FindMode::Ascending);
        assert_eq!(result, None);
    }

    #[test]
    fn find_next_element_descending() {
        let input = vec![1, 2, 3, 4, 2, 5, 6];
        let result = find_next_element(&input, 4, &2, FindMode::Descending);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn find_next_element_descending_no_match() {
        let input = vec![1, 2, 3, 4, 5, 6];
        let result = find_next_element(&input, 3, &6, FindMode::Descending);
        assert_eq!(result, None);
    }

    #[test]
    fn find_next_element_ascending_start_middle() {
        let input = vec![1, 2, 3, 4, 2, 5, 6];
        let result = find_next_element(&input, 3, &2, FindMode::Ascending);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn find_next_element_descending_start_middle() {
        let input = vec![1, 2, 3, 4, 2, 5, 6];
        let result = find_next_element(&input, 4, &1, FindMode::Descending);
        assert_eq!(result, Some(0));
    }
}
