// SPDX-License-Identifier: GPL-2.0-only
//! Circuit from a typed finite law through the actual retained native edge and immutable bytes.
mod archive;
mod verify;

use super::{
    ConstantHelix, HelixFailure, HelixQuantity, HelixStage, HelixTraceBounds, HelixTraceLimits,
    failure::{native, poll},
    fit,
    law::positive,
};
use martensite_tkbrep::topods::{Edge, Wire};
use martensite_tkg3d::geom::GeomCurve;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeWire};

const MAX_NATIVE_SEGMENTS: usize = 512;

/// Independent fit, retained topology and certification limits.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HelixFitOptions {
    /// Maximum effective binary64 turns admitted for native fitting.
    pub max_turns: f64,
    /// Requested native fit error, in model units.
    pub approximation_tolerance: f64,
    /// Maximum number of native approximation segments requested.
    pub max_segments: usize,
    /// Maximum calls to the authored evaluator inside native approximation.
    pub max_evaluations: u32,
    /// Maximum finite tolerance stored on the native edge and vertices, in model units.
    pub max_topology_tolerance: f64,
    /// One independently metered whole-interval correspondence proof and spatial cap.
    pub trace: HelixTraceLimits,
}

/// Measured and certified quantities for the actual detached native curve family.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HelixFitReceipt {
    /// Native fit estimate reported by AdvApprox. Independent trace proves the retained edge.
    pub native_estimate: f64,
    /// Number of authored evaluations performed by the native fitter.
    pub evaluations: u32,
    /// Effective native segment limit after intersecting fit and trace policies.
    pub effective_max_segments: usize,
    /// Certified actual source trace, axial progression and proof work.
    pub trace: HelixTraceBounds,
    /// Largest actual stored edge or endpoint tolerance in model units.
    pub stored_topology_tolerance: f64,
    /// Certified upper model-space L1 error at each oriented terminal vertex.
    pub terminal_error_upper: [f64; 2],
}

/// The retained native edge's relationship to its one authored span.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HelixSpanHistory {
    /// Actual active native radian range, in its authored forward direction.
    pub native_parameter: [f64; 2],
    /// Corresponding authored directed interval normalized by the effective turn count.
    pub source_fraction: [f64; 2],
}

/// One immutable checked fit whose native representation is held as private bytes.
#[derive(Debug)]
pub struct FittedHelix {
    law: ConstantHelix,
    options: HelixFitOptions,
    receipt: HelixFitReceipt,
    history: HelixSpanHistory,
    archive: Box<[u8]>,
}

/// Fresh native wire and its unique oriented edge carrying the proven source span.
#[derive(Debug)]
pub struct DetachedHelix {
    /// Native open wire owned by this export.
    pub wire: Wire,
    /// The actual retained edge occurrence beneath this wire.
    pub edge: Edge,
    /// Ordered correspondence with the authored finite span.
    pub history: HelixSpanHistory,
}

