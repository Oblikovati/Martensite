use super::*;
use crate::brep_check::{BRepCheckAnalyzer, BRepCheckStatus};
use martensite_tkbrep::brep::{BRepBuilder, BRepCurve3D};
use martensite_tkbrep::topods::Wire;
use martensite_tkernel::ncollection::Array1;
use martensite_tkg3d::geom::{GeomCurve, GeomCylindricalSurface};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::gp::{Axis3, Point2, Point3, Vector3};
use martensite_tkmath::predicates::roots::metric_bounds::certify_cylindrical_trim;
use martensite_tkmath::toploc::Location;

const LIMIT: f64 = 1e-7;

fn proof(z: f64) -> CertifiedCylindricalTrim {
    certify_cylindrical_trim(
        CylindricalTrimQuery {
            curve: MetricBezier {
                poles: &[[2., 0., z], [2., 0., 1. + z]],
                weights: Some(&[1., 1.]),
            },
            pcurve: &[[0., z], [0., 1. + z]],
            radius: 2.,
            spatial_limit: LIMIT,
            options: Default::default(),
        },
        || false,
    )
    .unwrap()
}

struct Curves {
    rail: Handle<dyn GeomCurve>,
    cylinder: Handle<dyn GeomSurface>,
    trace: Handle<dyn Geom2dCurve>,
}

impl Curves {
    fn new(z: f64, radial_shift: f64, uv_shift: f64) -> Self {
        let rail = [
            Point3::new(2. + radial_shift, 0., z),
            Point3::new(2. + radial_shift, 0., 1. + z),
        ];
        let trace = [Point2::new(uv_shift, z), Point2::new(uv_shift, 1. + z)];
        Self {
            rail: Handle::new(GeomBezierCurve::new(&Array1::from_shared(&rail, 1)).unwrap())
                .into_upcast(),
            cylinder: Handle::new(GeomCylindricalSurface::new(Axis3::default(), 2.).unwrap())
                .into_upcast(),
            trace: Handle::new(Geom2dBezierCurve::new(&Array1::from_shared(&trace, 1)).unwrap())
                .into_upcast(),
        }
    }

    fn reuse_at(&self, proof: &CertifiedCylindricalTrim, limit: f64, tz: f64) -> bool {
        let mut transform = Transform3::default();
        transform.set_translation(Vector3::new(0., 0., tz));
        let reference = BRepCurve3D::new(&self.rail, &Location::new(transform));
        let rail = self
            .rail
            .get()
            .unwrap()
            .transformed(transform)
            .unwrap()
            .cast();
        let surface = self
            .cylinder
            .get()
            .unwrap()
            .transformed(transform)
            .unwrap()
            .cast();
        let loaded_reference =
            Handle::new(GeomAdaptorCurve::with_curve(&rail).unwrap()).into_upcast();
        CylinderMetricSource {
            reference: &reference,
            loaded_reference: &loaded_reference,
            surface: &self.cylinder,
            moved_surface: &surface,
            trace: &self.trace,
            reference_transform: transform,
            surface_transform: transform,
        }
        .matches(proof, limit)
    }

    fn wall_use(&self) -> (Edge, Face) {
        let builder = BRepBuilder::default();
        let (mut edge, mut wire, mut face) = (Edge::default(), Wire::default(), Face::default());
        builder
            .make_face_on_surface(&mut face, &self.cylinder, LIMIT)
            .unwrap();
        builder
            .make_edge_with_curve(&mut edge, &self.rail, LIMIT)
            .unwrap();
        builder
            .update_edge_pcurve_on_face(&edge, &self.trace, &face, LIMIT)
            .unwrap();
        builder.range(&edge, 0., 1., false).unwrap();
        builder.make_wire(&mut wire);
        builder.add(&mut wire, &edge).unwrap();
        builder.add(&mut face, &wire).unwrap();
        (edge, face)
    }
}

#[test]
fn rebinds_actual_curve_trace_and_loaded_tolerance() {
    let proof = proof(0.);
    let controls = Curves::new(0., 0., 0.);
    assert!(controls.reuse_at(&proof, LIMIT, 1.));
    assert!(!controls.reuse_at(&proof, proof.spatial_residual_bound(), 1.));
    assert!(!Curves::new(0., 1e-3, 0.).reuse_at(&proof, LIMIT, 1.));
    assert!(!Curves::new(0., 0., 1e-3).reuse_at(&proof, LIMIT, 1.));
    controls
        .cylinder
        .downcast::<GeomCylindricalSurface>()
        .set_radius(2.001)
        .unwrap();
    assert!(!controls.reuse_at(&proof, LIMIT, 1.));
}

#[test]
fn reflected_chart_rebinds_only_the_actual_height_on_the_cylinder() {
    let proof = proof(0.);
    let mut controls = Curves::new(0., 0., 0.);
    let mut axes = Axis3::default();
    axes.z_reverse();
    controls.cylinder = Handle::new(GeomCylindricalSurface::new(axes, 2.).unwrap()).into_upcast();
    assert!(!controls.reuse_at(&proof, LIMIT, 1.));
    let trace = [Point2::new(0., -0.), Point2::new(0., -1.)];
    controls.trace =
        Handle::new(Geom2dBezierCurve::new(&Array1::from_shared(&trace, 1)).unwrap()).into_upcast();
    assert!(controls.reuse_at(&proof, LIMIT, 1.));
}

#[test]
fn refuses_when_translation_rounding_consumes_loaded_margin() {
    let z = 0.001;
    let proof = proof(z);
    let controls = Curves::new(z, 0., 0.);
    assert!(controls.reuse_at(&proof, LIMIT, 1.));
    let translation = martensite_cmath::powi(2., 50);
    assert!(!controls.reuse_at(&proof, LIMIT, translation));
    assert!(controls.reuse_at(&proof, 0.01, translation));
}

#[test]
fn analyzer_ignores_foreign_native_use_and_falls_back_on_changed_control() {
    let proof = Arc::new(proof(0.));
    let (edge, face) = Curves::new(0., 0.01, 0.).wall_use();
    let mut bounds = CylindricalCurveBounds::default();
    bounds.insert(edge.clone(), face.clone(), Arc::clone(&proof));
    assert!(
        bounds
            .find(&edge.oriented(Orientation::Reversed), &face)
            .is_some()
    );
    assert!(bounds.find(&edge, &Face::default()).is_none());
    let ordinary = BRepCheckAnalyzer::new(&face, true, false, true).unwrap();
    let reused = BRepCheckAnalyzer::with_cylindrical_curve_bounds(&face, &bounds).unwrap();
    let status = |analyzer: BRepCheckAnalyzer| {
        analyzer
            .result(&edge)
            .unwrap()
            .get()
            .unwrap()
            .status_on_shape_of(&face)
            .unwrap()
            .iter()
            .copied()
            .collect::<Vec<_>>()
    };
    assert_eq!(status(ordinary), status(reused));
    let reused = BRepCheckAnalyzer::with_cylindrical_curve_bounds(&face, &bounds).unwrap();
    assert!(status(reused).contains(&BRepCheckStatus::InvalidCurveOnSurface));
}
