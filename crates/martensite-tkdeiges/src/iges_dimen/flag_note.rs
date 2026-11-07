//! IgesDimenFlagNote.
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
use crate::iges_data::{IgesDataIgesEntity, iges_array1_item, iges_live};
use crate::iges_dimen::{IgesDimenGeneralNote, IgesDimenLeaderArrow};
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, exceptions};
use martensite_tkmath::gp::{Point3, Xyz};

iges_entity! {
/// defines FlagNote, Type `208` Form `0`
/// Is label information formatted in different ways
IgesDimenFlagNote, IgesDimenFlagNoteState, "IgesDimenFlagNote", parent IgesDataIgesEntity, {
the_lower_leftcorner: Xyz,
the_angle: f64,
the_note: Handle<IgesDimenGeneralNote>,
the_leaders: Handle<HArray1<Handle<IgesDimenLeaderArrow>>>,
}
}

impl IgesDimenFlagNote {
    iges_methods! {
    get angle = the_angle: f64;
    get note = the_note: Handle<IgesDimenGeneralNote>;
    }
    /// This method is used to set the fields of the class
    /// FlagNote
    /// - leftCorner  : Lower left corner of the Flag
    /// - anAngle     : Rotation angle in radians
    /// - aNote       : General Note Entity
    /// - someLeaders : Leader Entities
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        left_corner: Xyz,
        an_angle: f64,
        a_note: &Handle<IgesDimenGeneralNote>,
        some_leaders: &Handle<HArray1<Handle<IgesDimenLeaderArrow>>>,
    ) -> Result<(), Failure> {
        if !some_leaders.is_null() {
            if iges_array1_lower(some_leaders, "IgesDimenFlagNote::init")? != 1 {
                return Err(exceptions::dimension_mismatch("IgesDimenFlagNote : Init"));
            }
        }
        {
            let mut state = self.state.write();
            state.the_lower_leftcorner = left_corner;
            state.the_angle = an_angle;
            state.the_note = a_note.clone();
            state.the_leaders = some_leaders.clone();
        }
        self.init_type_and_form(208, 0);
        Ok(())
    }
    /// returns Lower Left coordinate of Flag as Pnt from package gp
    pub fn lower_left_corner(&self) -> Point3 {
        let mut lowerleft: Point3 = Point3::from(self.state.read().the_lower_leftcorner);
        lowerleft
    }
    /// returns Lower Left coordinate of Flag as Pnt from package gp
    /// after Transformation.
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn transformed_lower_left_corner(&self) -> Result<Point3, Failure> {
        let mut temp_xyz: Xyz = self.state.read().the_lower_leftcorner;
        if self.has_transf() {
            self.location()?.transforms(&mut temp_xyz);
        }
        Ok(Point3::from(temp_xyz))
    }
    /// returns number of Arrows (Leaders) or zero
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_leaders(&self) -> Result<i32, Failure> {
        Ok(if self.state.read().the_leaders.is_null() {
            0
        } else {
            iges_array1_length(
                &self.state.read().the_leaders,
                "IgesDimenFlagNote::nb_leaders",
            )?
        })
    }
    /// returns Leader Entity
    /// raises exception if Index at most 0 or Index &gt; NbLeaders()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn leader(&self, index: i32) -> Result<Handle<IgesDimenLeaderArrow>, Failure> {
        iges_array1_item(
            &self.state.read().the_leaders,
            index,
            "IgesDimenFlagNote::leader",
        )
    }
    /// returns Height computed by the formula :
    /// Height = 2 * CH   where CH is from theNote
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn height(&self) -> Result<f64, Failure> {
        Ok(2.0 * self.character_height()?)
    }
    /// returns the Character Height (from General Note)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn character_height(&self) -> Result<f64, Failure> {
        let mut max: f64 = iges_live(
            &self.state.read().the_note,
            "IgesDimenFlagNote::character_height",
        )?
        .box_height(1)?;
        let loop_bound = iges_live(
            &self.state.read().the_note,
            "IgesDimenFlagNote::character_height",
        )?
        .nb_strings()?;
        for i in 2..=loop_bound {
            if max
                < (iges_live(
                    &self.state.read().the_note,
                    "IgesDimenFlagNote::character_height",
                )?
                .box_height(i)?)
            {
                max = iges_live(
                    &self.state.read().the_note,
                    "IgesDimenFlagNote::character_height",
                )?
                .box_height(i)?;
            }
        }
        Ok(max)
    }
    /// returns Length computed by the formula :
    /// Length = TW + 0.4*CH  where CH is from theNote
    /// and TW is from theNote
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn length(&self) -> Result<f64, Failure> {
        Ok(self.text_width()? + (0.4 * self.character_height()?))
    }
    /// returns the Text Width (from General Note)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn text_width(&self) -> Result<f64, Failure> {
        let mut width: f64 = 0.0;
        let loop_bound = iges_live(&self.state.read().the_note, "IgesDimenFlagNote::text_width")?
            .nb_strings()?;
        for i in 1..=loop_bound {
            width += iges_live(&self.state.read().the_note, "IgesDimenFlagNote::text_width")?
                .box_width(i)?;
        }
        Ok(width)
    }
    /// returns TipLength computed by the formula :
    /// TipLength = 0.5 * H / tan 35(deg)  where H is Height()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn tip_length(&self) -> Result<f64, Failure> {
        Ok(0.5
            * (self.height()? / (martensite_cmath::tan((35.0 / 180.0) * (std::f64::consts::PI)))))
    }
}
