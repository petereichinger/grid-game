pub struct HeightGrid {
    pub width: u32,
    pub depth: u32,
    pub heights: Vec<u32>,
}

impl HeightGrid {
    pub fn new(width: u32, depth: u32, heights: Vec<u32>) -> Self {
        assert!(width > 0);
        assert!(depth > 0);
        assert!(heights.len() > 0);
        assert_eq!(width as usize * depth as usize, heights.len());

        Self {
            width,
            depth,
            heights,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HeightGrid;

    #[test]
    #[should_panic]
    fn fail_on_wrong_height_size() {
        HeightGrid::new(4, 4, vec![]);
    }

    #[test]
    #[should_panic]
    fn fail_on_zero_width() {
        HeightGrid::new(0, 4, vec![1, 1, 1, 1]);
    }

    #[test]
    #[should_panic]
    fn fail_on_zero_depth() {
        HeightGrid::new(4, 0, vec![1, 1, 1, 1]);
    }

    #[test]
    #[should_panic]
    fn fail_on_empty_array() {
        HeightGrid::new(4, 4, vec![]);
    }

    #[test]
    fn works_on_flat_grid() {
        HeightGrid::new(4, 4, vec![0; 16]);
    }
}
