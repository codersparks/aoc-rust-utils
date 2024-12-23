pub trait RatatuiStylised {
    fn get_style(&self) -> Option<ratatui::style::Style> {
        None
    }

    fn get_cell_content_max_dimensions() -> (usize, usize) {
        (1, 1)
    }
}