// SPDX-License-Identifier: GPL-2.0-only
//! Capture final section instances in the same scope as one unrephased ruled build.
use super::BRepOffsetAPIThruSections;
use martensite_tkbool::brep_fill::{BRepFillGenerator, ThruSectionErrorStatus};
use martensite_tkbrep::{
    topexp::map_shapes_of_type,
    topods::{self, Face, Shape, Solid, Wire},
    toptools::IndexedMapOfShape,
};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use std::{error::Error, fmt};

/// Why a corresponding pair did not produce a complete native receipt.
#[derive(Debug)]
#[non_exhaustive]
pub enum CorrespondingRuledPairFailure {
    /// The supplied precision must be finite and positive.
    InvalidPrecision(f64),
    /// The indicated section is open.
    OpenSection(usize),
    /// The two input closed wires must have the same positive edge count.
    IncompatibleEdgeCount([usize; 2]),
    /// The ordinary native builder did not complete.
    Incomplete(ThruSectionErrorStatus),
    /// The indicated section lacks a cap or a unique copied member.
    Binding {
        /// Section ordinal in the authored request.
        section: usize,
        /// Native member kind with an incomplete correspondence.
        kind: ShapeEnum,
    },
    /// A native construction or lookup error.
    Native(Box<Failure>),
}

impl fmt::Display for CorrespondingRuledPairFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPrecision(value) => write!(
                f,
                "ruled pair precision {value} must be finite and positive"
            ),
            Self::OpenSection(index) => write!(f, "ruled pair section {index} must be closed"),
            Self::IncompatibleEdgeCount(counts) => write!(
                f,
                "ruled pair edge counts {counts:?} must be positive and equal"
            ),
            Self::Incomplete(status) => {
                write!(f, "the native ruled pair did not complete ({status:?})")
            }
            Self::Binding { section, kind } => write!(
                f,
                "ruled pair section {section} needs unique retained {kind:?} bindings"
            ),
            Self::Native(source) => write!(f, "{source}"),
        }
    }
}

impl Error for CorrespondingRuledPairFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Native(source) => Some(source.as_ref()),
            _ => None,
        }
    }
}

/// A completed pair's retained cap and its exact original-to-result incidence.
#[derive(Debug)]
pub struct CorrespondingRuledSection {
    /// Actual cap of this section.
    pub cap: Face,
    /// Input and output edges. Shape identity, not index order, associates callers' keys.
    pub edges: Vec<(Shape, Shape)>,
    /// Input and output vertices.
    pub vertices: Vec<(Shape, Shape)>,
}

/// Native solid and same-generation binding for the first and last input sections.
#[derive(Debug)]
pub struct CorrespondingRuledPair {
    /// Actual native body.
    pub solid: Solid,
    /// First and last source sections in explicit input order.
    pub sections: [CorrespondingRuledSection; 2],
}

impl BRepOffsetAPIThruSections {
    /// Build the ordinary ruled solid without rephasing either corresponding closed wire.
    /// The caller keeps the wire models stable during the operation and supplies any
    /// required geometric correspondence and non-intersection proof separately.
    /// Example: `BRepOffsetAPIThruSections::corresponding_ruled_pair([&a, &b], 1e-6)?`.
    pub fn corresponding_ruled_pair(
        wires: [&Wire; 2],
        precision: f64,
    ) -> Result<CorrespondingRuledPair, CorrespondingRuledPairFailure> {
        admit(wires, precision)?;
        let mut loft = Self::new(true, true, precision);
        loft.check_compatibility(false);
        loft.set_mutable_input(false);
        for wire in wires {
            loft.add_wire(wire);
        }
        native(loft.build_with_construction_error(true))?;
        if !loft.is_done() {
            return Err(CorrespondingRuledPairFailure::Incomplete(loft.get_status()));
        }
        let solid = native(topods::solid(native(loft.shape())?))?;
        let generator = native(loft.generator())?;
        let sections = [
            section_images(&solid, generator, wires[0], loft.first_shape(), 0)?,
            section_images(&solid, generator, wires[1], loft.last_shape(), 1)?,
        ];
        Ok(CorrespondingRuledPair { solid, sections })
    }
}

fn admit(wires: [&Wire; 2], precision: f64) -> Result<(), CorrespondingRuledPairFailure> {
    use CorrespondingRuledPairFailure as Cause;
    if !precision.is_finite() || precision <= 0. {
        return Err(Cause::InvalidPrecision(precision));
    }
    for (index, wire) in wires.iter().enumerate() {
        if !native(wire.closed())? {
            return Err(Cause::OpenSection(index));
        }
    }
    let counts = wires.map(|wire| wire.nb_children() as usize);
    if counts[0] == 0 || counts[0] != counts[1] {
        return Err(Cause::IncompatibleEdgeCount(counts));
    }
    Ok(())
}

fn section_images(
    solid: &Solid,
    generator: &BRepFillGenerator,
    source: &Wire,
    cap: &Face,
    section: usize,
) -> Result<CorrespondingRuledSection, CorrespondingRuledPairFailure> {
    let mut faces = IndexedMapOfShape::default();
    native(map_shapes_of_type(solid, ShapeEnum::Face, &mut faces))?;
    let cap_index = faces.find_index(cap);
    if cap_index == 0 {
        return Err(CorrespondingRuledPairFailure::Binding {
            section,
            kind: ShapeEnum::Face,
        });
    }
    let cap = native(topods::face(native(faces.find_key(cap_index))?))?;
    let edges = members(generator, source, &cap, section, ShapeEnum::Edge)?;
    let vertices = members(generator, source, &cap, section, ShapeEnum::Vertex)?;
    Ok(CorrespondingRuledSection {
        cap,
        edges,
        vertices,
    })
}

fn members(
    generator: &BRepFillGenerator,
    source: &Wire,
    cap: &Face,
    section: usize,
    kind: ShapeEnum,
) -> Result<Vec<(Shape, Shape)>, CorrespondingRuledPairFailure> {
    let failure = || CorrespondingRuledPairFailure::Binding { section, kind };
    let mut inputs = IndexedMapOfShape::default();
    let mut actual = IndexedMapOfShape::default();
    let mut visited = IndexedMapOfShape::default();
    native(map_shapes_of_type(source, kind, &mut inputs))?;
    native(map_shapes_of_type(cap, kind, &mut actual))?;
    if inputs.extent() == 0 || inputs.extent() != actual.extent() {
        return Err(failure());
    }
    (1..=inputs.extent())
        .map(|index| {
            let input = native(inputs.find_key(index))?;
            let image = native(generator.result_shape(input))?;
            let index = actual.find_index(&image);
            if index == 0 || visited.contains(&image) {
                return Err(failure());
            }
            visited.add(image);
            Ok((input.clone(), native(actual.find_key(index))?.clone()))
        })
        .collect()
}

fn native<T>(value: Result<T, Failure>) -> Result<T, CorrespondingRuledPairFailure> {
    value.map_err(|source| CorrespondingRuledPairFailure::Native(Box::new(source)))
}
