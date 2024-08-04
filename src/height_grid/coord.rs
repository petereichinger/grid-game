use bevy::math::UVec2;

#[derive(Clone, Copy, Debug, Default, Eq)]
pub struct Coord(UVec2);

impl PartialEq<Coord> for (u32, u32) {
    fn eq(&self, other: &Coord) -> bool {
        self.0 == other.0.x && self.1 == other.0.y
    }
}

impl PartialEq<(u32, u32)> for Coord {
    fn eq(&self, other: &(u32, u32)) -> bool {
        self.0.x == other.0 && self.0.y == other.1
    }
}

impl PartialEq<Coord> for Coord {
    fn eq(&self, other: &Coord) -> bool {
        self.0.eq(&other.0)
    }
}

impl Into<UVec2> for Coord {
    fn into(self) -> UVec2 {
        self.0
    }
}

impl Into<Coord> for UVec2 {
    fn into(self) -> Coord {
        Coord(self)
    }
}
