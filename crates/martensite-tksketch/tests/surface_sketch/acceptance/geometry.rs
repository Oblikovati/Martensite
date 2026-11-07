//! Checks decoded geometry through semantic bindings, never traversal-derived identity.
use martensite_cmath::{cos, powi, sin};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::geom_abs::SurfaceType;
use martensite_tkmath::gp::Point3;
use martensite_tksketch::surface_sketch::*;
use martensite_tktopalgo::brep_gprop::{BRepGProp, VolumeGKFlags};

pub(super) fn entity(body: &DetachedNormalBody, key: HistoryKey) -> &Shape {
    let matches: Vec<_> = body
        .entities
        .iter()
        .filter(|(candidate, _)| *candidate == key)
        .collect();
    assert_eq!(matches.len(), 1, "missing/duplicate binding {key:?}");
    &matches[0].1
}

pub(super) fn vertex_point(body: &DetachedNormalBody, key: HistoryKey) -> Point3 {
    BRepTool::pnt(&topods::vertex(entity(body, key)).unwrap()).unwrap()
}

pub(super) fn assert_point(actual: Point3, expected: Point3) {
    assert_point_with_limit(actual, expected, 1e-6);
}

fn assert_point_with_limit(actual: Point3, expected: Point3, limit: f64) {
    let squared = powi(actual.x() - expected.x(), 2)
        + powi(actual.y() - expected.y(), 2)
        + powi(actual.z() - expected.z(), 2);
    assert!(
        squared < limit * limit,
        "actual {actual:?}, expected {expected:?}"
    );
}

pub(super) fn assert_body(
    result: &NormalExtrusionResult,
    region: &CylinderRectangle,
    depths: [f64; 2],
) {
    assert!(result.validation().is_valid(), "{:?}", result.validation());
    let body = result.export_brep().unwrap();
    assert_eq!(body.entities.len(), 27);
    assert_eq!(result.history().len(), 27);
    assert_census(&body);
    assert_vertices(&body, region, depths);
    assert_faces(&body, region, depths);
    assert_incidence_and_pcurves(&body, point_limit(region));
    let expected = assert_volume(&body, region, depths);
    assert!(
        result.volume().encloses(expected),
        "{:?} vs {expected}",
        result.volume()
    );
}

fn assert_census(body: &DetachedNormalBody) {
    for (kind, count) in [
        (ShapeEnum::Face, 6),
        (ShapeEnum::Edge, 12),
        (ShapeEnum::Vertex, 8),
    ] {
        let mut map = martensite_tkbrep::toptools::IndexedMapOfShape::default();
        martensite_tkbrep::topexp::map_shapes_of_type(&body.solid, kind, &mut map).unwrap();
        assert_eq!(map.extent(), count);
    }
}

fn radius(region: &CylinderRectangle, depth: f64) -> f64 {
    match region.support.normal {
        NormalSense::Outward => region.support.radius + depth,
        NormalSense::Inward => region.support.radius - depth,
    }
}

fn expected_point(region: &CylinderRectangle, s: f64, z: f64, depth: f64) -> Point3 {
    let support = &region.support;
    let a = support.axis;
    let e = support.radial;
    let cross = [
        a[1] * e[2] - a[2] * e[1],
        a[2] * e[0] - a[0] * e[2],
        a[0] * e[1] - a[1] * e[0],
    ];
    let angle = support.chart_origin + s / support.radius;
    let r = radius(region, depth);
    let xyz: [f64; 3] = std::array::from_fn(|i| {
        support.origin[i] + z * a[i] + r * (cos(angle) * e[i] + sin(angle) * cross[i])
    });
    Point3::new(xyz[0], xyz[1], xyz[2])
}

fn assert_vertices(body: &DetachedNormalBody, region: &CylinderRectangle, depths: [f64; 2]) {
    let corners = [
        (region.s[0], region.z[0]),
        (region.s[1], region.z[0]),
        (region.s[1], region.z[1]),
        (region.s[0], region.z[1]),
    ];
    for (index, (s, z)) in corners.into_iter().enumerate() {
        for (depth, role) in [
            (depths[0], HistoryRole::FirstDepthVertex),
            (depths[1], HistoryRole::LastDepthVertex),
        ] {
            let key = HistoryKey {
                source: HistorySource::Vertex(region.corners[index]),
                role,
            };
            assert_point_with_limit(
                vertex_point(body, key),
                expected_point(region, s, z, depth),
                point_limit(region),
            );
        }
    }
}

fn point_limit(region: &CylinderRectangle) -> f64 {
    1e-6 * (region.support.radius / 10.).min(1.)
}

