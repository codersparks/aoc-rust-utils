mod grid_cell;
mod grid_config;
pub mod grid_utils;

use crate::grid::grid_cell::GridCell;
use crate::grid::grid_config::GridCellEdge;
use crate::grid::grid_utils::DisplayRowColumnNumber;
use crate::traits::ratatui::RatatuiStylised;
use ndarray::ArrayView2;
use ratatui::backend::Backend;
use ratatui::buffer::Buffer;
use ratatui::layout::{Layout, Rect};
use ratatui::style::Style;
use ratatui::widgets::{Clear, Widget};
use ratatui::{CompletedFrame, Terminal};
use std::collections::HashMap;
use std::fmt::Display;
use std::io;

pub struct GridVisualiser<'a, T>
where
    T: Backend,
{
    style_map: HashMap<String, Style>,
    terminal: &'a mut Terminal<T>,
    max_rows: Option<usize>,
    max_cols: Option<usize>,
}

impl<'a, T: Backend> GridVisualiser<'a, T> {

    /// This is the size of a cell to hold row/col numbers in (and includes size of border)
    const NUMBERS_ROW_HEIGHT:usize = 3;
    const NUMBERS_COL_WIDTH:usize = 6;

    pub fn new(terminal: &'a mut Terminal<T>) -> Self {
        Self {
            style_map: HashMap::new(),
            terminal,
            max_rows: None,
            max_cols: None,
        }
    }

    pub fn new_with_limits(terminal: &'a mut Terminal<T>, max_rows: usize, max_cols: usize) -> Self {
        Self {
            style_map: HashMap::new(),
            terminal,
            max_rows: Some(max_rows),
            max_cols: Some(max_cols),
        }
    }

    pub fn add_style(&mut self, name: String, style: Style) {
        self.style_map.insert(name, style);
    }

    /// This is a utility function that calculates the number of rows and cols that can be displayed
    /// based on cell width and height
    /// Returns (Row Nos, Col Nos)
    pub fn calculate_viewable_grid_size(
        &mut self,
        content_max_height: usize,
        content_max_width: usize,
        show_identifiers: DisplayRowColumnNumber,
    ) -> Result<(usize, usize), String> {


        let area = self.terminal.get_frame().area();

        let mut no_rows:usize;
        let mut no_cols:usize;

        match show_identifiers {
            DisplayRowColumnNumber::Always => {

                // We need to remove the size of the row/coll, before dividing the remaining space

                no_rows = (area.height as usize - Self::NUMBERS_ROW_HEIGHT - 1) / (content_max_height + 2);
                no_cols = (area.width as usize  - Self::NUMBERS_COL_WIDTH - 1) / (content_max_width + 2);

            }
            DisplayRowColumnNumber::Dynamic => {
                unimplemented!("Dynamic display of row/col number of grid not yet implemented")
            }
            DisplayRowColumnNumber::Never => {
                no_rows = (area.height as usize - 1) / (content_max_height + 2);
                no_cols = (area.width as usize - 1) / (content_max_width + 2);

            }
        }

        if self.max_rows.is_some() && no_rows > self.max_rows.unwrap() {
            no_rows = self.max_rows.unwrap();
        }

        if self.max_cols.is_some() && no_cols > self.max_cols.unwrap() {
            no_cols = self.max_cols.unwrap();
        }

        if no_rows == 0 || no_cols == 0 {
            return Err("No space to display grid".to_string());
        }

        Ok((no_rows, no_cols))
    }

    // pub fn draw<C>(&self, grid: ArrayView2<C>, area: Rect, buf: &mut Buffer)
    // where
    //     C: RatatuiStylised,
    //     C: Display,
    // {
    //
    //     let row_constraint = Self::create_constraints(grid.nrows(), self.cell_row_size);
    //
    //     let rows = Layout::default()
    //         .direction(ratatui::layout::Direction::Vertical)
    //         .constraints(row_constraint)
    //         .split(area);
    //
    //     for (row_idx, grid_row) in rows.iter().enumerate() {
    //         let col_constraint = Self::create_constraints(grid.ncols(), self.cell_col_size);
    //         let cols = Layout::default()
    //             .direction(ratatui::layout::Direction::Horizontal)
    //             .constraints(col_constraint)
    //             .split(*grid_row);
    //
    //         for (col_idx, grid_col) in cols.iter().enumerate() {
    //             let value = grid[[row_idx, col_idx]].to_string();
    //
    //             let grid_cell;
    //
    //             let mut edge: GridCellEdge = GridCellEdge::empty();
    //             if row_idx == 0 {
    //                 edge |= GridCellEdge::TOP;
    //             } else if row_idx == grid.nrows() - 1 {
    //                 edge |= GridCellEdge::BOTTOM;
    //             }
    //
    //             if col_idx == 0 {
    //                 edge |= GridCellEdge::LEFT;
    //             } else if col_idx == grid.ncols() - 1 {
    //                 edge |= GridCellEdge::RIGHT;
    //             }
    //
    //             if let Some(s) = grid[[row_idx, col_idx]].get_style() {
    //                 grid_cell = GridCell::with_style(value, s.clone(), edge);
    //             } else {
    //                 grid_cell = GridCell::new(value, edge);
    //             }
    //
    //             grid_cell.render(*grid_col, buf);
    //         }
    //     }
    // }
    //

