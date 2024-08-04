use super::corner::Corner;

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
