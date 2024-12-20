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

        if self.edge.contains(GridCellEdge::TOP) {
            if self.edge.contains(GridCellEdge::LEFT) {
                border_set.top_left = symbols::line::TOP_LEFT;
            } else {
                border_set.top_left = symbols::line::HORIZONTAL_DOWN;
            }

            if self.edge.contains(GridCellEdge::RIGHT) {
                border_set.top_right = symbols::line::TOP_RIGHT;
            } else {
                border_set.top_right = symbols::line::VERTICAL_LEFT;
            }
        } else if self.edge.contains(GridCellEdge::BOTTOM) {
            if self.edge.contains(GridCellEdge::LEFT) {
                border_set.top_left = symbols::line::VERTICAL_RIGHT;
                border_set.bottom_left = symbols::line::BOTTOM_LEFT;
            } else {
                if self.edge.contains(GridCellEdge::RIGHT) {
                    border_set.top_right = symbols::line::VERTICAL_LEFT
                }
                border_set.top_left = symbols::line::CROSS;
                border_set.bottom_left = symbols::line::HORIZONTAL_UP;
            }
        } else {
            if self.edge.contains(GridCellEdge::LEFT) {
                border_set.top_left = symbols::line::VERTICAL_RIGHT;
            } else {
                if self.edge.contains(GridCellEdge::RIGHT) {
                    border_set.top_right = symbols::line::VERTICAL_LEFT;
                }
                border_set.top_left = symbols::line::CROSS;
            }
        }
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
mod grid_cell {
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
