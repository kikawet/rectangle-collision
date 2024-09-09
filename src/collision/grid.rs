use std::ops::RangeInclusive;

#[derive(Debug, PartialEq)]
struct Cell<'a, T>(Vec<&'a T>);
struct Grid<'a, T> {
    data: Vec<Cell<'a, T>>,
    rows: usize,
    cols: usize,
}

impl<'a, T> Grid<'a, T> {
    fn new(rows: usize, cols: usize) -> Self {
        let data = (0..rows * cols).map(|_| Cell::empty()).collect();

        Self { data, rows, cols }
    }

    fn get(&self, row: usize, col: usize) -> Option<&Cell<T>> {
        self.data.get(self.get_index(row, col))
    }

    fn get_index(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }
}

impl<'a, T: PartialEq> Grid<'a, T> {
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
            .fold(vec![], |mut acc, cur| {
                if !acc.contains(&cur) {
                    acc.push(cur);
                }
                acc
            })
    }

    fn set_unique(&mut self, item: &'a T, row: usize, col: usize) -> Option<bool> {
        let index = self.get_index(row, col);
        self.data
            .get_mut(index)
            .map(|cell| cell.insert_unique(item))
    }
}

impl<'a, T> Cell<'a, T> {
    fn empty() -> Self {
        Self(vec![])
    }

    fn single(item: &'a T) -> Self {
        Self(vec![item])
    }
}

impl<'a, T: PartialEq> Cell<'a, T> {
    fn insert_unique(&mut self, item: &'a T) -> bool {
        if !self.0.contains(&item) {
            self.0.push(item);
            return true;
        }

        false
    }
}

impl<'a, T> Default for Cell<'a, T> {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_grid() {
        let grid: Grid<i8> = Grid::new(10, 41);

        assert_eq!(grid.get(0, 0), Some(&Cell::empty()));
        assert_eq!(grid.get(9, 40), Some(&Cell::empty()));
        assert_eq!(grid.get(10, 40), None);
        assert_eq!(grid.get(9, 41), None);
    }

    #[test]
    fn test_set_unique() {
        let mut grid = Grid::new(50, 50);

        assert_eq!(grid.get(9, 15), Some(&Cell::empty()));
        assert_eq!(grid.set_unique(&69, 9, 15), Some(true));
        assert_eq!(grid.get(9, 15), Some(&Cell::single(&69)));

        assert_eq!(grid.set_unique(&69, 9, 15), Some(false));
        assert_eq!(grid.get(9, 15), Some(&Cell::single(&69)));

        assert_eq!(grid.set_unique(&420, 9, 15), Some(true));
        assert_eq!(grid.get(9, 15), Some(&Cell(vec![&69, &420])));
    }

    #[test]
    fn test_get_many() {
        let mut grid = Grid::new(50, 50);

        assert_eq!(grid.set_unique(&10, 10, 10), Some(true));
        assert_eq!(grid.set_unique(&10, 15, 15), Some(true));
        assert_eq!(grid.set_unique(&420, 15, 15), Some(true));
        assert_eq!(grid.set_unique(&69, 20, 20), Some(true));
        assert_eq!(grid.set_unique(&13, 21, 21), Some(true));

        let mut many = grid.get_many(10..=20, 10..=20);
        assert_eq!(many.len(), 3);

        many.sort();
        assert_eq!(many, [&10, &69, &420]);
    }
}
