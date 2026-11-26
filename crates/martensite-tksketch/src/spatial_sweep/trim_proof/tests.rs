// SPDX-License-Identifier: GPL-2.0-only
//! Real native trim mutations preserve the carrier and previously bound occurrence key.
use super::{Face, PendingShell, snapshot};
use crate::spatial_sweep::{
    Prepared, SolidSweepHistory, SolidSweepOptions, SweepFailure, SweepHistory, SweepOptions,
    SweepPathPart,
    admission::Admission,
    geometry::trim_basis,
    prepare, sweep_rounded_solid,
    trim_proof::{Trig, certify, verify_lateral},
};
use martensite_tkbo::profile_extrusion::{DisplayTolerance, Resolution, ToleranceCap};
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool, EdgePCurve},
    topexp::Explorer,
    topods::{self, Edge},
};
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{Geom2dCircle, Geom2dGeometry};
use martensite_tkg3d::{
    geom::{GeomPlane, GeomRectangularTrimmedSurface, GeomSurface},
    top_abs::ShapeEnum,
};
use martensite_tkmath::gp::{Point3, Vector2};
use martensite_tksketch3d::{
    Edit, LineId, MinimumLength, PointId, PointRole, SketchLine, SketchPoint, Snapshot,
    wire::{DirectedLineUse, LineDirection, WireClosure, WireRecipe},
};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeFace, BRepBuilderAPIMakePolygon};
use std::num::NonZeroU64;

const LIMIT: f64 = 1e-8;

#[test]
fn actual_planar_band_with_the_same_carrier_and_key_refuses_a_shifted_whole_trim() {
    let (admission, shell) = fixture();
    let mut face = select(shell, 0, 0, false);
    verify(&admission, &face, false).unwrap();
    let (carrier, location) = BRepTool::surface_with_location(&face.shape).unwrap();
    let mut shifted = BRepBuilderAPIMakePolygon::new();
    let ends = [[0., -0.5], [4., -0.5], [4., 0.], [3.5, 0.5], [0., 0.5]];
    for [x, y] in ends {
        shifted.add_point(Point3::new(x + 0.25, y, -0.5)).unwrap();
    }
    shifted.close().unwrap();
    face.shape =
        BRepBuilderAPIMakeFace::from_surface_wire(&carrier, &shifted.wire().unwrap(), true)
            .unwrap()
            .face()
            .unwrap();
    let (same, after) = BRepTool::surface_with_location(&face.shape).unwrap();
    assert!(carrier == same);
    assert_eq!(location, after);
    let failure = verify(&admission, &face, false).unwrap_err();
    assert!(
        matches!(failure, SweepFailure::Trace { bound, limit: LIMIT, .. } if bound >= 0.25),
        "{failure:?}"
    );
}

#[test]
fn actual_revolved_corner_with_the_same_carrier_and_key_refuses_a_clipped_wire() {
    let (admission, shell) = fixture();
    let mut face = select(shell, 3, 1, true);
    verify(&admission, &face, true).unwrap();
    let (carrier, location) = BRepTool::surface_with_location(&face.shape).unwrap();
    let represented = carrier
        .downcast_ref::<GeomRectangularTrimmedSurface>()
        .unwrap();
    let (u0, u1, v0, v1) = represented.bounds();
    face.shape =
        BRepBuilderAPIMakeFace::from_surface_bounds(&carrier, (u0, u1 * 0.5, v0, v1), 1e-6)
            .unwrap()
            .face()
            .unwrap();
    let (same, after) = BRepTool::surface_with_location(&face.shape).unwrap();
    assert!(carrier == same);
    assert_eq!(location, after);
    assert_eq!(represented.bounds(), (u0, u1, v0, v1));
    assert!(matches!(verify(&admission, &face, true),
        Err(SweepFailure::FaceTrim { expected, .. })
            if expected == "the actual four directed, exact UV iso sides of the revolution rectangle"));
}

fn verify(admission: &Admission, face: &Face, revolved: bool) -> Result<f64, SweepFailure> {
    verify_lateral(
        admission,
        face,
        revolved,
        0,
        LIMIT,
        &mut Trig::new(),
        &mut || false,
    )
}

fn select(shell: PendingShell, side: usize, station: usize, corner: bool) -> Face {
    shell
        .faces
        .into_iter()
        .find(|face| face.profile == side && face.station == station && face.corner == corner)
        .unwrap()
}

fn recipes() -> (WireRecipe, WireRecipe) {
    let route = wire(
        [[0., 0., 0.], [4., 0., 0.], [4., 4., 0.], [4., 4., 4.]],
        WireClosure::Open,
    );
    let profile = wire(
        [
            [0., -0.5, -0.5],
            [0., 0.5, -0.5],
            [0., 0.5, 0.5],
            [0., -0.5, 0.5],
        ],
        WireClosure::Closed,
    );
    (route, profile)
}

fn options() -> SweepOptions {
    SweepOptions {
        resolution: Resolution::new(
            1e-6,
            1e-6,
            1e-6,
            DisplayTolerance {
                chordal: 1e-3,
                angular: 0.1,
            },
            ToleranceCap::Refuse(1e-5),
        ),
        boundary_tolerance: 1e-6,
        source_trace_limit: LIMIT,
        approximation_limit: 1e-6,
    }
}

