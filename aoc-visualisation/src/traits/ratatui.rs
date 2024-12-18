pub trait RatatuiStylised {
    fn get_style(&self) -> Option<ratatui::style::Style> {
        None
    }
}