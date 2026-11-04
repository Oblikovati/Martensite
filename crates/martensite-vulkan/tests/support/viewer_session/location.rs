//! Object locations set through the context
//! (`translate_location` is in objects.rs), and
//! removal from the context that keeps the name bound.
use super::*;
use martensite_tkmath::gp::{Transform3, Vector3};
use martensite_tkmath::toploc::Location;

impl ViewerSession {
    /// The location becomes that translation; then update.
    /// Example: `session.set_location("b", [0.5, 0., 0.])?`.
    pub fn set_location(&self, name: &str, translation: [f64; 3]) -> Result<(), Failure> {
        self.relocate(name, |_| translation_of(translation))
    }

    /// The current location with this translation part.
    /// Example: `session.set_location_translation("b", [500., 0., 0.])?`.
    pub fn set_location_translation(
        &self,
        name: &str,
        translation: [f64; 3],
    ) -> Result<(), Failure> {
        self.relocate(name, |mut local| {
            local.set_translation_part(vector(translation));
            local
        })
    }

    /// The current location with this rotation part.
    /// Example: `session.set_location_rotation("b", [1., 0., 0., 1.])?`.
    pub fn set_location_rotation(&self, name: &str, [x, y, z, w]: [f64; 4]) -> Result<(), Failure> {
        self.relocate(name, |mut local| {
            local.set_rotation_part(martensite_tkmath::gp::Quaternion::new(x, y, z, w));
            local
        })
    }

    /// The translation part as `%g %g %g` text.
    /// Example: `assert_eq!(session.location_text("b2")?, "0 1.80711 0");`.
    pub fn location_text(&self, name: &str) -> Result<String, Failure> {
        let object = self.located(name)?;
        let local = object
            .get()
            .map(|o| o.local_transformation())
            .unwrap_or_default();
        let part = local.translation_part();
        Ok([part.x(), part.y(), part.z()].map(printf_g).join(" "))
    }

    /// The full command result retains C `%g` formatting; `session.transformation_text("b")?`.
    pub fn transformation_text(&self, name: &str) -> Result<String, Failure> {
        let local = self.located(name)?.local_transformation();
        let point = local.translation_part();
        let rotation = local.rotation_quaternion();
        let location = [point.x(), point.y(), point.z()]
            .map(|value| martensite_tkernel::storage::format_general(value, 6))
            .join(" ");
        let quaternion = [rotation.x(), rotation.y(), rotation.z(), rotation.w()]
            .map(|value| martensite_tkernel::storage::format_general(value, 6))
            .join(" ");
        Ok(format!(
            "Location: {location}\nRotation: {quaternion}\nScale:    {}\n",
            martensite_tkernel::storage::format_general(local.scale_factor(), 6)
        ))
    }

    /// The rotation about the axis through
    /// `origin` along `axis`, applied after the current location.
    /// Example: `session.rotate_location("b", [1., -1., -1.], [0., 1., 0.], 90.)?`.
    pub fn rotate_location(
        &self,
        name: &str,
        origin: [f64; 3],
        axis: [f64; 3],
        degrees: f64,
    ) -> Result<(), Failure> {
        let [x, y, z] = origin;
        let direction = martensite_tkmath::gp::Direction3::new(axis[0], axis[1], axis[2])?;
        let line = martensite_tkmath::gp::Axis1::new(
            martensite_tkmath::gp::Point3::new(x, y, z),
            direction,
        );
        let mut rotation = Transform3::default();
        rotation.set_rotation(line, degrees * (std::f64::consts::PI / 180.0))?;
        self.relocate(name, |local| rotation * local)
    }

    /// The mirror in the plane through `origin` with
    /// normal `normal`, applied after the current location.
    /// Example: `session.mirror_location("b", [0., -0.5, 0.], [0., 1., 0.])?`.
    pub fn mirror_location(
        &self,
        name: &str,
        [x, y, z]: [f64; 3],
        [nx, ny, nz]: [f64; 3],
    ) -> Result<(), Failure> {
        let plane = martensite_tkmath::gp::Axis2::new(
            martensite_tkmath::gp::Point3::new(x, y, z),
            martensite_tkmath::gp::Direction3::new(nx, ny, nz)?,
        )?;
        let mut mirror = Transform3::default();
        mirror.set_mirror_plane(plane);
        self.relocate(name, |local| mirror * local)
    }

    /// The identity location; `session.reset_location("b")?`.
    pub fn reset_location(&self, name: &str) -> Result<(), Failure> {
        self.relocate(name, |_| Transform3::default())
    }

    /// Remove the object from the context, keeping its name bound.
    /// Example: `session.remove_from_context("b")?`.
    pub fn remove_from_context(&self, name: &str) -> Result<(), Failure> {
        let object = self.located(name)?.clone();
        self.with_update(|context| context.remove(&object, false))
    }

    fn relocate(
        &self,
        name: &str,
        location: impl FnOnce(Transform3) -> Transform3,
    ) -> Result<(), Failure> {
        let object = self.located(name)?.clone();
        let local = object
            .get()
            .map(|o| o.local_transformation())
            .unwrap_or_default();
        let trsf = location(local);
        self.with_update(|context| context.set_location(&object, &Location::new(trsf)))
    }
}

fn vector([x, y, z]: [f64; 3]) -> Vector3 {
    Vector3::new(x, y, z)
}

fn translation_of(translation: [f64; 3]) -> Transform3 {
    let mut trsf = Transform3::default();
    trsf.set_translation_part(vector(translation));
    trsf
}

/// C `printf("%g")`: six significant digits, trailing zeros dropped, exponent form outside
/// 1e-4..1e6; `printf_g(1.807106781) == "1.80711"`.
pub fn printf_g(value: f64) -> String {
    if value == 0. || !value.is_finite() {
        return if value == 0. {
            "0".into()
        } else {
            value.to_string()
        };
    }
    let exponent = format!("{value:.5e}")
        .split('e')
        .nth(1)
        .and_then(|e| e.parse::<i32>().ok())
        .unwrap_or(0);
    if !(-4..6).contains(&exponent) {
        let text = format!("{value:.5e}");
        let (mantissa, power) = text.split_once('e').unwrap_or((&text, "0"));
        let power: i32 = power.parse().unwrap_or(0);
        let sign = if power < 0 { '-' } else { '+' };
        return format!("{}e{sign}{:02}", trim_zeros(mantissa), power.abs());
    }
    let decimals = (5 - exponent).max(0) as usize;
    trim_zeros(&format!("{value:.decimals$}"))
}

fn trim_zeros(text: &str) -> String {
    if !text.contains('.') {
        return text.into();
    }
    text.trim_end_matches('0').trim_end_matches('.').into()
}

#[cfg(test)]
mod printf_g_tests {
    use super::printf_g;

    #[test]
    fn six_significant_digits_without_trailing_zeros() {
        assert_eq!(printf_g(1.807106781), "1.80711");
        assert_eq!(printf_g(0.), "0");
        assert_eq!(printf_g(46.), "46");
        assert_eq!(printf_g(-2.5), "-2.5");
        assert_eq!(printf_g(1234567.), "1.23457e+06");
        assert_eq!(printf_g(0.0000123), "1.23e-05");
        assert_eq!(printf_g(999999.7), "1e+06");
    }
}
