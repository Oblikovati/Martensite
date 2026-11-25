// SPDX-License-Identifier: GPL-2.0-only
//! Operation-local native wiring followed by validation of the archive readback.
mod build;
mod verify;

use super::{HelixSequence, HelixSequenceError as Failure, HelixSequenceSpan, stations::Form};
use crate::authored::{
    HelixFailure, HelixFitOptions, HelixStage, HelixTraceBounds, failure::poll, native::archive,
};
use martensite_tkbrep::topods::{Edge, Wire};

/// Native measurements and bounded evidence for one retained source span.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HelixSequenceSpanReceipt {
    /// Source identity and directed active trim with its ideal axial stations.
    pub span: HelixSequenceSpan,
    /// Original AdvApprox fit estimate before canonical endpoints were enforced.
    pub native_estimate: f64,
    /// Actual number of analytic evaluator calls.
    pub evaluations: u32,
    /// Proven actual retained polynomial correspondence and progress.
    pub trace: HelixTraceBounds,
    /// Maximum actual edge and vertex tolerance for this span.
    pub stored_topology_tolerance: f64,
}

/// Immutable detached-wire archive and ordered validated source history.
#[derive(Debug)]
pub struct FittedHelixSequence {
    options: HelixFitOptions,
    receipts: Box<[HelixSequenceSpanReceipt]>,
    archive: Box<[u8]>,
}

/// One fresh native wire and its directed ordered edge occurrences.
#[derive(Debug)]
pub struct DetachedHelixSequence {
    /// Independently owned open native wire.
    pub wire: Wire,
    /// Directed actual edge occurrences in source order.
    pub edges: Box<[Edge]>,
}

impl HelixSequence {
    /// Fit the ordered law with common native joins and certify the archived result.
    /// The flat-end native proof uses a cardinal normal and requires each retained flat
    /// to advance strictly along its chord. The analytic law remains available after a
    /// typed native refusal. Use `law.fit(options, || false)?` for an uncancellable operation.
    pub fn fit(
        &self,
        options: HelixFitOptions,
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> Result<FittedHelixSequence, Failure> {
        poll(&mut cancelled, HelixStage::Admission)?;
        let turns = self
            .spans
            .last()
            .and_then(|span| span.stations[1].turns.nearest_binary64())
            .ok_or(Failure::Helix(HelixFailure::Topology(
                crate::authored::HelixNativeInvariant::OrderedWire,
            )))?;
        let max_segments = options.admit_turns(turns)? as usize;
        let flat_axis = self.flat_axis()?;
        let candidate = build::assemble(self, options, max_segments, flat_axis, &mut cancelled)?;
        let bytes = archive::encode(&candidate.wire, &mut cancelled)?;
        let wire = archive::decode(&bytes, HelixStage::Verification, &mut cancelled)?;
        let receipts = verify::retained(self, &wire, options, &candidate.work, &mut cancelled)?;
        poll(&mut cancelled, HelixStage::Publication)?;
        Ok(FittedHelixSequence {
            options,
            receipts,
            archive: bytes.into_boxed_slice(),
        })
    }

    fn flat_axis(&self) -> Result<Option<usize>, Failure> {
        if self
            .spans
            .iter()
            .all(|span| matches!(span.form, Form::Constant))
        {
            return Ok(None);
        }
        let z = self.placement.frame.direction();
        let axis = z.coordinates().iter().position(|value| value.abs() == 1.);
        axis.filter(|index| {
            z.coordinates()
                .iter()
                .enumerate()
                .all(|(i, value)| i == *index || *value == 0.)
        })
        .map(Some)
        .ok_or(Failure::NativeFlatFrame)
    }
}

impl FittedHelixSequence {
    /// Original fit and shared operation-wide work limits.
    /// Use `checked.options().max_segments` to recover the requested segment cap.
    pub fn options(&self) -> HelixFitOptions {
        self.options
    }

    /// Actual read-back work, geometry and source evidence in authored order.
    /// Read the first source with `checked.receipts().first()`.
    pub fn receipts(&self) -> &[HelixSequenceSpanReceipt] {
        &self.receipts
    }

    /// Decode disjoint native ownership and rebind each ordered edge under the fresh root.
    /// Use `checked.export_brep(|| false)?` for an uncancellable readback.
    pub fn export_brep(
        &self,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<DetachedHelixSequence, Failure> {
        let wire = archive::decode(&self.archive, HelixStage::Export, &mut cancelled)?;
        let edges = verify::edges(&wire, self.receipts.len(), HelixStage::Export)?;
        poll(&mut cancelled, HelixStage::Export)?;
        Ok(DetachedHelixSequence {
            wire,
            edges: edges.into_boxed_slice(),
        })
    }
}
