use super::corner::Corner;

pub type Coord = (u32, u32);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    heights: (u32, u32, u32, u32),
}

impl From<(u32, u32, u32, u32)> for Cell {
    fn from(value: (u32, u32, u32, u32)) -> Self {
        Self { heights: value }
    }
}

impl Cell {
    pub fn set_height(&mut self, corner: Corner, height: u32) {
        let corner = match corner {
            Corner::TopLeft => &mut self.heights.0,
            Corner::TopRight => &mut self.heights.1,
            Corner::BottomLeft => &mut self.heights.2,
            Corner::BottomRight => &mut self.heights.3,
        };

        *corner = height;
    }
    pub fn get_height(&self, corner: Corner) -> u32 {
        match corner {
            Corner::TopLeft => self.heights.0,
            Corner::TopRight => self.heights.1,
            Corner::BottomLeft => self.heights.2,
            Corner::BottomRight => self.heights.3,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FlipAxis {
    Horizontal,
    Vertical,
    Diagonal,
}
pub trait FlipCorner {
    fn flip(&self, flip: FlipAxis) -> Option<(Coord, Corner)>;
}

impl FlipCorner for (Coord, Corner) {
    fn flip(&self, flip: FlipAxis) -> Option<(Coord, Corner)> {
        let &((x, y), corner) = self;
        use Corner::*;
        use FlipAxis::*;

        let new_corner = match flip {
            Horizontal => match corner {
                TopLeft => ((Some(x), y.checked_add(1)), BottomLeft),
                TopRight => ((Some(x), y.checked_add(1)), BottomRight),
                BottomLeft => ((Some(x), y.checked_sub(1)), TopLeft),
                BottomRight => ((Some(x), y.checked_sub(1)), TopRight),
            },
            Vertical => match corner {
                TopLeft => ((x.checked_sub(1), Some(y)), TopRight),
                TopRight => ((x.checked_add(1), Some(y)), TopLeft),
                BottomLeft => ((x.checked_sub(1), Some(y)), BottomRight),
                BottomRight => ((x.checked_add(1), Some(y)), BottomLeft),
            },
            Diagonal => match corner {
                TopLeft => ((x.checked_sub(1), y.checked_add(1)), BottomRight),
                TopRight => ((x.checked_add(1), y.checked_add(1)), BottomLeft),
                BottomLeft => ((x.checked_sub(1), y.checked_sub(1)), TopRight),
                BottomRight => ((x.checked_add(1), y.checked_sub(1)), TopLeft),
            },
        };

        match new_corner {
            ((Some(x), Some(y)), corner) => Some(((x, y), corner)),
            _ => None,
        }
    }
}
