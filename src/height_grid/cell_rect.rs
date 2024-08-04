use bevy::prelude::*;

pub struct CellRect {
    min: UVec2,
    max: UVec2,
}

impl CellRect {
    pub fn new(min: impl Into<UVec2>, max: impl Into<UVec2>) -> Self {
        let min = min.into();
        let max = max.into();
        assert!(min.x <= max.x);
        assert!(min.y <= max.y);
        Self { min, max }
    }

    pub fn from_center(center: impl Into<UVec2>, extents: impl Into<UVec2>) -> Self {
        let center = center.into();
        let extents = extents.into();
        let bottom_left = center.saturating_sub(extents);
        let top_right = center.saturating_add(extents).saturating_add(UVec2::ONE);
        Self::new(bottom_left, top_right)
    }

    pub fn width(&self) -> u32 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> u32 {
        self.max.y - self.min.y
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
    type Item = UVec2;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        if current >= self.rect.num_cells() {
            None
        } else {
            let width = self.rect.width();
            let y = self.rect.min.y + current / width;
            let x = self.rect.min.x + current % width;

            self.current += 1;
            Some((x, y).into())
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
        let CellRect { min, max } = CellRect::new((0, 0), (1, 1));

        assert_eq!(min, (0, 0).into());
        assert_eq!(max, (1, 1).into());
    }

    #[test]
    fn from_center_works() {
        let CellRect { min, max } = CellRect::from_center((3, 3), (1, 2));

        assert_eq!(min, (2, 1).into());
        assert_eq!(max, (5, 6).into());
    }

    #[test]
    fn from_center_clamps_at_0() {
        let CellRect {
            min: bottom_left,
            max: top_right,
        } = CellRect::from_center((1, 1), (3, 4));

        assert_eq!(bottom_left, (0, 0).into());
        assert_eq!(top_right, (5, 6).into());
    }

    #[test]
    fn from_center_clamps_at_u32_max() {
        let CellRect {
            min: bottom_left,
            max: top_right,
        } = CellRect::from_center((u32::MAX, u32::MAX - 1), (3, 4));

        assert_eq!(bottom_left, (u32::MAX - 3, u32::MAX - 5).into());
        assert_eq!(top_right, (u32::MAX, u32::MAX).into());
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

        assert_eq!(cells, vec![(0, 0).into()]);
    }

    #[test]
    fn multi_cell_iterator_works() {
        let iter: CellRectIter = CellRect::new((0, 0), (3, 2)).into();

        let cells: Vec<_> = iter.collect();

        assert_eq!(
            cells,
            vec![
                (0, 0).into(),
                (1, 0).into(),
                (2, 0).into(),
                (0, 1).into(),
                (1, 1).into(),
                (2, 1).into()
            ]
        );
    }
}
