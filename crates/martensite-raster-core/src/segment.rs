/// One segment in signed 16-bit raster coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PixelSegment {
    /// Start and end points, `[x, y]` each.
    pub points: [[i16; 2]; 2],
}

impl PixelSegment {
    /// Wrap coordinates into the signed 16-bit raster range.
    ///
    /// ```
    /// use martensite_raster_core::PixelSegment;
    /// assert_eq!(PixelSegment::new(1, 2, 70000, 4).points, [[1, 2], [4464, 4]]);
    /// ```
    pub fn new(x_start: i32, y_start: i32, x_end: i32, y_end: i32) -> Self {
        Self {
            points: [
                [x_start as i16, y_start as i16],
                [x_end as i16, y_end as i16],
            ],
        }
    }

    /// Read endpoint zero or one; return `None` for another index.
    ///
    /// ```
    /// use martensite_raster_core::PixelSegment;
    /// assert_eq!(PixelSegment::new(1, 2, 3, 4).point(1), Some([3, 4]));
    /// ```
    pub fn point(&self, index: usize) -> Option<[i16; 2]> {
        self.points.get(index).copied()
    }

    /// Borrow an endpoint for modification; `segment.point_mut(0)`.
    pub fn point_mut(&mut self, index: usize) -> Option<&mut [i16; 2]> {
        self.points.get_mut(index)
    }
}
