use std::collections::HashMap;

pub fn count_elements<T>(collection: &Vec<T>) -> HashMap<&T, i32> where T: std::hash::Hash + std::cmp::Eq {
    collection.into_iter().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    })
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
}
