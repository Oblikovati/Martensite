//! Rational Jacobian integrals and independent carrier equations, not builder observations.
use crate::acceptance::geometry::{
    assert_census, assert_incidence_and_pcurves, assert_point_with_limit, entity, vertex_point,
};
use martensite_cmath::{cos, sin};
use martensite_tkbrep::{brep_adaptor::BRepAdaptorSurface, topods};
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::{
    geom_abs::SurfaceType,
    gp::{Direction3, Plane3, Point3},
};
use martensite_tksketch::surface_sketch::*;
use martensite_tktopalgo::brep_gprop::{BRepGProp, VolumeGKFlags};

pub(super) struct Expected {
    pub volume: f64,
    pub areas: [f64; 6],
}
impl Expected {
    pub(super) fn outward() -> Self {
        Self {
            volume: 53.76,
            areas: [14.56, 21.28, 10.92, 12., 15.96, 12.],
        }
    }
    pub(super) fn inward() -> Self {
        Self {
            volume: 47.04,
            areas: [19.04, 12.32, 9.24, 12., 14.28, 12.],
        }
    }
    pub(super) fn angular_scale(mut self, scale: f64) -> Self {
        self.volume *= scale;
        for index in [0, 1, 2, 4] {
            self.areas[index] *= scale;
        }
        self
    }
    pub(super) fn linear_scale(mut self, scale: f64) -> Self {
        self.volume *= scale * scale * scale;
        self.areas = self.areas.map(|area| area * scale * scale);
        self
    }
}

pub(super) fn assert_cone(
    result: &NormalExtrusionResult,
    request: &ConeSector,
    depths: [f64; 2],
    expected: Expected,
) {
    assert!(result.validation().is_valid(), "{:?}", result.validation());
    assert_eq!(result.history().len(), 27);
    let body = result.export_brep().unwrap();
    assert_eq!(body.entities.len(), 27);
    assert_census(&body);
    super::bindings::assert_semantic_incidence(&body, request);
    assert_incidence_and_pcurves(&body, linear_limit(request));
    assert_corners(&body, request, depths);
    assert_faces(&body, request, depths, expected.areas);
    assert_volume(&body, request, depths, expected.volume);
    assert!(
        result.volume().encloses(expected.volume),
        "{:?} vs {}",
        result.volume(),
        expected.volume
    );
    assert!(
        result.volume().error_bound
            <= result.volume().value * result.options().volume_relative_error
    );
}

fn linear_limit(request: &ConeSector) -> f64 {
    1e-7 * (request.slant[1] / 12.).min(1.)
}
fn sign(request: &ConeSector) -> f64 {
    match request.support.normal {
        NormalSense::Outward => 1.,
        NormalSense::Inward => -1.,
    }
}
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn point(request: &ConeSector, slant: f64, phi: f64, depth: f64) -> Point3 {
    let q = sign(request);
    let radius = 0.6 * slant + q * 0.8 * depth;
    let axial = 0.8 * slant - q * 0.6 * depth;
    let angle = request.support.chart_origin + phi / 0.6;
    let transverse = cross(request.support.axis, request.support.radial);
    let xyz: [f64; 3] = std::array::from_fn(|i| {
        request.support.apex[i]
            + axial * request.support.axis[i]
            + radius * (cos(angle) * request.support.radial[i] + sin(angle) * transverse[i])
    });
    Point3::new(xyz[0], xyz[1], xyz[2])
}
fn assert_corners(body: &DetachedNormalBody, request: &ConeSector, depths: [f64; 2]) {
    let coordinates = [(0, 0), (1, 0), (1, 1), (0, 1)];
    for (index, (phi, slant)) in coordinates.into_iter().enumerate() {
        let source = HistorySource::Vertex(request.corners[index]);
        let first = HistoryKey {
            source,
            role: HistoryRole::FirstDepthVertex,
        };
        let last = HistoryKey {
            source,
            role: HistoryRole::LastDepthVertex,
        };
        let p0 = point(
            request,
            request.slant[slant],
            request.developed_angle[phi],
            depths[0],
        );
        let p1 = point(
            request,
            request.slant[slant],
            request.developed_angle[phi],
            depths[1],
        );
        assert_point_with_limit(vertex_point(body, first), p0, linear_limit(request));
        assert_point_with_limit(vertex_point(body, last), p1, linear_limit(request));
        let actual0 = vertex_point(body, first);
        let actual1 = vertex_point(body, last);
        assert_point_with_limit(
            Point3::new(
                actual1.x() - actual0.x(),
                actual1.y() - actual0.y(),
                actual1.z() - actual0.z(),
            ),
            Point3::new(p1.x() - p0.x(), p1.y() - p0.y(), p1.z() - p0.z()),
            linear_limit(request),
        );
    }
}

