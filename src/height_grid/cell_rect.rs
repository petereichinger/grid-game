use super::coord::Coord;

pub struct CellRect {
    min: Coord,
    max: Coord,
}

impl CellRect {
    pub fn new(min: Coord, max: Coord) -> Self {
        assert!(min.0 <= max.0);
        assert!(min.1 <= max.1);
        Self { min, max }
    }

    pub fn from_center(center: Coord, extents: Coord) -> Self {
        let bottom_left = (
            center.0.saturating_sub(extents.0),
            center.1.saturating_sub(extents.1),
        );
        let top_right = (
            center.0.saturating_add(extents.0 + 1),
            center.1.saturating_add(extents.1 + 1),
        );
        Self::new(bottom_left, top_right)
    }

    pub fn width(&self) -> u32 {
        self.max.0 - self.min.0
    }

    pub fn height(&self) -> u32 {
        self.max.1 - self.min.1
    }

    pub fn num_cells(&self) -> u32 {
        self.width() * self.height()
    }
}

pub struct CellRectIter {
    rect: CellRect,
    current: u32,
}

impl From<CellRect> for CellRectIter {
    fn from(value: CellRect) -> Self {
        Self {
            rect: value,
            current: 0,
        }
    }
}
impl Iterator for CellRectIter {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        if current >= self.rect.num_cells() {
            None
        } else {
            let width = self.rect.width();
            let y = self.rect.min.1 + current / width;
            let x = self.rect.min.0 + current % width;

            self.current += 1;
            Some((x, y))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let width = self.rect.width() as usize;
        let height = self.rect.height() as usize;

        let remaining = width * height - self.current as usize;

        (remaining, Some(remaining))
    }
}

#[cfg(test)]
mod tests {
    use super::{CellRect, CellRectIter};

    #[test]
    #[should_panic]
    fn wrong_x_order() {
        CellRect::new((1, 1), (0, 0));
    }

    #[test]
    #[should_panic]
    fn wrong_y_order() {
        CellRect::new((0, 1), (1, 0));
    }

    #[test]
    fn correct_order_works() {
        let CellRect {
            min: bottom_left,
            max: top_right,
        } = CellRect::new((0, 0), (1, 1));

        assert_eq!(bottom_left, (0, 0));
        assert_eq!(top_right, (1, 1));
    }

    #[test]
    fn from_center_works() {
        let CellRect {
            min: bottom_left,
            max: top_right,
        } = CellRect::from_center((3, 3), (1, 2));

        assert_eq!(bottom_left, (2, 1));
        assert_eq!(top_right, (5, 6));
    }

    #[test]
    fn from_center_clamps_at_0() {
        let CellRect {
            min: bottom_left,
            max: top_right,
        } = CellRect::from_center((1, 1), (3, 4));

        assert_eq!(bottom_left, (0, 0));
        assert_eq!(top_right, (5, 6));
    }

    #[test]
    fn from_center_clamps_at_u32_max() {
        let CellRect {
            min: bottom_left,
            max: top_right,
        } = CellRect::from_center((u32::MAX, u32::MAX - 1), (3, 4));

        assert_eq!(bottom_left, (u32::MAX - 3, u32::MAX - 5));
        assert_eq!(top_right, (u32::MAX, u32::MAX));
    }

    #[test]
    fn empty_iterator_works() {
        let iter: CellRectIter = CellRect::new((0, 0), (0, 0)).into();

        let cells: Vec<_> = iter.collect();

        assert_eq!(cells, vec![]);
    }

    #[test]
    fn single_cell_iterator_works() {
        let iter: CellRectIter = CellRect::new((0, 0), (1, 1)).into();

        let cells: Vec<_> = iter.collect();

        assert_eq!(cells, vec![(0, 0)]);
    }

    #[test]
    fn multi_cell_iterator_works() {
        let iter: CellRectIter = CellRect::new((0, 0), (3, 2)).into();

        let cells: Vec<_> = iter.collect();

        assert_eq!(cells, vec![(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1)]);
    }
}
