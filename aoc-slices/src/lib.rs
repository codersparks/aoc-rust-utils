
trait CountSlice {
    type Item : PartialEq;
    fn count_slice (self: &'_ Self, slice: &'_ [Self::Item]) -> u32;
}

impl<Item : PartialEq> CountSlice for [Item] {
    type Item = Item;

    fn count_slice (self: &'_ [Item], slice: &'_ [Item]) -> u32
    {
        let len = slice.len();

        self.windows(len).into_iter().filter(move | sub_slice| sub_slice == &slice).count() as u32

    }
}


pub fn count_sub_slice_u8(slice: &[u8], sub_slice: &[u8]) -> u32 {
    slice.count_slice(sub_slice)
}

pub fn count_sub_slice_ref_u8(slice: &[&u8], sub_slice: &[&u8]) -> u32 {
    slice.count_slice(sub_slice)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_sub_slice_u8_no_match() {

        let word = "hello world";
        let word_bytes = word.as_bytes();
        let sub_word = "abcd";
        let sub_word_bytes = sub_word.as_bytes();

        let result = count_sub_slice_u8(word_bytes, sub_word_bytes);
        assert_eq!(result, 0);
    }

    #[test]
    fn count_sub_slice_u8_single_match() {

        let word = "hello world";
        let word_bytes = word.as_bytes();
        let sub_word = "world";
        let sub_word_bytes = sub_word.as_bytes();

        let result = count_sub_slice_u8(word_bytes, sub_word_bytes);
        assert_eq!(result, 1);
    }

    #[test]
    fn count_sub_slice_u8_multiple_match() {
        let word = "hello world, isn't the world a nice place to live? and rejoice with the wideworld";
        let word_bytes = word.as_bytes();
        let sub_word = "world";
        let sub_word_bytes = sub_word.as_bytes();

        let result = count_sub_slice_u8(word_bytes, sub_word_bytes);
        assert_eq!(result, 3);
    }

    #[test]
    fn count_sub_slice_ref_u8_no_match() {

        let word = "hello world";
        let word_bytes_vec = word.as_bytes().iter().map(|x| x as &u8).collect::<Vec<_>>();
        let sub_word = "abcd";
        let sub_word_bytes_vec = sub_word.as_bytes().iter().map(|x| x as &u8).collect::<Vec<_>>();

        let result = count_sub_slice_ref_u8(word_bytes_vec.as_slice(), sub_word_bytes_vec.as_slice());
        assert_eq!(result, 0);
    }

    #[test]
    fn count_sub_slice_ref_u8_single_match() {

        let word = "hello world";
        let word_bytes_vec = word.as_bytes().iter().map(|x| x as &u8).collect::<Vec<_>>();
        let sub_word = "world";
        let sub_word_bytes_vec = sub_word.as_bytes().iter().map(|x| x as &u8).collect::<Vec<_>>();

        let result = count_sub_slice_ref_u8(word_bytes_vec.as_slice(), sub_word_bytes_vec.as_slice());
        assert_eq!(result, 1);
    }

    #[test]
    fn count_sub_slice_ref_u8_multiple_match() {

        let word = "hello world, isn't the world a nice place to live? and rejoice with the wideworld";
        let word_bytes_vec = word.as_bytes().iter().map(|x| x as &u8).collect::<Vec<_>>();
        let sub_word = "world";
        let sub_word_bytes_vec = sub_word.as_bytes().iter().map(|x| x as &u8).collect::<Vec<_>>();

        let result = count_sub_slice_ref_u8(word_bytes_vec.as_slice(), sub_word_bytes_vec.as_slice());
        assert_eq!(result, 3);
    }
}
