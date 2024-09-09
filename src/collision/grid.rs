use std::ops::RangeInclusive;

use crate::traits::Draw;

#[derive(Debug, PartialEq)]
struct Cell<T>(Vec<T>);
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

    fn get(&self, row: usize, col: usize) -> Option<&Cell<T>> {
        self.data.get(self.get_index(row, col))
    }

    fn get_many(
        &self,
        cols: impl Into<RangeInclusive<usize>>,
        rows: impl Into<RangeInclusive<usize>>,
    ) -> Vec<&T> {
        let cols = cols.into();
        let rows = rows.into();
        cols.flat_map(|col| rows.clone().map(move |row| self.get_index(row, col)))
            .filter_map(|index| self.data.get(index))
            .flat_map(|cell| &cell.0)
            .collect::<Vec<_>>()
    }

    fn get_index(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    fn get_grid_index(row: usize, col: usize, cols: usize) -> usize {
        row * cols + col
    }

    pub fn set(&mut self, item: T, row: usize, col: usize) -> Option<()> {
        let index = self.get_index(row, col);
        self.data.get_mut(index).map(|cell| cell.insert(item))
    }
}

impl<T: Clone> Grid<T> {
    pub fn set_many(
        &mut self,
        item: T,
        cols: impl Into<RangeInclusive<usize>>,
        rows: impl Into<RangeInclusive<usize>>,
    ) -> Option<()> {
        let cols = cols.into();
        let rows = rows.into();
        let grid_cols = self.cols;
        cols.flat_map(|col| {
            rows.clone()
                .map(move |row| Self::get_grid_index(row, col, grid_cols))
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
    fn get_many_unique(
        &self,
        cols: impl Into<RangeInclusive<usize>>,
        rows: impl Into<RangeInclusive<usize>>,
    ) -> Vec<&T> {
        let cols = cols.into();
        let rows = rows.into();
        cols.flat_map(|col| rows.clone().map(move |row| self.get_index(row, col)))
            .filter_map(|index| self.data.get(index))
            .flat_map(|cell| &cell.0)
            .fold(vec![], |mut acc, cur| {
                if !acc.contains(&cur) {
                    acc.push(cur);
                }
                acc
            })
    }

    pub fn set_unique(&mut self, item: T, row: usize, col: usize) -> Option<bool> {
        let index = self.get_index(row, col);
        self.data
            .get_mut(index)
            .map(|cell| cell.insert_unique(item))
    }
}

impl<T> Cell<T> {
    fn empty() -> Self {
        Self(vec![])
    }

    fn single(item: T) -> Self {
        Self(vec![item])
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
    fn draw(&self, canvas: &mut impl raylib::prelude::RaylibDraw) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_grid() {
        let grid: Grid<i8> = Grid::new(10, 41, 1.);

        assert_eq!(grid.get(0, 0), Some(&Cell::empty()));
        assert_eq!(grid.get(9, 40), Some(&Cell::empty()));
        assert_eq!(grid.get(10, 40), None);
        assert_eq!(grid.get(9, 41), None);
    }

    #[test]
    fn test_set_unique() {
        let mut grid = Grid::new(50, 50, 1.);

        assert_eq!(grid.get(9, 15), Some(&Cell::empty()));
        assert_eq!(grid.set_unique(69, 9, 15), Some(true));
        assert_eq!(grid.get(9, 15), Some(&Cell::single(69)));

        assert_eq!(grid.set_unique(69, 9, 15), Some(false));
        assert_eq!(grid.get(9, 15), Some(&Cell::single(69)));

        assert_eq!(grid.set_unique(420, 9, 15), Some(true));
        assert_eq!(grid.get(9, 15), Some(&Cell(vec![69, 420])));
    }

    #[test]
    fn test_get_many() {
        let mut grid = Grid::new(50, 50, 1.);

        assert_eq!(grid.set_unique(10, 10, 10), Some(true));
        assert_eq!(grid.set_unique(10, 15, 15), Some(true));
        assert_eq!(grid.set_unique(420, 15, 15), Some(true));
        assert_eq!(grid.set_unique(69, 20, 20), Some(true));
        assert_eq!(grid.set_unique(13, 21, 21), Some(true));

        let mut many = grid.get_many_unique(10..=20, 10..=20);
        assert_eq!(many.len(), 3);

        many.sort();
        assert_eq!(many, [&10, &69, &420]);
    }
}
