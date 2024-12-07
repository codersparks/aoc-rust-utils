use std::collections::HashMap;

pub fn count_elements<T>(collection: &Vec<T>) -> HashMap<&T, i32> where T: std::hash::Hash + std::cmp::Eq {
    collection.into_iter().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    })
}

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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_integers() {
        let input = vec![1,1,2,1,2,3,1,2,3,4,1,2,3,4,5,6];
        let result = count_elements(&input);

        let one_count = result.get(&1).unwrap();
        assert_eq!(*one_count, 5);
        let two_count = result.get(&2).unwrap();
        assert_eq!(*two_count, 4);
        let three_count = result.get(&3).unwrap();
        assert_eq!(*three_count, 3);
        let four_count = result.get(&4).unwrap();
        assert_eq!(*four_count, 2);
        let five_count = result.get(&5).unwrap();
        assert_eq!(*five_count, 1);
        let six_count = result.get(&6).unwrap();
        assert_eq!(*six_count, 1);
    }

    #[test]
    fn count_strings() {
        let input = vec!["one", "two", "three", "two", "three", "three"];
        let result = count_elements(&input);

        let one_count = result.get(&"one").unwrap();
        assert_eq!(*one_count, 1);
        let two_count = result.get(&"two").unwrap();
        assert_eq!(*two_count, 2);
        let three_count = result.get(&"three").unwrap();
        assert_eq!(*three_count, 3);
    }


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
}
