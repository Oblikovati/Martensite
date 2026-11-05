//! `BOPAlgoBuilder::NbShapes`: the counts of the shapes taking part in an operation, by type.
use super::BOPAlgoBuilder;
use super::work_estimates::source_work_counts;
use crate::bop_ds::DataStructure;
use martensite_tkernel::standard::Failure;

/// How many shapes of each type an operation treats (`BOPAlgoBuilder::NbShapes`), all zero when new.
/// `nb_vertices` is the number of same-domain bindings, as natively, not of source vertices.
///
/// ```
/// use martensite_tkbo::bop_algo::NbShapes;
/// let mut counts = NbShapes::new();
/// *counts.nb_faces_mut() = 6;
/// assert_eq!((counts.nb_faces(), counts.nb_edges()), (6, 0));
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NbShapes {
    counts: [i32; 8],
}

impl NbShapes {
    /// All counts zero (`NbShapes()`); `NbShapes::new().nb_solids() == 0`.
    pub fn new() -> Self {
        Self::default()
    }
}

/// One read and one write accessor of a slot, as the two native overloads of each count.
macro_rules! count_accessors {
    ($($slot:literal $read:ident $write:ident $native:literal;)+) => {
        impl NbShapes {
            $(
                #[doc = concat!("The count of `", $native, "`; `counts.", stringify!($read), "()`.")]
                pub fn $read(&self) -> i32 {
                    self.counts[$slot]
                }

                #[doc = concat!("The mutable count of `", $native, "`; `*counts.", stringify!($write), "() += 1`.")]
                pub fn $write(&mut self) -> &mut i32 {
                    &mut self.counts[$slot]
                }
            )+
        }
    };
}

count_accessors! {
    0 nb_vertices nb_vertices_mut "NbVertices";
    1 nb_edges nb_edges_mut "NbEdges";
    2 nb_wires nb_wires_mut "NbWires";
    3 nb_faces nb_faces_mut "NbFaces";
    4 nb_shells nb_shells_mut "NbShells";
    5 nb_solids nb_solids_mut "NbSolids";
    6 nb_compsolids nb_compsolids_mut "NbCompsolids";
    7 nb_compounds nb_compounds_mut "NbCompounds";
}

impl BOPAlgoBuilder {
    /// Counts the shapes of `structure` this operation treats (`getNbShapes`), taking the data
    /// structure of the run explicitly instead of the stored pointer; it changes nothing.
    /// Example: `let counts = builder.get_nb_shapes(filler.data_structure().unwrap())?;`.
    pub fn get_nb_shapes(&self, structure: &DataStructure) -> Result<NbShapes, Failure> {
        let mut counts = [0; 8];
        for (count, source) in counts.iter_mut().zip(source_work_counts(structure)?) {
            *count = i32::try_from(source).map_err(|_| {
                Failure::out_of_range(&format!(
                    "shape_count={source}; expected a value that fits i32"
                ))
            })?;
        }
        Ok(NbShapes { counts })
    }
}

#[cfg(test)]
#[path = "nb_shapes_contract.rs"]
mod contract;
