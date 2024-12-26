
#[derive(PartialEq)]
pub enum DisplayRowColumnNumber {
    Always,
    Never,
}

pub enum DisplayNumbersType {
    Row(usize),
    Column(usize)
}

impl DisplayNumbersType {
    pub fn get_value(self) -> usize {
        match self {
            DisplayNumbersType::Row(v) => {v}
            DisplayNumbersType::Column(v) => {v}
        }
    }

}

