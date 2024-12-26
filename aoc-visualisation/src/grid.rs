mod grid_cell;
mod grid_config;
pub mod grid_utils;

use crate::grid::grid_cell::GridCell;
use crate::grid::grid_config::GridCellEdge;
use crate::grid::grid_utils::{DisplayRowColumnNumber, DisplayNumbersType};
use crate::traits::ratatui::RatatuiStylised;
use ndarray::ArrayView2;
use ratatui::backend::Backend;
use ratatui::buffer::Buffer;
use ratatui::layout::{Layout, Rect};
use ratatui::style::Style;
use ratatui::widgets::Widget;
use ratatui::{CompletedFrame, Terminal};
use std::collections::HashMap;
use std::fmt::Display;
use std::io;
use tracing::debug;

pub struct GridVisualiser<'a, T>
where
    T: Backend,
{
    style_map: HashMap<String, Style>,
    terminal: &'a mut Terminal<T>,
    display_row_column_number: DisplayRowColumnNumber,
    max_rows: Option<usize>,
    max_cols: Option<usize>,
}

impl<'a, T: Backend> GridVisualiser<'a, T> {

    /// This is the size of a cell to hold row/col numbers in (and includes size of border)
    const DISPLAY_NUMBERS_ROW:DisplayNumbersType = DisplayNumbersType::Row(2);
    const DISPLAY_NUMBERS_COL:DisplayNumbersType = DisplayNumbersType::Column(6);

    pub fn new(terminal: &'a mut Terminal<T>, display_row_column_number: DisplayRowColumnNumber) -> Self {
        Self {
            style_map: HashMap::new(),
            display_row_column_number,
            terminal,
            max_rows: None,
            max_cols: None,
        }
    }