    pub fn draw_ref<C>(&mut self, grid: &ArrayView2<C>) -> io::Result<CompletedFrame>
    where
        C: RatatuiStylised,
        C: Display,
    {
        self.terminal.draw(|f| {
            let area = f.area();
            let buf = f.buffer_mut();

            f.render_widget(Clear, area);

            let (cell_content_row_size, cell_content_col_size) = C::get_cell_content_max_dimensions();
            let row_constraint = Self::create_constraints(grid.nrows(), cell_content_row_size);
            let col_constraint = Self::create_constraints(grid.ncols(), cell_content_col_size);

            let rows = Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints(row_constraint)
                .split(area);

            for (row_idx, grid_row) in rows.iter().enumerate() {
                let cols = Layout::default()
                    .direction(ratatui::layout::Direction::Horizontal)
                    .constraints(col_constraint.clone())
                    .split(*grid_row);

                for (col_idx, grid_col) in cols.iter().enumerate() {
                    let value = grid[[row_idx, col_idx]].to_string();

                    let grid_cell;

                    let mut edge: GridCellEdge = GridCellEdge::empty();
                    if row_idx == 0 {
                        edge |= GridCellEdge::TOP;
                    }
                    if row_idx == grid.nrows() - 1 {
                        edge |= GridCellEdge::BOTTOM;
                    }

                    if col_idx == 0 {
                        edge |= GridCellEdge::LEFT;
                    }
                    if col_idx == grid.ncols() - 1 {
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
        })
    }

    fn create_constraints(n: usize, cell_size: usize) -> Vec<ratatui::layout::Constraint> {
        (0..n)
            .enumerate()
            .map(|(idx, _)| {
                if idx == n - 1 {
                    return ratatui::layout::Constraint::Length(cell_size as u16 + 2);
                }
                ratatui::layout::Constraint::Length(cell_size as u16 + 1)
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
    use ratatui::backend::TestBackend;
    use ratatui::layout::Constraint;

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
    fn test_draw_ref_full_grid() {
        let grid = array![
            [TestGridItem::new('A'), TestGridItem::new('B')],
            [TestGridItem::new('C'), TestGridItem::new('D')]
        ];

        let grid_view = grid.view();

        let mut terminal = Terminal::new(TestBackend::new(5, 5)).unwrap();

        let mut visualiser = GridVisualiser::new(&mut terminal);




        let result = visualiser.draw_ref(&grid_view);

        assert!(result.is_ok());

        // Validate buffer contents
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
            "┌─┬─┐",
            "│A│B│",
            "├─┼─┤",
            "│C│D│",
            "└─┴─┘",
        ]);

        terminal.backend().assert_buffer(&expected);
    }

    #[test]
    fn test_draw_ref_with_style() {
        let grid = array![
            [TestGridItem::new('A'), TestGridItem::new('#')],
            [TestGridItem::new('C'), TestGridItem::new('D')]
        ];

        let grid_view = grid.view();

        let test_backend = TestBackend::new(5, 5);
        let mut terminal = Terminal::new(test_backend).unwrap();
        let mut visualiser = GridVisualiser::new(&mut terminal);


        let result = visualiser.draw_ref(&grid_view);
        assert!(result.is_ok());

        // Validate buffer contents
        #[rustfmt::skip]
        let mut expected = Buffer::with_lines([
            "┌─┬─┐",
            "│A│#│",
            "├─┼─┤",
            "│C│D│",
            "└─┴─┘",
        ]);
        // Validate that the '#' item got a red background
        expected.set_style(Rect::new(3, 1, 1, 1), Style::default().bg(Color::Red));

        terminal.backend_mut().assert_buffer(&expected);

    }

    #[test]
    fn test_draw_ref_partial_grid() {
        let grid = array![
            [TestGridItem::new('A'), TestGridItem::new('B')],
            [TestGridItem::new('C'), TestGridItem::new('D')]
        ];

        // Partial slice of the grid
        let grid_view = grid.slice(s![.., 1..]);

        let mut terminal = Terminal::new(TestBackend::new(3, 5)).unwrap();
        let mut visualiser = GridVisualiser::new(&mut terminal);


        let result = visualiser.draw_ref(&grid_view);
        assert!(result.is_ok());

        // Validate buffer contents
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
            "┌─┐",
            "│B│",
            "├─┤",
            "│D│",
            "└─┘",
        ]);

        terminal.backend_mut().assert_buffer(&expected);
    }

    #[test]
    fn test_create_constraints_multiple_cells() {
        let constraints = GridVisualiser::<TestBackend>::create_constraints(3, 5);

        // Check if the length matches the number of cells
        assert_eq!(constraints.len(), 3);

        // Check each constraint is correctly set
        assert_eq!(constraints[0], Constraint::Length(6)); // First constraint is +1
        assert_eq!(constraints[1], Constraint::Length(6)); // Second constraint is +1
        assert_eq!(constraints[2], Constraint::Length(7)); // Last constraint is +2
    }

    #[test]
    fn test_create_constraints_single_cell() {
        let constraints = GridVisualiser::<TestBackend>::create_constraints(1, 10);

        assert_eq!(constraints.len(), 1);
        assert_eq!(constraints[0], Constraint::Length(12)); // Single cell gets +2
    }

    #[test]
    fn test_calculate_viewable_grid_size_identifiers_always() {
        let mock_backend = TestBackend::new(20, 10); // Mock terminal with 20x10 size
        let mut terminal = Terminal::new(mock_backend).unwrap();
        let mut visualiser = GridVisualiser::new(& mut terminal);

        let (rows, cols) = visualiser.calculate_viewable_grid_size(
            1,
            1,
            DisplayRowColumnNumber::Always,
        ).unwrap();

        assert_eq!(rows, 2); // (10 - NUMBERS_ROW_HEIGHT - 1) / (1 + 2)
        assert_eq!(cols, 4); // (20 - NUMBERS_COL_WIDTH - 1) / (1 + 2)
    }

    #[test]
    fn test_calculate_viewable_grid_size_identifiers_never() {
        let mock_backend = TestBackend::new(20, 10); // Mock terminal with 20x10 size
        let mut terminal = Terminal::new(mock_backend).unwrap();
        let mut visualiser = GridVisualiser::new(& mut terminal);

        let (rows, cols) = visualiser.calculate_viewable_grid_size(
            1,
            1,
            DisplayRowColumnNumber::Never,
        ).unwrap();

        assert_eq!(rows, 3); // (10 - 1) / (1 + 2)
        assert_eq!(cols, 6); // (20 - 1) / (1 + 2)
    }


    #[test]
    fn test_calculate_viewable_grid_size_small_terminal_area() {
        let mock_backend = TestBackend::new(8, 5); // Mock terminal with 8x5 size
        let mut terminal = Terminal::new(mock_backend).unwrap();
        let mut visualiser = GridVisualiser::new(& mut terminal);

        let result = visualiser.calculate_viewable_grid_size(
            3,
            6,
            DisplayRowColumnNumber::Always,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_calculate_viewable_grid_size_with_set_limits() {
        let mock_backend = TestBackend::new(20, 10); // Mock terminal with 20x10 size
        let mut terminal = Terminal::new(mock_backend).unwrap();

        // Initialize with max_rows=1 and max_cols=2
        let mut visualiser = GridVisualiser::new_with_limits(&mut terminal, 1, 2);

        // Test behavior with set limits for Always
        let (rows, cols) = visualiser
            .calculate_viewable_grid_size(1, 1, DisplayRowColumnNumber::Always)
            .unwrap();

        assert_eq!(rows, 1); // Restricted by max_rows
        assert_eq!(cols, 2); // Restricted by max_cols

        // Test behavior with set limits for Never
        let (rows, cols) = visualiser
            .calculate_viewable_grid_size(1, 1, DisplayRowColumnNumber::Never)
            .unwrap();

        assert_eq!(rows, 1); // Restricted by max_rows
        assert_eq!(cols, 2); // Restricted by max_cols
    }

    #[test]
    fn test_calculate_viewable_grid_size_with_large_limits() {
        let mock_backend = TestBackend::new(30, 15); // Mock terminal with 30x15 size
        let mut terminal = Terminal::new(mock_backend).unwrap();

        // Initialize with large max_rows and max_cols
        let mut visualiser = GridVisualiser::new_with_limits(&mut terminal, 10, 10);

        let (rows, cols) = visualiser
            .calculate_viewable_grid_size(2, 2, DisplayRowColumnNumber::Never)
            .unwrap();

        assert_eq!(rows, 3); // Limited by terminal size, not max_rows
        assert_eq!(cols, 7); // Limited by terminal size, not max_cols
    }

    #[test]
    fn test_calculate_viewable_grid_size_limits_exceeded() {
        let mock_backend = TestBackend::new(20, 10); // Mock terminal with 20x10 size
        let mut terminal = Terminal::new(mock_backend).unwrap();

        // Initialize with max_rows=0 and max_cols=0 (no space)
        let mut visualiser = GridVisualiser::new_with_limits(&mut terminal, 0, 0);

        let result = visualiser.calculate_viewable_grid_size(1, 1, DisplayRowColumnNumber::Never);

        assert!(result.is_err()); // Should return an error
    }
}