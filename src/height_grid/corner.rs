#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum Corner {
    #[default]
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

pub const CORNERS: [Corner; 4] = [
    Corner::TopLeft,
    Corner::TopRight,
    Corner::BottomRight,
    Corner::BottomLeft,
];

impl Corner {
    pub fn get_corner_offset(&self) -> (f32, f32) {
        match self {
            Corner::TopLeft => (0.0, 1.0),
            Corner::TopRight => (1.0, 1.0),
            Corner::BottomLeft => (0.0, 0.0),
            Corner::BottomRight => (1.0, 0.0),
        }
    }
}
