//! A virtual adaptor exposing precisely the state Contap samples, including failures.
use super::geometry_records::common::Reader;
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, StandardTransient},
};
use martensite_tkg3d::adaptor3d::{Adaptor3dSurface, h_surface_tool::Adaptor3dSurfaceHandle};
use martensite_tkmath::geom_abs::SurfaceType;
struct ContourSamplingSurface {
    parent: StandardTransient,
    kind: SurfaceType,
    bounds: [f64; 4],
    counts: [i32; 6],
    failure_field: i32,
}
impl_transient!(ContourSamplingSurface, "ContourSamplingSurface", dyn Adaptor3dSurface, parent, interfaces: [dyn Adaptor3dSurface]);
impl ContourSamplingSurface {
    fn check_field(&self, field: i32) -> Result<(), Failure> {
        if self.failure_field == field {
            return Err(Failure::with_message(
                "Standard_DomainError",
                "contour fixture",
            ));
        }
        Ok(())
    }
    fn bound(&self, index: usize, field: i32) -> Result<f64, Failure> {
        self.check_field(field)?;
        Ok(self.bounds[index])
    }
    fn count(&self, index: usize, field: i32) -> Result<i32, Failure> {
        self.check_field(field)?;
        Ok(self.counts[index])
    }
}
impl Adaptor3dSurface for ContourSamplingSurface {
    fn first_u_parameter(&self) -> Result<f64, Failure> {
        self.bound(0, 1)
    }
    fn last_u_parameter(&self) -> Result<f64, Failure> {
        self.bound(2, 2)
    }
    fn first_v_parameter(&self) -> Result<f64, Failure> {
        self.bound(1, 3)
    }
    fn last_v_parameter(&self) -> Result<f64, Failure> {
        self.bound(3, 4)
    }
    fn get_type(&self) -> Result<SurfaceType, Failure> {
        self.check_field(5)?;
        Ok(self.kind)
    }
    fn nb_u_knots(&self) -> Result<i32, Failure> {
        self.count(0, 6)
    }
    fn u_degree(&self) -> Result<i32, Failure> {
        self.count(2, 7)
    }
    fn nb_v_knots(&self) -> Result<i32, Failure> {
        self.count(1, 8)
    }
    fn v_degree(&self) -> Result<i32, Failure> {
        self.count(3, 9)
    }
    fn nb_u_poles(&self) -> Result<i32, Failure> {
        self.count(4, 10)
    }
    fn nb_v_poles(&self) -> Result<i32, Failure> {
        self.count(5, 11)
    }
}
const CONTOUR_SURFACE_KINDS: [SurfaceType; 11] = [
    SurfaceType::Plane,
    SurfaceType::Cylinder,
    SurfaceType::Cone,
    SurfaceType::Sphere,
    SurfaceType::Torus,
    SurfaceType::BezierSurface,
    SurfaceType::BSplineSurface,
    SurfaceType::SurfaceOfRevolution,
    SurfaceType::SurfaceOfExtrusion,
    SurfaceType::OffsetSurface,
    SurfaceType::OtherSurface,
];

/// Read a virtual surface with original field values; `sampling_surface(reader)`.
pub fn sampling_surface(reader: &mut Reader) -> Adaptor3dSurfaceHandle {
    let kind = CONTOUR_SURFACE_KINDS[reader.integer() as usize];
    let bounds = std::array::from_fn(|_| reader.real());
    let counts = std::array::from_fn(|_| reader.integer());
    let failure_field = reader.integer();
    Handle::new(ContourSamplingSurface {
        parent: StandardTransient::default(),
        kind,
        bounds,
        counts,
        failure_field,
    })
    .cast()
}

#[cfg(test)]
mod tests {
    use super::{Reader, sampling_surface};
    use martensite_tkernel::standard::Failure;
    use martensite_tkg3d::adaptor3d::h_surface_tool::Adaptor3dSurfaceHandle;

    #[test]
    fn every_controlled_failure_field_keeps_the_original_fixture_diagnostic() {
        for field in 1..=11 {
            let fields = [
                0.,
                1.,
                2.,
                9.,
                18.,
                2.,
                2.,
                1.,
                1.,
                3.,
                4.,
                f64::from(field),
            ];
            let surface = sampling_surface(&mut Reader::new(&fields));
            let failure = read_field(&surface, field).unwrap_err();
            assert_eq!(failure.exception_type(), "Standard_DomainError");
            assert_eq!(failure.message_bytes(), b"contour fixture");
        }
    }

    fn read_field(surface: &Adaptor3dSurfaceHandle, field: i32) -> Result<f64, Failure> {
        match field {
            1 => surface.first_u_parameter(),
            2 => surface.last_u_parameter(),
            3 => surface.first_v_parameter(),
            4 => surface.last_v_parameter(),
            5 => surface.get_type().map(|_| 0.),
            6 => surface.nb_u_knots().map(f64::from),
            7 => surface.u_degree().map(f64::from),
            8 => surface.nb_v_knots().map(f64::from),
            9 => surface.v_degree().map(f64::from),
            10 => surface.nb_u_poles().map(f64::from),
            11 => surface.nb_v_poles().map(f64::from),
            _ => panic!("field {field}: expected controlled field1..=11"),
        }
    }
}
