mod grid_cell;
mod grid_config;

use crate::grid::grid_cell::GridCell;
use crate::grid::grid_config::GridCellEdge;
use crate::traits::ratatui::RatatuiStylised;
use ndarray::ArrayView2;
use ratatui::buffer::Buffer;
use ratatui::layout::{Layout, Rect};
use ratatui::style::Style;
use ratatui::widgets::Widget;
use std::collections::HashMap;
use std::fmt::Display;

pub struct GridVisualiser {
    style_map: HashMap<String, Style>,
    cell_row_size: usize,
    cell_col_size: usize,
}

impl GridVisualiser {
    pub fn new(cell_row_size: usize, cell_col_size: usize) -> Self {
        Self {
            style_map: HashMap::new(),
            cell_row_size,
            cell_col_size,
        }
    }

    pub fn add_style(&mut self, name: String, style: Style) {
        self.style_map.insert(name, style);
    }

    pub fn draw<T>(&self, grid: ArrayView2<T>, area: Rect, buf: &mut Buffer)
    where
        T: RatatuiStylised,
        T: Display,
    {
        let row_constraint = Self::create_constraints(grid.nrows(), self.cell_row_size);

        let rows = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(row_constraint)
            .split(area);

        for (row_idx, grid_row) in rows.iter().enumerate() {
            let col_constraint = Self::create_constraints(grid.ncols(), self.cell_col_size);
            let cols = Layout::default()
                .direction(ratatui::layout::Direction::Horizontal)
                .constraints(col_constraint)
                .split(*grid_row);

            for (col_idx, grid_col) in cols.iter().enumerate() {
                let value = grid[[row_idx, col_idx]].to_string();

                let grid_cell;

                let mut edge: GridCellEdge = GridCellEdge::empty();
                if row_idx == 0 {
                    edge |= GridCellEdge::TOP;
                } else if row_idx == grid.nrows() - 1 {
                    edge |= GridCellEdge::BOTTOM;
                }

                if col_idx == 0 {
                    edge |= GridCellEdge::LEFT;
                } else if col_idx == grid.ncols() - 1 {
                    edge |= GridCellEdge::RIGHT;
                }

                if let Some(s) = grid[[row_idx, col_idx]].get_style() {
                    grid_cell = GridCell::with_style(value, s.clone(), edge);
                } else {
                    grid_cell = GridCell::new(value, edge);
                }

                grid_cell.render(*grid_col, buf);
            }
        }
    }