fn assert_faces(body: &DetachedNormalBody, region: &CylinderRectangle, depths: [f64; 2]) {
    let angle = (region.s[1] - region.s[0]) / region.support.radius;
    let height = region.z[1] - region.z[0];
    let radii = [radius(region, depths[0]), radius(region, depths[1])];
    for (r, role) in [
        (radii[0], HistoryRole::FirstDepthFace),
        (radii[1], HistoryRole::LastDepthFace),
    ] {
        let key = HistoryKey {
            source: HistorySource::Region(region.region),
            role,
        };
        assert_face(
            body,
            key,
            SurfaceType::Cylinder,
            r * angle * height,
            Some(r),
        );
    }
    let radial_width = (radii[1] - radii[0]).abs();
    for (index, curve) in region.boundaries.iter().enumerate() {
        let area = if index.is_multiple_of(2) {
            angle * radial_width * (radii[0] + radii[1]) / 2.
        } else {
            height * radial_width
        };
        let key = HistoryKey {
            source: HistorySource::Curve(*curve),
            role: HistoryRole::SideFace,
        };
        assert_face(body, key, SurfaceType::Plane, area, None);
    }
}

fn assert_face(
    body: &DetachedNormalBody,
    key: HistoryKey,
    kind: SurfaceType,
    area: f64,
    radius: Option<f64>,
) {
    let shape = entity(body, key);
    let face = topods::face(shape).unwrap();
    let adaptor = BRepAdaptorSurface::with_face(&face, true).unwrap();
    assert_eq!(adaptor.get_type().unwrap(), kind, "{key:?}");
    if let Some(radius) = radius {
        assert!((adaptor.cylinder().unwrap().radius() - radius).abs() < (radius * 1e-8).min(1e-7));
    }
    let mut properties = GProps::new();
    let error = BRepGProp::surface_properties_eps(shape, &mut properties, 1e-9, false).unwrap();
    assert!(
        error.is_finite() && (0. ..=1e-9).contains(&error),
        "{key:?}: {error}"
    );
    assert!(
        (properties.mass() - area).abs() < (area * 1e-8).min(1e-6),
        "{key:?}: {} != {area}",
        properties.mass()
    );
}

fn assert_volume(body: &DetachedNormalBody, region: &CylinderRectangle, depths: [f64; 2]) -> f64 {
    let r0 = radius(region, depths[0]);
    let r1 = radius(region, depths[1]);
    let angle = (region.s[1] - region.s[0]) / region.support.radius;
    let expected = angle * (region.z[1] - region.z[0]) * (r1 - r0).abs() * (r1 + r0) / 2.;
    let mut properties = GProps::new();
    let error = BRepGProp::volume_properties_gk(
        &body.solid,
        &mut properties,
        1e-9,
        true,
        VolumeGKFlags::default(),
    )
    .unwrap();
    assert!(
        error.is_finite() && (0. ..=1e-9).contains(&error),
        "quadrature error {error}"
    );
    assert!(properties.mass() > 0.);
    assert!(
        (properties.mass() - expected).abs() < (expected * 1e-8).min(1e-6),
        "{} != {expected}",
        properties.mass()
    );
    expected
}

fn assert_incidence_and_pcurves(body: &DetachedNormalBody, point_limit: f64) {
    for (_, edge) in body
        .entities
        .iter()
        .filter(|(_, s)| s.shape_type().unwrap() == ShapeEnum::Edge)
    {
        let mut uses = Vec::new();
        for (_, face) in body
            .entities
            .iter()
            .filter(|(_, s)| s.shape_type().unwrap() == ShapeEnum::Face)
        {
            let mut explorer = Explorer::new(face, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
            while explorer.more() {
                if explorer.current().is_same(edge) {
                    uses.push(explorer.current().orientation());
                    assert_pcurve(explorer.current(), face, point_limit);
                }
                explorer.next().unwrap();
            }
        }
        assert_eq!(uses.len(), 2);
        assert!(uses.contains(&Orientation::Forward) && uses.contains(&Orientation::Reversed));
    }
}

fn assert_pcurve(edge: &Shape, face: &Shape, point_limit: f64) {
    let edge = topods::edge(edge).unwrap();
    let face = topods::face(face).unwrap();
    let pcurve = BRepTool::curve_on_face(&edge, &face).unwrap();
    let (curve, first, last) = BRepTool::curve(&edge).unwrap();
    let adaptor = BRepAdaptorSurface::with_face(&face, true).unwrap();
    assert!((pcurve.first - first).abs() < 1e-7 && (pcurve.last - last).abs() < 1e-7);
    for fraction in [0., 0.5, 1.] {
        let parameter = first + fraction * (last - first);
        let uv = pcurve.pcurve.get().unwrap().value(parameter).unwrap();
        let on_surface = adaptor.value(uv.x(), uv.y()).unwrap();
        assert_point_with_limit(
            on_surface,
            curve.get().unwrap().value(parameter).unwrap(),
            point_limit,
        );
    }
}
