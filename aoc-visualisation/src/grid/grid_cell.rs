use crate::grid::grid_config::GridCellEdge;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Style, Widget};
use ratatui::symbols;
use ratatui::widgets::{Block, Borders, Paragraph};

pub(crate) struct GridCell {
    value: String,
    style: Style,
    edge: GridCellEdge,
}

impl GridCell {
    pub fn new(value: String, edge: GridCellEdge) -> Self {
        Self {
            value,
            style: Style::default(),
            edge,
        }
    }

    pub fn with_style(value: String, style: Style, edge: GridCellEdge) -> Self {
        Self { value, style, edge }
    }

    fn generate_borders(&self) -> Borders {
        // All cells have top and left border
        let mut borders = Borders::TOP | Borders::LEFT;

        if self.edge.contains(GridCellEdge::RIGHT) {
            borders = borders | Borders::RIGHT;
        }

        if self.edge.contains(GridCellEdge::BOTTOM) {
            borders = borders | Borders::BOTTOM;
        }

        borders
    }

    fn generate_border_set(&self) -> symbols::border::Set {
        let mut border_set = symbols::border::PLAIN;

        border_set.top_left = match (self.edge.contains(GridCellEdge::TOP), self.edge.contains(GridCellEdge::LEFT)) {
            (true, true) => symbols::line::TOP_LEFT,
            (true, false) => symbols::line::HORIZONTAL_DOWN,
            (false, true) => symbols::line::VERTICAL_RIGHT,
            (false, false) => symbols::line::CROSS,
        };

        border_set.top_right = match (self.edge.contains(GridCellEdge::TOP), self.edge.contains(GridCellEdge::RIGHT)) {
            (true, true) => symbols::line::TOP_RIGHT,
            (false, true) => symbols::line::VERTICAL_LEFT,
            _ => border_set.top_right,
        };

        border_set.bottom_left = match (self.edge.contains(GridCellEdge::BOTTOM), self.edge.contains(GridCellEdge::LEFT)) {
            (true, true) => symbols::line::BOTTOM_LEFT,
            (true, false) => symbols::line::HORIZONTAL_UP,
            _ => border_set.bottom_left,
        };

        border_set
    }
}

impl Widget for GridCell {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border_set = self.generate_border_set();

        let borders = self.generate_borders();

        let block = Block::default().border_set(border_set).borders(borders);

        let inner_area = block.inner(area);

        block.render(area, buf);

        Paragraph::new(self.value)
            .style(self.style)
            .centered()
            .render(inner_area, buf);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_render() {
        let value = "#";

        let mut buffer = Buffer::empty(Rect::new(0, 0, 3, 3));

        let grid_cell = GridCell::new(value.to_string(), GridCellEdge::ALL);
        grid_cell.render(buffer.area, &mut buffer);

        #[rustfmt::skip]
            let expected = Buffer::with_lines([
        "┌─┐",
        "│#│",
        "└─┘",
            ]);

        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_render_top_left() {
        let value = "#";

        let mut buffer = Buffer::empty(Rect::new(0, 0, 3, 3));

        let grid_cell = GridCell::new(value.to_string(), GridCellEdge::TOP | GridCellEdge::LEFT);
        grid_cell.render(buffer.area, &mut buffer);

        #[rustfmt::skip]
            let expected = Buffer::with_lines([
        "┌──",
        "│ #",
        "│  ",
            ]);

        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_render_left() {
        let value = "#";

        let mut buffer = Buffer::empty(Rect::new(0, 0, 3, 3));

        let grid_cell = GridCell::new(value.to_string(), GridCellEdge::LEFT);
        grid_cell.render(buffer.area, &mut buffer);

        #[rustfmt::skip]
            let expected = Buffer::with_lines([
        "├──",
        "│ #",
        "│  ",
            ]);

        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_render_bottom_left() {
        let value = "#";

        let mut buffer = Buffer::empty(Rect::new(0, 0, 3, 3));

        let grid_cell = GridCell::new(value.to_string(), GridCellEdge::BOTTOM | GridCellEdge::LEFT);
        grid_cell.render(buffer.area, &mut buffer);

        #[rustfmt::skip]
            let expected = Buffer::with_lines([
        "├──",
        "│ #",
        "└──",
            ]);

        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_render_top_right() {
        let value = "#";

        let mut buffer = Buffer::empty(Rect::new(0, 0, 3, 3));

        let grid_cell = GridCell::new(value.to_string(), GridCellEdge::TOP | GridCellEdge::RIGHT);
        grid_cell.render(buffer.area, &mut buffer);

        #[rustfmt::skip]
            let expected = Buffer::with_lines([
        "┬─┐",
        "│#│",
        "│ │",
            ]);

        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_render_right() {
        let value = "#";

        let mut buffer = Buffer::empty(Rect::new(0, 0, 3, 3));

        let grid_cell = GridCell::new(value.to_string(), GridCellEdge::RIGHT);
        grid_cell.render(buffer.area, &mut buffer);

        #[rustfmt::skip]
            let expected = Buffer::with_lines([
        "┼─┤",
        "│#│",
        "│ │",
            ]);

        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_render_bottom_right() {
        let value = "#";

        let mut buffer = Buffer::empty(Rect::new(0, 0, 3, 3));

        let grid_cell = GridCell::new(value.to_string(), GridCellEdge::BOTTOM | GridCellEdge::RIGHT);
        grid_cell.render(buffer.area, &mut buffer);

        #[rustfmt::skip]
            let expected = Buffer::with_lines([
        "┼─┤",
        "│#│",
        "┴─┘",
            ]);

        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_render_other() {
        let value = "#";

        let mut buffer = Buffer::empty(Rect::new(0, 0, 3, 3));

        let grid_cell = GridCell::new(value.to_string(), GridCellEdge::empty());
        grid_cell.render(buffer.area, &mut buffer);

        #[rustfmt::skip]
            let expected = Buffer::with_lines([
        "┼──",
        "│ #",
        "│  ",
            ]);

        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_render_with_style_default() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 3, 3));

        let grid_cell = GridCell::with_style("#".to_string(), Style::default(), GridCellEdge::ALL);
        grid_cell.render(buffer.area, &mut buffer);

        #[rustfmt::skip]
            let expected = Buffer::with_lines([
        "┌─┐",
        "│#│",
        "└─┘",
            ]);

        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_render_with_style_red_bg() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 3, 3));

        let grid_cell = GridCell::with_style(
            "#".to_string(),
            Style::default().bg(ratatui::style::Color::Red),
            GridCellEdge::ALL,
        );
        grid_cell.render(buffer.area, &mut buffer);

        #[rustfmt::skip]
            let mut expected = Buffer::with_lines([
        "┌─┐",
        "│#│",
        "└─┘",
            ]);

        expected.set_style(
            Rect::new(1, 1, 1, 1),
            Style::default().bg(ratatui::style::Color::Red),
        );


        assert_eq!(buffer, expected);
    }
}