impl ConstantHelix {
    /// Construct and independently certify one native polynomial edge for this authored law.
    ///
    /// Cancellation is polled during analytic callbacks and certificate work, and around
    /// indivisible native building or archive calls. Use `law.fit(options, || false)?` when cancellation is unnecessary.
    pub fn fit(
        self,
        options: HelixFitOptions,
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> Result<FittedHelix, HelixFailure> {
        poll(&mut cancelled, HelixStage::Admission)?;
        let segments = options.admit(self)?;
        let curve = fit::build(self, options, segments, &mut cancelled)?;
        let history = self.history();
        let wire = assemble(&curve, history, &mut cancelled)?;
        let archive = archive::encode(&wire, &mut cancelled)?;
        let decoded = archive::decode(&archive, HelixStage::Verification, &mut cancelled)?;
        let edge = verify::edge(&decoded)?;
        let validated = verify::retained(self, &decoded, &edge, options, &mut cancelled)?;
        poll(&mut cancelled, HelixStage::Publication)?;
        Ok(FittedHelix {
            law: self,
            options,
            history,
            archive: archive.into_boxed_slice(),
            receipt: HelixFitReceipt {
                native_estimate: curve.estimate,
                evaluations: curve.evaluations,
                effective_max_segments: segments as usize,
                trace: validated.trace,
                stored_topology_tolerance: validated.stored_topology_tolerance,
                terminal_error_upper: validated.terminal_error_upper,
            },
        })
    }

    fn history(self) -> HelixSpanHistory {
        HelixSpanHistory {
            native_parameter: [0., self.resolved().parameter_end],
            source_fraction: [0., 1.],
        }
    }
}

impl HelixFitOptions {
    fn admit(self, law: ConstantHelix) -> Result<i32, HelixFailure> {
        positive(self.max_turns, HelixQuantity::Turns)?;
        positive(
            self.approximation_tolerance,
            HelixQuantity::ApproximationTolerance,
        )?;
        positive(
            self.max_topology_tolerance,
            HelixQuantity::TopologyTolerance,
        )?;
        let requested = law.resolved().turns;
        if requested > self.max_turns {
            return Err(HelixFailure::TurnLimit {
                requested,
                limit: self.max_turns,
            });
        }
        if !self.trace.max_position_error.is_finite() || self.trace.max_position_error < 0. {
            return Err(HelixFailure::InvalidScalar {
                quantity: HelixQuantity::PositionTolerance,
                value: self.trace.max_position_error,
            });
        }
        let work = self.max_segments.min(self.trace.max_native_spans);
        if !(1..=MAX_NATIVE_SEGMENTS).contains(&work) || self.max_evaluations == 0 {
            return Err(HelixFailure::InvalidWorkLimits {
                max_segments: work,
                max_evaluations: self.max_evaluations,
            });
        }
        Ok(work as i32)
    }
}

fn assemble(
    fit: &fit::Fit,
    history: HelixSpanHistory,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Wire, HelixFailure> {
    poll(cancelled, HelixStage::Assembly)?;
    let carrier = fit.curve.clone().into_upcast::<dyn GeomCurve>();
    let [first, last] = history.native_parameter;
    let edge = native(
        BRepBuilderAPIMakeEdge::from_curve_params(&carrier, first, last),
        HelixStage::Assembly,
    )?;
    let edge = native(edge.edge(), HelixStage::Assembly)?;
    let wire = native(
        BRepBuilderAPIMakeWire::from_edge(&edge),
        HelixStage::Assembly,
    )?;
    poll(cancelled, HelixStage::Assembly)?;
    native(wire.wire(), HelixStage::Assembly)
}

impl FittedHelix {
    /// Finite authored evaluator. Read its opening station with `checked.law().evaluate(0.)?`.
    pub fn law(&self) -> ConstantHelix {
        self.law
    }

    /// Original fit and validation policy, including `checked.options().max_turns`.
    pub fn options(&self) -> HelixFitOptions {
        self.options
    }

    /// Achieved error and work, including `checked.receipt().trace.position_error_upper`.
    pub fn receipt(&self) -> HelixFitReceipt {
        self.receipt
    }

    /// Source interval of the unique retained edge. Read it with `checked.history().source_fraction`.
    pub fn history(&self) -> HelixSpanHistory {
        self.history
    }

    /// Decode an entirely independent native wire and rebind its one retained edge.
    /// Use `checked.export_brep(|| false)?` when cancellation is unnecessary.
    pub fn export_brep(
        &self,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<DetachedHelix, HelixFailure> {
        let wire = archive::decode(&self.archive, HelixStage::Export, &mut cancelled)?;
        let edge = verify::edge(&wire)?;
        poll(&mut cancelled, HelixStage::Export)?;
        Ok(DetachedHelix {
            wire,
            edge,
            history: self.history,
        })
    }
}
