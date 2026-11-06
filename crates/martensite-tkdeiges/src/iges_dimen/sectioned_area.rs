//! IgesDimenSectionedArea.
#![allow(
    unused_mut,
    unused_variables,
    unused_assignments,
    clippy::needless_late_init,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::manual_range_contains,
    clippy::too_many_arguments,
    clippy::collapsible_else_if,
    clippy::needless_return,
    reason = "the members keep the order and the locals of the record they read"
)]
use crate::iges_basic::entity_shape::{iges_array1_length, iges_array1_lower};
use crate::iges_data::{IgesDataIgesEntity, iges_array1_item};
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, exceptions};
use martensite_tkmath::gp::{Point3, Xyz};

iges_entity! {
/// defines IGES Sectioned Area, Type `230` Form `0`,
/// A sectioned area is a portion of a design which is to be
/// filled with a pattern of lines. Ordinarily, this entity
/// is used to reveal or expose shape or material characteri-
/// stics defined by other entities. It consists of a pointer
/// to an exterior definition curve, a specification of the
/// pattern of lines, the coordinates of a point on a pattern
/// line, the distance between the pattern lines, the angle
/// between the pattern lines and the X-axis of definition
/// space, and the specification of any enclosed definition
/// curves (commonly known as islands).
IgesDimenSectionedArea, IgesDimenSectionedAreaState, "IgesDimenSectionedArea", parent IgesDataIgesEntity, {
the_exterior_curve: Handle<IgesDataIgesEntity>,
the_pattern: i32,
the_passing_point: Xyz,
the_distance: f64,
the_angle: f64,
the_island_curves: Handle<HArray1<Handle<IgesDataIgesEntity>>>,
}
}

impl IgesDimenSectionedArea {
    iges_methods! {
    get exterior_curve = the_exterior_curve: Handle<IgesDataIgesEntity>;
    get pattern = the_pattern: i32;
    get distance = the_distance: f64;
    get angle = the_angle: f64;
    }
    /// `init`.
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        a_curve: &Handle<IgesDataIgesEntity>,
        a_pattern: i32,
        a_point: Xyz,
        a_distance: f64,
        an_angle: f64,
        some_islands: &Handle<HArray1<Handle<IgesDataIgesEntity>>>,
    ) -> Result<(), Failure> {
        if !some_islands.is_null() {
            if iges_array1_lower(some_islands, "IgesDimenSectionedArea::init")? != 1 {
                return Err(exceptions::dimension_mismatch(
                    "IgesDimenSectionedArea : Init",
                ));
            }
        }
        {
            let mut state = self.state.write();
            state.the_exterior_curve = a_curve.clone();
            state.the_pattern = a_pattern;
            state.the_passing_point = a_point;
            state.the_distance = a_distance;
            state.the_angle = an_angle;
            state.the_island_curves = some_islands.clone();
        }
        self.init_type_and_form(230, self.form_number());
        Ok(())
    }
    /// Sets the cross hatches to be inverted or not,
    /// according value of `mode` (corresponds to FormNumber)
    pub fn set_inverted(&self, mode: bool) {
        self.init_type_and_form(230, if mode { 1 } else { 0 });
    }
    /// Returns True if cross hatches as Inverted, else they are
    /// Standard (Inverted : Form=1, Standard : Form=0)
    pub fn is_inverted(&self) -> bool {
        self.form_number() != 0
    }
    /// returns point thru which line should pass
    pub fn passing_point(&self) -> Point3 {
        Point3::from(self.state.read().the_passing_point)
    }
    /// returns point thru which line should pass after Transformation
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn transformed_passing_point(&self) -> Result<Point3, Failure> {
        let mut tmp_xyz: Xyz = self.state.read().the_passing_point;
        if self.has_transf() {
            self.location()?.transforms(&mut tmp_xyz);
        }
        Ok(Point3::from(tmp_xyz))
    }
    /// returns the Z depth
    pub fn z_depth(&self) -> f64 {
        self.state.read().the_passing_point.z()
    }
    /// returns the number of island curves
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_islands(&self) -> Result<i32, Failure> {
        Ok(if self.state.read().the_island_curves.is_null() {
            0
        } else {
            iges_array1_length(
                &self.state.read().the_island_curves,
                "IgesDimenSectionedArea::nb_islands",
            )?
        })
    }
    /// returns the interior definition curves, returns Null Handle
    /// exception raised if Index at most 0 or Index &gt; NbIslands()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn island_curve(&self, num: i32) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        iges_array1_item(
            &self.state.read().the_island_curves,
            num,
            "IgesDimenSectionedArea::island_curve",
        )
    }
}
