use super::corner::Corner;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Cell {
    heights: (u32, u32, u32, u32),
}

impl From<(u32, u32, u32, u32)> for Cell {
    fn from(value: (u32, u32, u32, u32)) -> Self {
        Self { heights: value }
    }
}

impl Cell {
    pub fn get_height(&self, corner: Corner) -> u32 {
        match corner {
            Corner::TopLeft => self.heights.0,
            Corner::TopRight => self.heights.1,
            Corner::BottomLeft => self.heights.2,
            Corner::BottomRight => self.heights.3,
        }
    }
}
