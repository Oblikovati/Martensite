//! IgesDimenGeneralSymbol.
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
use crate::iges_dimen::{IgesDimenGeneralNote, IgesDimenLeaderArrow};
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, exceptions};

iges_entity! {
/// defines General Symbol, Type `228`, Form &lt;0-3,5001-9999&gt;
/// Consists of zero or one (Form 0) or one (all other
/// forms), one or more geometry entities which define
/// a symbol, and zero, one or more associated leaders.
IgesDimenGeneralSymbol, IgesDimenGeneralSymbolState, "IgesDimenGeneralSymbol", parent IgesDataIgesEntity, {
the_note: Handle<IgesDimenGeneralNote>,
the_geoms: Handle<HArray1<Handle<IgesDataIgesEntity>>>,
the_leaders: Handle<HArray1<Handle<IgesDimenLeaderArrow>>>,
}
}

impl IgesDimenGeneralSymbol {
    iges_methods! {
    get note = the_note: Handle<IgesDimenGeneralNote>;
    }
    /// This method is used to set the fields of the class
    /// GeneralSymbol
    /// - aNote      : General Note, null for form 0
    /// - allGeoms   : Geometric Entities
    /// - allLeaders : Leader Arrows
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        a_note: &Handle<IgesDimenGeneralNote>,
        all_geoms: &Handle<HArray1<Handle<IgesDataIgesEntity>>>,
        all_leaders: &Handle<HArray1<Handle<IgesDimenLeaderArrow>>>,
    ) -> Result<(), Failure> {
        if (!all_geoms.is_null())
            && (iges_array1_lower(all_geoms, "IgesDimenGeneralSymbol::init")? != 1)
        {
            return Err(exceptions::dimension_mismatch(
                "IgesDimenGeneralSymbol : Init",
            ));
        }
        if !all_leaders.is_null() {
            if iges_array1_lower(all_leaders, "IgesDimenGeneralSymbol::init")? != 1 {
                return Err(exceptions::dimension_mismatch("$"));
            }
        }
        {
            let mut state = self.state.write();
            state.the_note = a_note.clone();
            state.the_geoms = all_geoms.clone();
            state.the_leaders = all_leaders.clone();
        }
        self.init_type_and_form(228, self.form_number());
        Ok(())
    }
    /// Changes FormNumber (indicates the Nature of the Symbol)
    /// Error if not in ranges \[0-3\] or \[&gt; 5000\]
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn set_form_number(&self, form: i32) -> Result<(), Failure> {
        if ((form < 0) || (form > 3)) && (form < 5000) {
            return Err(exceptions::out_of_range(
                "IgesDimenGeneralSymbol : SetFormNumber",
            ));
        }
        self.init_type_and_form(228, form);
        Ok(())
    }
    /// returns True if there is associated General Note Entity
    pub fn has_note(&self) -> bool {
        !(self.state.read().the_note.is_null())
    }
    /// returns number of Geometry Entities
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_geom_entities(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_geoms,
            "IgesDimenGeneralSymbol::nb_geom_entities",
        )
    }
    /// returns the Index'th Geometry Entity
    /// raises exception if Index at most 0 or Index &gt; NbGeomEntities()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn geom_entity(&self, index: i32) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        iges_array1_item(
            &self.state.read().the_geoms,
            index,
            "IgesDimenGeneralSymbol::geom_entity",
        )
    }
    /// returns number of Leaders or zero if not specified
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_leaders(&self) -> Result<i32, Failure> {
        Ok(if self.state.read().the_leaders.is_null() {
            0
        } else {
            iges_array1_length(
                &self.state.read().the_leaders,
                "IgesDimenGeneralSymbol::nb_leaders",
            )?
        })
    }
    /// returns the Index'th Leader Arrow
    /// raises exception if Index at most 0 or Index &gt; NbLeaders()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn leader_arrow(&self, index: i32) -> Result<Handle<IgesDimenLeaderArrow>, Failure> {
        iges_array1_item(
            &self.state.read().the_leaders,
            index,
            "IgesDimenGeneralSymbol::leader_arrow",
        )
    }
}
