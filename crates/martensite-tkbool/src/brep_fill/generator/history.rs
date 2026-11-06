//! The end of `Perform`: SameParameter on the shell (through the re-shaper for an
//! immutable input), its closure and the new wires of the modified sections.
use super::BRepFillGenerator;
use super::section_pair::SectionEnds;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::brep_tools::ReShape;
use martensite_tkbrep::topods::{self, Shape, ShapeIterator, Wire};
use martensite_tkbrep::toptools::IndexedMapOfShape;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_lib::{same_parameter_shape, same_parameter_shape_with_reshape};

/// The default tolerance of `BRepLib::SameParameter`.
const SAME_PARAMETER_TOLERANCE: f64 = 1.0e-5;

impl BRepFillGenerator {
    /// SameParameter on the shell (in place for a mutable input, else on copies
    /// recorded with the edge copies in the re-shaper, which then rebuilds the shell),
    /// and the Closed flag of a shell between two punctual sections of a closed profile.
    pub(super) fn finish_shell(&mut self, ends: SectionEnds) -> Result<(), Failure> {
        // All the vertices of the shell are the vertices of the original sections.
        if self.my_mutable_input {
            same_parameter_shape(&self.my_shell, SAME_PARAMETER_TOLERANCE, false)?;
        } else {
            for (key, value) in self.my_old_new_shapes.items() {
                self.my_reshaper.replace(key, value)?;
            }
            same_parameter_shape_with_reshape(
                &self.my_shell,
                &self.my_reshaper,
                SAME_PARAMETER_TOLERANCE,
                false,
            )?;
            let rebuilt: Shape = self.my_reshaper.apply(&self.my_shell, ShapeEnum::Shape)?;
            self.my_shell = topods::shell(&rebuilt)?;
        }
        if ends.u_closed && ends.degen_first && ends.degen_last {
            self.my_shell.set_closed(true)?;
        }
        Ok(())
    }

    /// A new wire for each modified section: its edges replaced by their result
    /// shapes and the flags of the old wire, bound in the old-new map.
    pub(super) fn update_wire_history(
        &mut self,
        modif_wires: &IndexedMapOfShape,
    ) -> Result<(), Failure> {
        let builder = BRepBuilder::default();
        for index in 1..=modif_wires.extent() {
            let cur_wire: Shape = modif_wires.find_key(index)?.clone();
            let mut new_wire = Wire::default();
            builder.make_wire(&mut new_wire);
            let mut edges = ShapeIterator::new(&cur_wire, true, true);
            while edges.more() {
                let new_edge: Shape = self.result_shape(edges.value()?)?;
                builder.add(&mut new_wire, &new_edge)?;
                edges.next();
            }
            copy_flags(&cur_wire, &new_wire)?;
            self.my_old_new_shapes.bind(cur_wire, new_wire.into_shape());
        }
        Ok(())
    }
}

/// The Free, Modified, Checked, Orientable, Closed, Infinite and Convex flags of
/// `from` set on `to`.
fn copy_flags(from: &Shape, to: &Shape) -> Result<(), Failure> {
    to.set_free(from.free()?)?;
    to.set_modified(from.modified()?)?;
    to.set_checked(from.checked()?)?;
    to.set_orientable(from.orientable()?)?;
    to.set_closed(from.closed()?)?;
    to.set_infinite(from.infinite()?)?;
    to.set_convex(from.convex()?)
}
