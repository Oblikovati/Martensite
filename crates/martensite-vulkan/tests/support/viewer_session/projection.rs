//! The active view's projection as a frame for hidden-line removal.
use super::*;
use martensite_tkmath::gp::{Axis2, Direction3, Point3};

impl ViewerSession {
    /// The frame the view projects along: origin and direction from the eye ray through the
    /// view centre, X direction towards the middle of the right border.
    /// Example: `let frame = session.projection_frame()?;`.
    pub fn projection_frame(&self) -> Result<Axis2, Failure> {
        let view = self.view().view_base();
        let (width, height) = self.source_window_size();
        let right = view.convert_with_proj(width, height / 2)?;
        let center = view.convert_with_proj(width / 2, height / 2)?;
        let mut frame = Axis2::default();
        frame.set_location(Point3::new(-center[0], -center[1], -center[2]));
        frame.set_direction(Direction3::new(-center[3], -center[4], -center[5])?)?;
        frame.set_x_direction(Direction3::new(
            right[0] - center[0],
            right[1] - center[1],
            right[2] - center[2],
        )?)?;
        Ok(frame)
    }
}
