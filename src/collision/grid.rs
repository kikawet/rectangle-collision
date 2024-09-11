use std::{fmt::Debug, ops::RangeInclusive};

use raylib::{color::Color, math::Vector2};

use crate::traits::Draw;

#[derive(Debug, PartialEq)]
struct Cell<T>(Vec<T>);

#[derive(PartialEq, Clone)]
pub struct Row(pub usize);
#[derive(PartialEq, Clone)]
pub struct Col(pub usize);

pub struct Grid<T> {
    data: Vec<Cell<T>>,
    rows: usize,
    cols: usize,
    pub spacing: f32,
}

impl<T> Grid<T> {
    pub fn new(rows: usize, cols: usize, spacing: f32) -> Self {
        let data = (0..rows * cols).map(|_| Cell::empty()).collect();

        Self {
            data,
            rows,
            cols,
            spacing,
        }
    }

    pub fn get(&self, row: &Row, col: &Col) -> Option<&Vec<T>> {
        self.data.get(self.get_index(row, col)).map(|cell| &cell.0)
    }

    pub fn get_many(&self, rows: RangeInclusive<Row>, cols: RangeInclusive<Col>) -> Vec<&T> {
        let cols = Self::map_range(cols);
        let rows = Self::map_range(rows);
        cols.flat_map(|col| {
            rows.clone()
                .map(move |row| self.get_index(&Row(row), &Col(col)))
        })
        .filter_map(|index| self.data.get(index))
        .flat_map(|cell| &cell.0)
        .collect::<Vec<_>>()
    }

    fn get_index(&self, row: &Row, col: &Col) -> usize {
        row.0 * self.cols + col.0
    }

    fn get_vec_index(row: &Row, col: &Col, cols: usize) -> usize {
        row.0 * cols + col.0
    }

    fn get_grid_index(index: usize, cols: usize) -> (Row, Col) {
        let row = index / cols;
        let col = index % cols;

        (Row(row), Col(col))
    }

    pub fn set(&mut self, item: T, row: &Row, col: &Col) -> Option<()> {
        let index = self.get_index(row, col);
        self.data.get_mut(index).map(|cell| cell.insert(item))
    }

    fn map_range<Item: Into<usize> + Clone>(range: RangeInclusive<Item>) -> RangeInclusive<usize> {
        let start = range.start().clone().into();
        let end = range.end().clone().into();
        start..=end
    }
}

impl<T: Clone> Grid<T> {
    pub fn set_many(
        &mut self,
        item: T,
        rows: RangeInclusive<Row>,
        cols: RangeInclusive<Col>,
    ) -> Option<()> {
        let cols = Self::map_range(cols);
        let rows = Self::map_range(rows);
        let grid_cols = self.cols;
        cols.flat_map(|col| {
            rows.clone()
                .map(move |row| Self::get_vec_index(&Row(row), &Col(col), grid_cols))
        })
        .map(|index| {
            self.data
                .get_mut(index)
                .map(|cell| cell.insert(item.clone()))
        })
        .collect::<Option<_>>()
    }
}
impl<T: PartialEq> Grid<T> {
    pub fn get_many_unique(&self, rows: RangeInclusive<Row>, cols: RangeInclusive<Col>) -> Vec<&T> {
        let cols = Self::map_range(cols);
        let rows = Self::map_range(rows);
        cols.flat_map(|col| {
            rows.clone()
                .map(move |row| self.get_index(&Row(row), &Col(col)))
        })
        .filter_map(|index| self.data.get(index))
        .flat_map(|cell| &cell.0)
        .fold(vec![], |mut acc, cur| {
            if !acc.contains(&cur) {
                acc.push(cur);
            }
            acc
        })
    }

    pub fn set_unique(&mut self, item: T, row: &Row, col: &Col) -> Option<bool> {
        let index = self.get_index(row, col);
        self.data
            .get_mut(index)
            .map(|cell| cell.insert_unique(item))
    }
}

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        writeln!(f, "cols: {}", self.cols)?;
        writeln!(f, "rows: {}", self.rows)?;
        writeln!(f, "spacing: {:?}", self.spacing)?;
        writeln!(f, "data:")?;
        self.data
            .iter()
            .enumerate()
            .filter(|(_, cell)| !cell.0.is_empty())
            .try_for_each(|(index, cell)| {
                writeln!(
                    f,
                    "\t{:?}: {:#?}",
                    Self::get_grid_index(index, self.cols),
                    cell.0
                )
            })
    }
}

