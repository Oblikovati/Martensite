//! One owned frame constructor for independently typed retained field readers.
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::{Axis2, Axis3, Direction3, Point3};

pub(super) fn from_fields(
    origin: Point3,
    normal: Direction3,
    x_direction: Direction3,
    reverse_y: impl FnOnce() -> bool,
) -> Result<Axis3, Failure> {
    let mut frame = Axis3::from_axis2(Axis2::with_x_direction(origin, normal, x_direction)?);
    // The original reader consumes the flag only after frame construction succeeds.
    if reverse_y() {
        frame.y_reverse();
    }
    Ok(frame)
}

#[cfg(test)]
mod tests {
    use super::from_fields;
    use martensite_tkmath::gp::{Direction3, Point3};
    use std::cell::Cell;

    #[test]
    fn invalid_frame_does_not_consume_reversal_field() {
        let consumed = Cell::new(false);
        let result = from_fields(Point3::default(), Direction3::Z, Direction3::Z, || {
            consumed.set(true);
            false
        });
        assert!(result.is_err());
        assert!(!consumed.get());
    }

    #[test]
    fn valid_frame_consumes_reversal_once_and_retains_other_axes() {
        let consumed = Cell::new(0);
        let frame = from_fields(Point3::default(), Direction3::Z, Direction3::X, || {
            consumed.set(consumed.get() + 1);
            true
        })
        .unwrap();
        assert_eq!(consumed.get(), 1);
        assert_eq!(frame.x_direction().x(), 1.);
        assert_eq!(frame.y_direction().y(), -1.);
        assert_eq!(frame.direction().z(), 1.);
    }
}