fn fixture() -> (Admission, PendingShell) {
    let (route, profile) = recipes();
    let options = options();
    let Prepared {
        admission,
        wires,
        mut pipe,
        shell,
        terminals: [a, b],
    } = prepare(&route, &profile, options, &mut || false).unwrap();
    let pending = snapshot(&mut pipe, &shell, &wires, [&a, &b], &admission, &mut || {
        false
    })
    .unwrap();
    (admission, pending)
}

#[test]
fn a_rounded_pcurve_offset_charges_the_actual_common_edge_closure() {
    let (spine, profile) = recipes();
    let options = SolidSweepOptions {
        sweep: options(),
        volume_error_limit: 1e-6,
    };
    let result = sweep_rounded_solid(&spine, &profile, options, || false).unwrap();
    let native = result.export_brep(|| false).unwrap();
    let baseline = certify(&native.images, [0.; 3], options, &mut || false).unwrap();
    let (face, edge, active) = rounded_seam(&native);
    shift_planar_circle(&face, &edge, active, 2e-10);
    let perturbed = certify(&native.images, [0.; 3], options, &mut || false).unwrap();
    assert!(baseline.closure > 0. && baseline.closure < 1e-11);
    assert!(perturbed.closure > 1e-10 && perturbed.closure <= perturbed.measured.error_bound);
    assert!(perturbed.trace > 1e-10 && perturbed.trace < LIMIT);
    assert!(
        perturbed
            .measured
            .encloses(11.5 + std::f64::consts::PI / 8.)
    );
    let strict = SolidSweepOptions {
        volume_error_limit: perturbed.closure * 0.5,
        ..options
    };
    assert!(
        certify(
            &result.export_brep(|| false).unwrap().images,
            [0.; 3],
            strict,
            &mut || false
        )
        .is_ok()
    );
    assert!(
        matches!(certify(&native.images, [0.; 3], strict, &mut || false),
        Err(SweepFailure::VolumeErrorBound { bound, limit }) if bound >= perturbed.closure && limit < perturbed.closure)
    );
}

fn rounded_seam(
    native: &crate::spatial_sweep::DetachedSolidSweep,
) -> (topods::Face, Edge, EdgePCurve) {
    native
        .images
        .iter()
        .find_map(|image| {
            if !matches!(&image.history, SolidSweepHistory::Lateral(SweepHistory::Face(source))
            if matches!(source.path, SweepPathPart::Corner { .. }))
            {
                return None;
            }
            let face = topods::face(&image.shape).unwrap();
            let surface = BRepTool::surface(&face).unwrap();
            surface.downcast_ref::<GeomPlane>()?;
            let mut arcs = Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
            Iterator::find_map(&mut arcs, |edge| {
                let edge = topods::edge(&edge).unwrap();
                let active = trim_basis(BRepTool::curve_on_face(&edge, &face).unwrap())?;
                active.pcurve.downcast_ref::<Geom2dCircle>()?;
                Some((face.clone(), edge, active))
            })
        })
        .unwrap()
}

fn shift_planar_circle(face: &topods::Face, edge: &Edge, active: EdgePCurve, shift: f64) {
    let circle = active
        .pcurve
        .downcast_ref::<Geom2dCircle>()
        .unwrap()
        .circ2d()
        .unwrap();
    let adjusted = Geom2dCircle::from_circ2d(circle);
    adjusted.translate(Vector2::new(shift, 0.)).unwrap();
    let handle = Handle::new(adjusted).into_upcast();
    let builder = BRepBuilder::default();
    builder
        .update_edge_pcurve_on_face(edge, &handle, face, 1e-7)
        .unwrap();
    builder
        .range_on_face(edge, face, active.first, active.last)
        .unwrap();
}

fn wire(values: [[f64; 3]; 4], closure: WireClosure) -> WireRecipe {
    let nz = |id| NonZeroU64::new(id).unwrap();
    let point = |i| PointId::new(nz(1 + i as u64));
    let ends = if closure == WireClosure::Open { 3 } else { 4 };
    let points = values.into_iter().enumerate().map(|(i, [x, y, z])| {
        Edit::InsertPoint(SketchPoint {
            id: point(i),
            position: Point3::new(x, y, z),
            role: PointRole::Defining,
        })
    });
    let uses = (0..ends)
        .map(|i| DirectedLineUse {
            line: LineId::new(nz(10 + i as u64)),
            direction: LineDirection::Forward,
        })
        .collect::<Vec<_>>();
    let edges = uses.iter().enumerate().map(|(i, source)| {
        Edit::InsertLine(SketchLine {
            id: source.line,
            endpoints: [point(i), point((i + 1) % 4)],
        })
    });
    let source = Snapshot::empty(MinimumLength::new(1e-6).unwrap())
        .apply(&points.chain(edges).collect::<Vec<_>>(), || false)
        .unwrap();
    source.wire_recipe(&uses, closure, 6, || false).unwrap()
}