    pub fn new_with_limits(terminal: &'a mut Terminal<T>, display_row_column_number: DisplayRowColumnNumber, max_rows: usize, max_cols: usize) -> Self {
        Self {
            style_map: HashMap::new(),
            display_row_column_number,
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
        content_max_width: usize
    ) -> Result<(usize, usize), String> {
        let area = self.terminal.get_frame().area();
        let (mut no_rows, mut no_cols) = match self.display_row_column_number {
            DisplayRowColumnNumber::Always => (

                (area.height as usize - content_max_height - Self::DISPLAY_NUMBERS_ROW.get_value() - 2)
                    / (content_max_height + 1),
                (area.width as usize - content_max_width - Self::DISPLAY_NUMBERS_COL.get_value() - 2)
                    / (content_max_width + 1),
            ),
            DisplayRowColumnNumber::Never => (
                (area.height as usize - content_max_height - 2) / (content_max_height + 1),
                (area.width as usize - content_max_width - 2) / (content_max_width + 1),
            ),
        };

        if let Some(max_rows) = self.max_rows {
            no_rows = no_rows.min(max_rows);
        }
        if let Some(max_cols) = self.max_cols {
            no_cols = no_cols.min(max_cols);
        }

        if no_rows == 0 || no_cols == 0 {
            return Err("No space to display grid".to_string());
        }

        Ok((no_rows, no_cols))
    }

    pub fn draw_ref<C>(&mut self, grid: &ArrayView2<C>, row_offset: usize, col_offset: usize) -> io::Result<CompletedFrame>
    where
        C: RatatuiStylised,
        C: Display,
    {
        self.terminal.draw(|f| {
            let area = f.area();
            let buf = f.buffer_mut();

            let (cell_content_row_size, cell_content_col_size) = C::get_cell_content_max_dimensions();
            debug!("Cell content row size: {}, col size: {}", cell_content_row_size, cell_content_col_size);
            let row_constraint = Self::create_constraints(grid.nrows(), cell_content_row_size, &self.display_row_column_number,  Self::DISPLAY_NUMBERS_ROW).unwrap();
            let col_constraint = Self::create_constraints(grid.ncols(), cell_content_row_size, &self.display_row_column_number, Self::DISPLAY_NUMBERS_COL).unwrap();
            debug!("Row constraint: {:?}", row_constraint);
            debug!("Col constraint: {:?}", col_constraint);

            // let row_modifier;
            // let col_modifier;
            // match self.display_row_column_number {
            //     DisplayRowColumnNumber::Always => {
            //         row_modifier = 1;
            //         col_modifier = 1;
            //     }
            //     DisplayRowColumnNumber::Never => {
            //         row_modifier = 0;
            //         col_modifier = 0;
            //     }
            // }
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
                    let value : String;
                    if row_idx == 0 && self.display_row_column_number == DisplayRowColumnNumber::Always {
                        if col_idx == 0 {
                            value = "".to_string();
                        } else {
                            value = format!("{}", col_offset + col_idx - 1);
                        }
                    } else {
                        if col_idx == 0 && self.display_row_column_number == DisplayRowColumnNumber::Always {
                            value = format!("{}", row_offset + row_idx - 1);
                        } else {
                            value = grid[[row_idx, col_idx]].to_string();
                        }
                    }
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

    fn create_constraints(n: usize, cell_size: usize, show_numbers: &DisplayRowColumnNumber, constraint_type: DisplayNumbersType) -> Result<Vec<ratatui::layout::Constraint>, String> {

        let no_items: usize;
        let mut constraints = Vec::with_capacity(n);
        match show_numbers {
            DisplayRowColumnNumber::Always => {
                no_items = n - 1;

                if no_items == 0 {
                    return Err("No content to display as only 1 item but numbers column is shown".to_string())
                }
                match constraint_type {
                    DisplayNumbersType::Row(height) => { constraints.push(ratatui::layout::Constraint::Length(height as u16))}
                    DisplayNumbersType::Column(width) => { constraints.push(ratatui::layout::Constraint::Length(width as u16))}
                }
            }
            DisplayRowColumnNumber::Never => {
                no_items = n;
            }
        }
        (0..no_items)
            .enumerate()
            .for_each(|(idx, _)| {
                if idx == no_items - 1 {
                    constraints.push(ratatui::layout::Constraint::Length(cell_size as u16 + 2));
                } else {
                    constraints.push(ratatui::layout::Constraint::Length(cell_size as u16 + 1));
                }
            });
        Ok(constraints)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{array, s};

    use ratatui::backend::TestBackend;
    use ratatui::layout::Constraint;
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
    fn test_draw_ref_full_grid_no_display_numbers() {
        let grid = array![
            [TestGridItem::new('A'), TestGridItem::new('B')],
            [TestGridItem::new('C'), TestGridItem::new('D')]
        ];

        let grid_view = grid.view();

        let mut terminal = Terminal::new(TestBackend::new(5, 5)).unwrap();

        let mut visualiser = GridVisualiser::new(&mut terminal,DisplayRowColumnNumber::Never);




        let result = visualiser.draw_ref(&grid_view, 0, 0);

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
    fn test_draw_ref_full_grid_with_display_number_always_no_offset() {
        let grid = array![
            [TestGridItem::new('A'), TestGridItem::new('B')],
            [TestGridItem::new('C'), TestGridItem::new('D')]
        ];

        let grid_view = grid.view();

        let mut terminal = Terminal::new(TestBackend::new(12, 8)).unwrap();

        let mut visualiser = GridVisualiser::new(&mut terminal,DisplayRowColumnNumber::Always);

        let result = visualiser.draw_ref(&grid_view, 0, 0);

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
        let mut visualiser = GridVisualiser::new(&mut terminal, DisplayRowColumnNumber::Never);


        let result = visualiser.draw_ref(&grid_view, 0, 0);
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
        let mut visualiser = GridVisualiser::new(&mut terminal, DisplayRowColumnNumber::Never);


        let result = visualiser.draw_ref(&grid_view, 0, 0);
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
    fn test_create_constraints_with_always_row() {
        let n = 5;
        let cell_size = 3;
        let show_numbers = DisplayRowColumnNumber::Always;
        let constraint_type = DisplayNumbersType::Row(2);

        let constraints = GridVisualiser::<TestBackend>::create_constraints(n, cell_size, &show_numbers, constraint_type).unwrap();

        let expected_constraints = vec![
            Constraint::Length(2),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(5),
        ];
        assert_eq!(constraints, expected_constraints);
    }

    #[test]
    fn test_create_constraints_with_always_column() {
        let n = 3;
        let cell_size = 2;
        let show_numbers = DisplayRowColumnNumber::Always;
        let constraint_type = DisplayNumbersType::Column(1);

        let constraints = GridVisualiser::<TestBackend>::create_constraints(n, cell_size, &show_numbers, constraint_type).unwrap();

        let expected_constraints = vec![
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(4),
        ];
        assert_eq!(constraints, expected_constraints);
    }

    #[test]
    fn test_create_constraints_with_never() {
        let n = 4;
        let cell_size = 3;
        let show_numbers = DisplayRowColumnNumber::Never;

        // The `constraint_type` is not relevant when `show_numbers` is `Never`
        let constraints = GridVisualiser::<TestBackend>::create_constraints(n, cell_size, &show_numbers, DisplayNumbersType::Row(0)).unwrap();

        let expected_constraints = vec![
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(5),
        ];
        assert_eq!(constraints, expected_constraints);
    }

    #[test]
    fn test_create_constraints_with_single_item() {
        let n = 1;
        let cell_size = 7;
        let show_numbers = DisplayRowColumnNumber::Always;
        let constraint_type = DisplayNumbersType::Column(3);

        let result = GridVisualiser::<TestBackend>::create_constraints(n, cell_size, &show_numbers, constraint_type);

        match result {
            Ok(_) => { panic!("Error not raised when expected")}
            Err(_) => {}
        }
    }

    #[test]
    fn test_calculate_viewable_grid_size_identifiers_always() {
        let mock_backend = TestBackend::new(20, 10); // Mock terminal with 20x10 size
        let mut terminal = Terminal::new(mock_backend).unwrap();
        let mut visualiser = GridVisualiser::new(& mut terminal, DisplayRowColumnNumber::Always);

        let (rows, cols) = visualiser.calculate_viewable_grid_size(
            1,
            1,
        ).unwrap();

        assert_eq!(rows, 2); // (10 - NUMBERS_ROW_HEIGHT(3) - MAX_CONTENT_HEIGHT(1) - 2 / (MAX_CONTENT_HEIGHT(1) + 1)
        assert_eq!(cols, 5); // (20 - NUMBERS_COL_WIDTH(6) -  MAX_CONTENT_HEIGHT(1) - 2) / (MAX_CONTENT_WIDTH(1) + 1 )
    }

    #[test]
    fn test_calculate_viewable_grid_size_identifiers_never() {
        let mock_backend = TestBackend::new(20, 10); // Mock terminal with 20x10 size
        let mut terminal = Terminal::new(mock_backend).unwrap();
        let mut visualiser = GridVisualiser::new(& mut terminal, DisplayRowColumnNumber::Never);

        let (rows, cols) = visualiser.calculate_viewable_grid_size(
            1,
            1,
        ).unwrap();

        assert_eq!(rows, 3); // (10 - MAX_CONTENT_HEIGHT(1) - 2 / (MAX_CONTENT_HEIGHT(1) + 1)
        assert_eq!(cols, 8); // (20 - MAX_CONTENT_HEIGHT(1) - 2) / (MAX_CONTENT_WIDTH(1) + 1 )
    }


    #[test]
    fn test_calculate_viewable_grid_size_with_set_limits() {
        let mock_backend = TestBackend::new(20, 10); // Mock terminal with 20x10 size
        let mut terminal = Terminal::new(mock_backend).unwrap();

        // Initialize with max_rows=1 and max_cols=2
        let mut visualiser = GridVisualiser::new_with_limits(&mut terminal, DisplayRowColumnNumber::Always,1, 2);

        // Test behavior with set limits for Always
        let (rows, cols) = visualiser
            .calculate_viewable_grid_size(1, 1)
            .unwrap();

        assert_eq!(rows, 1); // Restricted by max_rows
        assert_eq!(cols, 2); // Restricted by max_cols

        let mut visualiser = GridVisualiser::new_with_limits(&mut terminal, DisplayRowColumnNumber::Never,1, 2);

        // Test behavior with set limits for Never
        let (rows, cols) = visualiser
            .calculate_viewable_grid_size(1, 1)
            .unwrap();

        assert_eq!(rows, 1); // Restricted by max_rows
        assert_eq!(cols, 2); // Restricted by max_cols
    }

    #[test]
    fn test_calculate_viewable_grid_size_with_large_limits() {
        let mock_backend = TestBackend::new(30, 15); // Mock terminal with 30x15 size
        let mut terminal = Terminal::new(mock_backend).unwrap();

        // Initialize with large max_rows and max_cols
        let mut visualiser = GridVisualiser::new_with_limits(&mut terminal, DisplayRowColumnNumber::Never, 10, 10);

        let (rows, cols) = visualiser
            .calculate_viewable_grid_size(2, 2)
            .unwrap();

        assert_eq!(rows, 3); // (15 - MAX_CONTENT_HEIGHT(2) - 2 / (MAX_CONTENT_HEIGHT(2) + 1)
        assert_eq!(cols, 8); // (30 - MAX_CONTENT_HEIGHT(2) - 2) / (MAX_CONTENT_WIDTH(2) + 1 )
    }

    #[test]
    fn test_calculate_viewable_grid_size_limits_exceeded() {
        let mock_backend = TestBackend::new(20, 10); // Mock terminal with 20x10 size
        let mut terminal = Terminal::new(mock_backend).unwrap();

        // Initialize with max_rows=0 and max_cols=0 (no space)
        let mut visualiser = GridVisualiser::new_with_limits(&mut terminal, DisplayRowColumnNumber::Never, 0, 0);

        let result = visualiser.calculate_viewable_grid_size(1, 1);

        assert!(result.is_err()); // Should return an error
    }
}