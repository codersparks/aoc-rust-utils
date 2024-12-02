
pub trait LineProcessor {
    type Item;
    type ProcessorError;
    fn process(&self, line: &str) -> Result<Self::Item, Self::ProcessorError>;
}