fn face_keys(request: &ConeSector) -> [HistoryKey; 6] {
    [
        HistoryKey {
            source: HistorySource::Region(request.region),
            role: HistoryRole::FirstDepthFace,
        },
        HistoryKey {
            source: HistorySource::Region(request.region),
            role: HistoryRole::LastDepthFace,
        },
        HistoryKey {
            source: HistorySource::Curve(request.boundaries[0]),
            role: HistoryRole::SideFace,
        },
        HistoryKey {
            source: HistorySource::Curve(request.boundaries[1]),
            role: HistoryRole::SideFace,
        },
        HistoryKey {
            source: HistorySource::Curve(request.boundaries[2]),
            role: HistoryRole::SideFace,
        },
        HistoryKey {
            source: HistorySource::Curve(request.boundaries[3]),
            role: HistoryRole::SideFace,
        },
    ]
}
fn assert_faces(
    body: &DetachedNormalBody,
    request: &ConeSector,
    depths: [f64; 2],
    areas: [f64; 6],
) {
    for (index, key) in face_keys(request).into_iter().enumerate() {
        let shape = entity(body, key);
        let face = topods::face(shape).unwrap();
        let adaptor = BRepAdaptorSurface::with_face(&face, true).unwrap();
        if matches!(index, 3 | 5) {
            assert_eq!(adaptor.get_type().unwrap(), SurfaceType::Plane);
        } else {
            assert_carrier(&adaptor, request, depths, index);
        }
        let mut props = GProps::new();
        let estimate = BRepGProp::surface_properties_eps(shape, &mut props, 1e-9, false).unwrap();
        assert!(
            estimate.is_finite() && (0. ..=1e-9).contains(&estimate),
            "{key:?}: {estimate}"
        );
        assert!(
            (props.mass() - areas[index]).abs() < areas[index] * 1e-8,
            "{key:?}: {} != {}",
            props.mass(),
            areas[index]
        );
    }
}
fn assert_carrier(
    adaptor: &BRepAdaptorSurface,
    request: &ConeSector,
    depths: [f64; 2],
    index: usize,
) {
    assert_eq!(adaptor.get_type().unwrap(), SurfaceType::Cone);
    let (offset, slope, half_sign) = if index < 2 {
        (-sign(request) * depths[index] / 0.6, 0.75, 1.)
    } else {
        (request.slant[usize::from(index == 4)] / 0.8, 4. / 3., -1.)
    };
    let apex: [f64; 3] =
        std::array::from_fn(|i| request.support.apex[i] + offset * request.support.axis[i]);
    let cone = adaptor.cone().unwrap();
    assert_point_with_limit(
        cone.apex(),
        Point3::new(apex[0], apex[1], apex[2]),
        linear_limit(request),
    );
    let direction = cone.axis().direction();
    let alignment = direction.x() * request.support.axis[0]
        + direction.y() * request.support.axis[1]
        + direction.z() * request.support.axis[2];
    assert!((alignment.abs() - 1.).abs() < 1e-10);
    assert!((sin(cone.semi_angle()).abs() / cos(cone.semi_angle()).abs() - slope).abs() < 1e-10);
    assert_half_cone(adaptor, request, apex, slope, half_sign);
}
fn assert_half_cone(
    adaptor: &BRepAdaptorSurface,
    request: &ConeSector,
    apex: [f64; 3],
    slope: f64,
    half_sign: f64,
) {
    let u = (adaptor.first_u_parameter().unwrap() + adaptor.last_u_parameter().unwrap()) / 2.;
    let v = (adaptor.first_v_parameter().unwrap() + adaptor.last_v_parameter().unwrap()) / 2.;
    let sample = adaptor.value(u, v).unwrap();
    let delta = [
        sample.x() - apex[0],
        sample.y() - apex[1],
        sample.z() - apex[2],
    ];
    let axial: f64 = (0..3).map(|i| delta[i] * request.support.axis[i]).sum();
    let radial_squared: f64 = (0..3)
        .map(|i| {
            let r = delta[i] - axial * request.support.axis[i];
            r * r
        })
        .sum();
    assert!(half_sign * axial > 0., "wrong carrier half: {sample:?}");
    assert!((radial_squared.sqrt() - axial.abs() * slope).abs() < linear_limit(request));
}
fn assert_volume(body: &DetachedNormalBody, request: &ConeSector, depths: [f64; 2], expected: f64) {
    let mut props = GProps::new();
    let estimate = BRepGProp::volume_properties_gk_plane(
        &body.solid,
        &mut props,
        alternate_reference(request, depths),
        1e-9,
        true,
        VolumeGKFlags::default(),
    )
    .unwrap();
    assert!(
        estimate.is_finite() && (0. ..=1e-9).contains(&estimate),
        "{estimate}"
    );
    assert!(props.mass() > 0.);
    assert!(
        (props.mass() - expected).abs() < expected * 1e-8,
        "{} != {expected}",
        props.mass()
    );
}
pub(super) fn assert_swapped_depth_vertices(
    out: &DetachedNormalBody,
    inward: &DetachedNormalBody,
    id: SurfaceVertexId,
) {
    let first = HistoryKey {
        source: HistorySource::Vertex(id),
        role: HistoryRole::FirstDepthVertex,
    };
    let last = HistoryKey {
        role: HistoryRole::LastDepthVertex,
        ..first
    };
    assert_point_with_limit(vertex_point(out, first), vertex_point(inward, last), 1e-7);
    assert_point_with_limit(vertex_point(out, last), vertex_point(inward, first), 1e-7);
}

// Use the opposite external axial plane from production. Rational integrals and
// per-face geometry checks remain the independent expected results (#417).
fn alternate_reference(request: &ConeSector, depths: [f64; 2]) -> Plane3 {
    let axial_max = request
        .slant
        .into_iter()
        .flat_map(|l| depths.map(|t| 0.8 * l - sign(request) * 0.6 * t))
        .fold(f64::NEG_INFINITY, f64::max);
    let offset = axial_max + 2. * (request.slant[1] + depths[0].abs().max(depths[1].abs()));
    let xyz: [f64; 3] =
        std::array::from_fn(|i| request.support.apex[i] + offset * request.support.axis[i]);
    Plane3::new(
        Point3::new(xyz[0], xyz[1], xyz[2]),
        Direction3::new(
            request.support.axis[0],
            request.support.axis[1],
            request.support.axis[2],
        )
        .unwrap(),
    )
    .unwrap()
}
