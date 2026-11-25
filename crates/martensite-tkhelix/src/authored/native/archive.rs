// SPDX-License-Identifier: GPL-2.0-only
//! No retained mutable native object survives this publication boundary.
use super::super::{
    HelixFailure, HelixNativeInvariant as Invariant, HelixStage,
    failure::{native, poll},
};
use martensite_tkbrep::{
    bintools::{self, WriteOptions},
    topods::{self, Shape, Wire},
};
use martensite_tkernel::{message::ProgressRange, storage::IoStream};
use martensite_tkgeombase::geom_tools::ConsoleOutput;
use std::sync::Arc;

pub(super) fn encode(
    wire: &Wire,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Vec<u8>, HelixFailure> {
    let stage = HelixStage::Publication;
    poll(cancelled, stage)?;
    let mut stream = IoStream::from_bytes(Vec::new());
    let options = WriteOptions {
        with_triangles: false,
        with_normals: false,
        ..WriteOptions::default()
    };
    native(
        bintools::write(wire, &mut stream, options, &ProgressRange::default()),
        stage,
    )?;
    let size = usize::try_from(stream.tellp())
        .map_err(|_| HelixFailure::Topology(Invariant::ArchiveLength))?;
    let mut bytes = vec![0; size];
    stream.seekg(0).read(&mut bytes);
    if stream.fail() {
        return Err(HelixFailure::Topology(Invariant::CompleteArchive));
    }
    poll(cancelled, stage)?;
    Ok(bytes)
}

pub(super) fn decode(
    bytes: &[u8],
    stage: HelixStage,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Wire, HelixFailure> {
    poll(cancelled, stage)?;
    let mut stream = IoStream::from_bytes(bytes.to_vec());
    let mut shape = Shape::default();
    native(
        bintools::read_with_console(
            &mut shape,
            &mut stream,
            &ProgressRange::default(),
            Arc::new(Quiet),
        ),
        stage,
    )?;
    if stream.fail() || shape.is_null() {
        return Err(HelixFailure::Topology(Invariant::CompleteArchive));
    }
    poll(cancelled, stage)?;
    native(topods::wire(&shape), stage)
}

struct Quiet;
impl ConsoleOutput for Quiet {
    fn write(&self, _bytes: &[u8]) {}
}