impl<T> Cell<T> {
    fn empty() -> Self {
        Self(Vec::with_capacity(8))
    }

    pub fn single(item: T) -> Self {
        let mut v = Vec::with_capacity(8);
        v.push(item);
        Self(v)
    }

    fn insert(&mut self, item: T) {
        self.0.push(item);
    }
}

impl<T: PartialEq> Cell<T> {
    fn insert_unique(&mut self, item: T) -> bool {
        if !self.0.contains(&item) {
            self.0.push(item);
            return true;
        }

        false
    }
}

impl<T> Default for Cell<T> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<T> Draw for Grid<T> {
    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_possible_truncation)]
    fn draw(&self, canvas: &mut impl raylib::prelude::RaylibDraw) {
        let color = Color::LIGHTGRAY;
        let line_thick: f32 = 1.;
        let width = self.cols as f32 * self.spacing * line_thick;
        let height = self.rows as f32 * self.spacing * line_thick;

        for row in 0..self.rows {
            let y = row as f32 * self.spacing * line_thick;
            canvas.draw_line_ex(
                Vector2::new(0., y),
                Vector2::new(width, y),
                line_thick,
                color,
            );
        }

        for col in 0..self.cols {
            let x = col as f32 * self.spacing * line_thick;
            canvas.draw_line_ex(
                Vector2::new(x, 0.),
                Vector2::new(x, height),
                line_thick,
                color,
            );
        }

        for index in 0..self.cols * self.rows {
            let (Row(row), Col(col)) = Self::get_grid_index(index, self.cols);
            let x = col as f32 * self.spacing * line_thick + 1.;
            let y = row as f32 * self.spacing * line_thick + 1.;

            canvas.draw_text(
                format!("{row},{col}").as_str(),
                x as i32,
                y as i32,
                1,
                Color::GRAY,
            );
        }
    }
}

impl From<Row> for usize {
    fn from(val: Row) -> Self {
        val.0
    }
}

impl Debug for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Row({})", self.0)
    }
}

impl From<Col> for usize {
    fn from(val: Col) -> Self {
        val.0
    }
}

impl Debug for Col {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Col({})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        assert_eq!(Grid::<usize>::get_vec_index(&Row(0), &Col(1), 10), 1);
        assert_eq!(Grid::<usize>::get_grid_index(1, 10), (Row(0), Col(1)));

        assert_eq!(Grid::<usize>::get_vec_index(&Row(2), &Col(1), 5), 11);
        assert_eq!(Grid::<usize>::get_grid_index(11, 5), (Row(2), Col(1)));
    }

    #[test]
    fn test_get() {
        let grid: Grid<i8> = Grid::new(10, 41, 1.);

        assert_eq!(grid.get(&Row(0), &Col(0)), Some(&vec![]));
        assert_eq!(grid.get(&Row(9), &Col(40)), Some(&vec![]));
        assert_eq!(grid.get(&Row(10), &Col(40)), None);
        assert_eq!(grid.get(&Row(9), &Col(41)), None);
    }

    #[test]
    fn test_set_unique() {
        let mut grid = Grid::new(50, 50, 1.);

        assert_eq!(grid.get(&Row(9), &Col(15)), Some(&vec![]));
        assert_eq!(grid.set_unique(69, &Row(9), &Col(15)), Some(true));
        assert_eq!(grid.get(&Row(9), &Col(15)), Some(&vec![69]));

        assert_eq!(grid.set_unique(69, &Row(9), &Col(15)), Some(false));
        assert_eq!(grid.get(&Row(9), &Col(15)), Some(&vec![69]));

        assert_eq!(grid.set_unique(420, &Row(9), &Col(15)), Some(true));
        assert_eq!(grid.get(&Row(9), &Col(15)), Some(&vec![69, 420]));
    }

    #[test]
    fn test_get_many() {
        let mut grid = Grid::new(50, 50, 1.);

        assert_eq!(grid.set_unique(10, &Row(10), &Col(10)), Some(true));
        assert_eq!(grid.set_unique(10, &Row(15), &Col(15)), Some(true));
        assert_eq!(grid.set_unique(420, &Row(15), &Col(15)), Some(true));
        assert_eq!(grid.set_unique(69, &Row(20), &Col(20)), Some(true));
        assert_eq!(grid.set_unique(13, &Row(21), &Col(21)), Some(true));

        let mut many = grid.get_many_unique(Row(10)..=Row(20), Col(10)..=Col(20));
        assert_eq!(many.len(), 3);

        many.sort();
        assert_eq!(many, [&10, &69, &420]);
    }
}