    fn create_constraints(n: usize, cell_size: usize) -> Vec<ratatui::layout::Constraint> {
        (0..n)
            .enumerate()
            .map(|(idx, _)| {
                if idx == n-1 {
                    return ratatui::layout::Constraint::Length(cell_size as u16 + 1)
                }
                ratatui::layout::Constraint::Length(cell_size as u16)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{array, s};

    use ratatui::style::Color;
    use std::fmt::Formatter;

    struct TestGridItem {
        value: char,
    }

    impl TestGridItem {
        fn new(c: char) -> Self {
            Self { value: c }
        }
    }

    impl Display for TestGridItem {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    impl RatatuiStylised for TestGridItem {
        fn get_style(&self) -> Option<Style> {
            if self.value == '#' {
                Some(Style::default().bg(Color::Red))
            } else {
                None
            }
        }
    }
    #[test]
    fn test_render() {
        let grid = array![
            [
                TestGridItem::new('.'),
                TestGridItem::new('.'),
                TestGridItem::new('A'),
                TestGridItem::new('.')
            ],
            [
                TestGridItem::new('.'),
                TestGridItem::new('.'),
                TestGridItem::new('A'),
                TestGridItem::new('.')
            ],
            [
                TestGridItem::new('.'),
                TestGridItem::new('A'),
                TestGridItem::new('.'),
                TestGridItem::new('x')
            ],
            [
                TestGridItem::new('.'),
                TestGridItem::new('.'),
                TestGridItem::new('.'),
                TestGridItem::new('v')
            ],
        ];

        let grid_view = grid.slice(s![.., ..]);

        let visualiser = GridVisualiser::new(3, 3);

        let mut buffer = Buffer::empty(Rect::new(0, 0, 12, 12));

        visualiser.draw(grid_view, buffer.area, &mut buffer);

        #[rustfmt::skip]
            let mut expected = Buffer::with_lines([
        "┌──┬──┬──┬─┐",
        "│ .│ .│ A│.│",
        "│  │  │  │ │",
        "├──┼──┼──┼─┤",
        "│ .│ .│ A│.│",
        "│  │  │  │ │",
        "├──┼──┼──┼─┤",
        "│ .│ A│ .│x│",
        "│  │  │  │ │",
        "├──┼──┼──┼─┤",
        "│ .│ .│ .│v│",
        "└──┴──┴──┴─┘",
            ]);

        expected.set_style(Rect::new(3, 0, 6, 6), Style::default());
        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_render_limited() {
        let grid = array![
            [
                TestGridItem::new('.'),
                TestGridItem::new('.'),
                TestGridItem::new('A'),
                TestGridItem::new('.')
            ],
            [
                TestGridItem::new('.'),
                TestGridItem::new('.'),
                TestGridItem::new('A'),
                TestGridItem::new('.')
            ],
            [
                TestGridItem::new('.'),
                TestGridItem::new('A'),
                TestGridItem::new('.'),
                TestGridItem::new('x')
            ],
            [
                TestGridItem::new('.'),
                TestGridItem::new('.'),
                TestGridItem::new('.'),
                TestGridItem::new('v')
            ],
        ];

        let grid_view = grid.slice(s![1..4, 2..4]);

        let visualiser = GridVisualiser::new(3, 3);

        let mut buffer = Buffer::empty(Rect::new(0, 0, 6, 9));

        visualiser.draw(grid_view, buffer.area, &mut buffer);

        #[rustfmt::skip]
            let mut expected = Buffer::with_lines([
        "┌──┬─┐",
        "│ A│.│",
        "│  │ │",
        "├──┼─┤",
        "│ .│x│",
        "│  │ │",
        "├──┼─┤",
        "│ .│v│",
        "└──┴─┘",
            ]);

        expected.set_style(Rect::new(3, 0, 6, 6), Style::default());
        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_render_with_style_matching() {
        let grid = array![
            [
                TestGridItem::new('.'),
                TestGridItem::new('.'),
                TestGridItem::new('A'),
                TestGridItem::new('.')
            ],
            [
                TestGridItem::new('.'),
                TestGridItem::new('#'),
                TestGridItem::new('A'),
                TestGridItem::new('.')
            ],
            [
                TestGridItem::new('.'),
                TestGridItem::new('A'),
                TestGridItem::new('.'),
                TestGridItem::new('x')
            ],
            [
                TestGridItem::new('.'),
                TestGridItem::new('.'),
                TestGridItem::new('.'),
                TestGridItem::new('v')
            ],
        ];

        let grid_view = grid.slice(s![..2, ..2]);

        let mut visualiser = GridVisualiser::new(3, 3);
        visualiser.add_style(
            "#".to_string(),
            Style::default().bg(ratatui::style::Color::Red),
        );

        let mut buffer = Buffer::empty(Rect::new(0, 0, 6, 6));

        visualiser.draw(grid_view, buffer.area, &mut buffer);

        #[rustfmt::skip]
            let mut expected = Buffer::with_lines([
        "┌──┬─┐",
        "│ .│.│",
        "│  │ │",
        "├──┼─┤",
        "│ .│#│",
        "└──┴─┘",
            ]);

        expected.set_style(Rect::new(4,4,1,1), Style::default().bg(ratatui::style::Color::Red));
        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_render_with_style_not_matching() {
        let grid = array![[TestGridItem::new('.')],];

        let grid_view = grid.slice(s![.., ..]);

        let mut visualiser = GridVisualiser::new(3, 3);
        visualiser.add_style(
            "#".to_string(),
            Style::default().bg(ratatui::style::Color::Red),
        );

        let mut buffer = Buffer::empty(Rect::new(0, 0, 3, 3));

        visualiser.draw(grid_view, buffer.area, &mut buffer);

        #[rustfmt::skip]
            let expected = Buffer::with_lines([
        "┌──",
        "│ .",
        "│  ",
            ]);

        assert_eq!(buffer, expected);
    }
}
