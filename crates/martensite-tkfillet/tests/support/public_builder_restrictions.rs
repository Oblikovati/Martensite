//! Owned, external-consumer arguments for the original non-implemented overloads.
use martensite_tkbrep::brep_adaptor::{BRepAdaptorCurve2d, BRepAdaptorSurface};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkfillet::brep_blend::WalkingDomain;
use martensite_tkfillet::chfi_ds::{ElSpine, Spine, SpineHandle, SurfData, SurfDataHandle, shared};
use martensite_tkfillet::chfi3d::{
    BlendSide, ChFi3dChBuilder, RstCall, RstRef, SurfOut, WalkSettings,
};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::math::Vector;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use martensite_tktopalgo::brep_top_adaptor::BRepTopAdaptorTopolTool;
use std::rc::Rc;
use std::sync::Arc;

type Simulation = fn(
    &mut ChFi3dChBuilder,
    &mut SurfDataHandle,
    &RstCall<'_>,
    &mut SurfOut,
    &mut [bool; 2],
) -> Result<(), Failure>;
type Performance = fn(
    &mut ChFi3dChBuilder,
    &mut Vec<SurfDataHandle>,
    &RstCall<'_>,
    &mut SurfOut,
    &mut [bool; 2],
) -> Result<(), Failure>;

struct RestrictionArguments {
    guide: Handle<ElSpine>,
    spine: SpineHandle,
    surface: Handle<BRepAdaptorSurface>,
    domain: WalkingDomain,
    curve: Handle<BRepAdaptorCurve2d>,
    solution: Vector<'static>,
}

impl RestrictionArguments {
    fn new() -> Self {
        Self {
            guide: Handle::new(ElSpine::new()),
            spine: shared(Spine::new()),
            surface: Handle::default(),
            domain: Rc::new(BRepTopAdaptorTopolTool::new()),
            curve: Handle::default(),
            solution: Vector::filled(1, 1, 7.).unwrap(),
        }
    }

    fn side(&self) -> BlendSide<'_> {
        let rst = RstRef {
            pc: &self.curve,
            sref: &self.surface,
            pcref: &self.curve,
        };
        BlendSide {
            s: &self.surface,
            i: &self.domain,
            orientation: Orientation::Forward,
            rst: Some(rst),
        }
    }

    fn call(&self) -> RstCall<'_> {
        RstCall {
            guide: &self.guide,
            spine: &self.spine,
            choix: 1,
            side1: self.side(),
            side2: self.side(),
            walk: walking_settings(),
            rec: [true; 4],
            soldep: &self.solution,
        }
    }
}

fn unit_box() -> martensite_tkbrep::topods::Shape {
    BRepPrimAPIMakeBox::from_sizes(1., 1., 1.)
        .unwrap()
        .shape()
        .unwrap()
        .clone()
}

fn chamfer() -> ChFi3dChBuilder {
    let solid = unit_box();
    ChFi3dChBuilder::new(&solid, 0.01).unwrap()
}

fn unchanged_output(out: SurfOut, detached: [bool; 2]) {
    assert_eq!(
        out,
        SurfOut {
            first: 13.25,
            last: 13.25,
            intf: 0,
            intl: 0
        }
    );
    assert_eq!(detached, [true; 2]);
}

fn walking_settings() -> WalkSettings {
    WalkSettings {
        max_step: 0.125,
        fleche: 0.125,
        tol_guide: 0.125,
        inside: true,
        appro: true,
        forward: true,
    }
}

fn output_arguments() -> (SurfOut, [bool; 2]) {
    let out = SurfOut {
        first: 13.25,
        last: 13.25,
        intf: 0,
        intl: 0,
    };
    (out, [true; 2])
}

fn failure_observation(
    label: &str,
    failure: Failure,
    out: SurfOut,
    detached: [bool; 2],
    identity: bool,
) {
    let expected = expected_failure(label);
    assert_eq!(failure.message(), expected);
    assert_eq!(failure.exception_type(), "Standard_Failure");
    assert!(identity, "{label}: expected original surface-data identity");
    unchanged_output(out, detached);
    print_failure_observation(label, failure.message(), out, detached, identity);
}

fn expected_failure(label: &str) -> &'static str {
    if label.starts_with("simul") {
        return "SimulSurf Not Implemented";
    }
    "PerformSurf Not Implemented"
}

fn print_failure_observation(
    label: &str,
    message: &str,
    out: SurfOut,
    detached: [bool; 2],
    identity: bool,
) {
    let identity_flag = u8::from(identity);
    println!(
        "{label}|{message}|{identity_flag}|{}|{}|{}|{}",
        out.first,
        out.last,
        u8::from(detached[0]),
        u8::from(detached[1])
    );
}

/// Exercise one actual public simulation default; `assert_simulation(method, "simul_surf_rst1")`.
pub(super) fn assert_simulation(method: Simulation, label: &str) {
    let arguments = RestrictionArguments::new();
    let mut builder = chamfer();
    let mut data = shared(SurfData::new());
    let before = Arc::clone(&data);
    let (mut out, mut detached) = output_arguments();
    let call = arguments.call();
    let failure = method(&mut builder, &mut data, &call, &mut out, &mut detached).unwrap_err();
    failure_observation(label, failure, out, detached, Arc::ptr_eq(&data, &before));
}

/// Exercise one actual public performance default; `assert_performance(method, "perform_surf_rst1")`.
pub(super) fn assert_performance(method: Performance, label: &str) {
    let arguments = RestrictionArguments::new();
    let mut builder = chamfer();
    let data = shared(SurfData::new());
    let mut sequence = vec![Arc::clone(&data)];
    let (mut out, mut detached) = output_arguments();
    let call = arguments.call();
    let failure = method(&mut builder, &mut sequence, &call, &mut out, &mut detached).unwrap_err();
    assert_eq!(sequence.len(), 1);
    failure_observation(
        label,
        failure,
        out,
        detached,
        Arc::ptr_eq(&sequence[0], &data),
    );
}

/// Execute the real dynamic computation failure; `assert_computation("There are no suitable edges for chamfer or fillet")`.
pub(super) fn assert_computation(expected: &str) {
    let mut builder = chamfer();
    let failure = martensite_tkfillet::chfi3d::compute_builder(&mut builder).unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_Failure");
    assert_eq!(failure.message(), expected);
    println!("compute_builder|{}", failure.message());
}

/// Observe the actual owned base lifetime; `assert_eq!(builder_lifetime_observation(), (true, true))`.
pub(super) fn builder_lifetime_observation() -> (bool, bool) {
    let solid = unit_box();
    let before = solid.tshape().ref_count();
    let builder = ChFi3dChBuilder::new(&solid, 0.01).unwrap();
    let retained = solid.tshape().ref_count() > before;
    drop(builder);
    let released = solid.tshape().ref_count() == before;
    assert!(std::mem::needs_drop::<
        martensite_tkfillet::chfi3d::ChFi3dBuilder,
    >());
    assert!(std::mem::needs_drop::<
        martensite_tkfillet::brep_blend::BRepBlendAppFuncRoot,
    >());
    println!(
        "builder_lifetime|{}|{}",
        u8::from(retained),
        u8::from(released)
    );
    (retained, released)
}
