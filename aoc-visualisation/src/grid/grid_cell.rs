use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Style, Widget};
use ratatui::symbols;
use ratatui::widgets::{Block, Borders, Paragraph};

pub(crate) struct GridCell {
    value: String,
    style: Style,
}

impl GridCell {
    pub fn new(value: String) -> Self {
        Self {
            value,
            style: Style::default(),
        }
    }

    pub fn with_style(value: String, style: Style) -> Self {
        Self { value, style }
    }
}

impl Widget for GridCell {
    fn render(self, area: Rect, buf: &mut Buffer) {

        let border_set = symbols::border::Set {
            top_left: symbols::line::CROSS,
            ..symbols::border::PLAIN
        };
        let block = Block::default()
            .border_set(border_set)
            .borders(Borders::LEFT | Borders::TOP);

        let inner_area = block.inner(area);

        block.render(area, buf);

        Paragraph::new(self.value)
            .style(self.style)
            .centered()
            .render(inner_area, buf);
    }
}