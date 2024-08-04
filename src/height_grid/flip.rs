use bevy::math::UVec2;

use super::corner::Corner;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FlipAxis {
    Horizontal,
    Vertical,
    Diagonal,
}

pub trait FlipCorner {
    fn flip(&self, flip: FlipAxis) -> Option<(UVec2, Corner)>;
}

impl FlipCorner for (UVec2, Corner) {
    fn flip(&self, flip: FlipAxis) -> Option<(UVec2, Corner)> {
        let &(UVec2 { x, y }, corner) = self;
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
            ((Some(x), Some(y)), corner) => Some((UVec2 { x, y }, corner)),
            _ => None,
        }
    }
}
