//! IgesGraphTextFontDef.
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
use crate::iges_basic::IgesBasicHArray1OfHArray1OfInteger;
use crate::iges_basic::entity_shape::{iges_array1_length, iges_array1_lower};
use crate::iges_data::{IgesDataIgesEntity, iges_array1_item, iges_live};
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, exceptions};
use martensite_tkernel::tcollection::HAsciiString;

iges_entity! {
/// defines IGES Text Font Definition Entity, Type `310`
///
/// Used to define the appearance of characters in a text font.
/// It may be used to describe a complete font or a
/// modification to a subset of characters in another font.
IgesGraphTextFontDef, IgesGraphTextFontDefState, "IgesGraphTextFontDef", parent IgesDataIgesEntity, {
the_font_code: i32,
the_font_name: Handle<HAsciiString>,
the_superseded_font_code: i32,
the_superseded_font_entity: Handle<IgesGraphTextFontDef>,
the_scale: i32,
the_ascii_codes: Handle<HArray1<i32>>,
the_next_char_origin_x: Handle<HArray1<i32>>,
the_next_char_origin_y: Handle<HArray1<i32>>,
the_nb_pen_motions: Handle<HArray1<i32>>,
the_pen_motions: Handle<IgesBasicHArray1OfHArray1OfInteger>,
the_pen_moves_to_x: Handle<IgesBasicHArray1OfHArray1OfInteger>,
the_pen_moves_to_y: Handle<IgesBasicHArray1OfHArray1OfInteger>,
}
}

impl IgesGraphTextFontDef {
    iges_methods! {
    get font_code = the_font_code: i32;
    get font_name = the_font_name: Handle<HAsciiString>;
    get superseded_font_code = the_superseded_font_code: i32;
    get superseded_font_entity = the_superseded_font_entity: Handle<IgesGraphTextFontDef>;
    get scale = the_scale: i32;
    }
    /// This method is used to set the fields of the class
    /// TextFontDef
    /// - aFontCode         : Font Code
    /// - aFontName         : Font Name
    /// - aSupersededFont   : Number of superseded font
    /// - aSupersededEntity : Text Definition Entity
    /// - aScale            : No. of grid units = 1 text height unit
    /// - allASCIICodes     : ASCII codes for characters
    /// - allNextCharX & Y  : Grid locations of the next
    ///   character's origin (Integer vals)
    /// - allPenMotions     : No. of pen motions for the characters
    /// - allPenFlags       : Pen up/down flags,
    ///   0 = Down (default), 1 = Up
    /// - allMovePenToX & Y : Grid locations the pen will move to
    ///   This method initializes the fields of the class TextFontDef.
    ///   An exception is raised if the lengths of allASCIICodes,
    ///   allNextChars, allPenMotions, allPenFlags and allMovePenTo
    ///   are not same.
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        a_font_code: i32,
        a_font_name: &Handle<HAsciiString>,
        a_superseded_font: i32,
        a_superseded_entity: &Handle<IgesGraphTextFontDef>,
        a_scale: i32,
        all_ascii_codes: &Handle<HArray1<i32>>,
        all_next_char_x: &Handle<HArray1<i32>>,
        all_next_char_y: &Handle<HArray1<i32>>,
        all_pen_motions: &Handle<HArray1<i32>>,
        all_pen_flags: &Handle<IgesBasicHArray1OfHArray1OfInteger>,
        all_move_pen_to_x: &Handle<IgesBasicHArray1OfHArray1OfInteger>,
        all_move_pen_to_y: &Handle<IgesBasicHArray1OfHArray1OfInteger>,
    ) -> Result<(), Failure> {
        let mut len: i32 = iges_array1_length(all_ascii_codes, "IgesGraphTextFontDef::init")?;
        if ((((((iges_array1_lower(all_ascii_codes, "IgesGraphTextFontDef::init")? != 1)
            || ((iges_array1_lower(all_next_char_x, "IgesGraphTextFontDef::init")? != 1)
                || (iges_array1_length(all_next_char_x, "IgesGraphTextFontDef::init")? != len)))
            || ((iges_array1_lower(all_next_char_y, "IgesGraphTextFontDef::init")? != 1)
                || (iges_array1_length(all_next_char_y, "IgesGraphTextFontDef::init")? != len)))
            || ((iges_array1_lower(all_pen_motions, "IgesGraphTextFontDef::init")? != 1)
                || (iges_array1_length(all_pen_motions, "IgesGraphTextFontDef::init")? != len)))
            || (((iges_live(all_pen_flags, "IgesGraphTextFontDef::init")?.lower()) != 1)
                || ((iges_live(all_pen_flags, "IgesGraphTextFontDef::init")?.length()) != len)))
            || (((iges_live(all_move_pen_to_x, "IgesGraphTextFontDef::init")?.lower()) != 1)
                || ((iges_live(all_move_pen_to_x, "IgesGraphTextFontDef::init")?.length()) != len)))
            || (((iges_live(all_move_pen_to_y, "IgesGraphTextFontDef::init")?.lower()) != 1)
                || ((iges_live(all_move_pen_to_y, "IgesGraphTextFontDef::init")?.length()) != len))
        {
            return Err(exceptions::dimension_mismatch(
                "IgesGraphTextFontDef : Init",
            ));
        }
        {
            let mut state = self.state.write();
            state.the_font_code = a_font_code;
            state.the_font_name = a_font_name.clone();
            state.the_superseded_font_code = a_superseded_font;
            state.the_superseded_font_entity = a_superseded_entity.clone();
            state.the_scale = a_scale;
            state.the_ascii_codes = all_ascii_codes.clone();
            state.the_next_char_origin_x = all_next_char_x.clone();
            state.the_next_char_origin_y = all_next_char_y.clone();
            state.the_nb_pen_motions = all_pen_motions.clone();
            state.the_pen_motions = all_pen_flags.clone();
            state.the_pen_moves_to_x = all_move_pen_to_x.clone();
            state.the_pen_moves_to_y = all_move_pen_to_y.clone();
        }
        self.init_type_and_form(310, 0);
        Ok(())
    }
    /// True if this definition supersedes another
    /// TextFontDefinition Entity,
    /// False if it supersedes value.
    pub fn is_superseded_font_entity(&self) -> bool {
        !(self.state.read().the_superseded_font_entity.is_null())
    }
    /// returns the number of characters in this definition.
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_characters(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_ascii_codes,
            "IgesGraphTextFontDef::nb_characters",
        )
    }
    /// returns the ASCII code of Chnum'th character.
    /// Exception OutOfRange is raised if Chnum at most 0 or Chnum &gt; NbCharacters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn ascii_code(&self, chnum: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_ascii_codes,
            chnum,
            "IgesGraphTextFontDef::ascii_code",
        )
    }
    /// returns grid location of origin of character next to Chnum'th char.
    /// Exception OutOfRange is raised if Chnum at most 0 or Chnum &gt; NbCharacters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn next_char_origin(&self, chnum: i32, nx: &mut i32, ny: &mut i32) -> Result<(), Failure> {
        *nx = iges_array1_item(
            &self.state.read().the_next_char_origin_x,
            chnum,
            "IgesGraphTextFontDef::next_char_origin",
        )?;
        *ny = iges_array1_item(
            &self.state.read().the_next_char_origin_y,
            chnum,
            "IgesGraphTextFontDef::next_char_origin",
        )?;
        Ok(())
    }
    /// returns number of pen motions for Chnum'th character.
    /// Exception OutOfRange is raised if Chnum at most 0 or Chnum &gt; NbCharacters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_pen_motions(&self, chnum: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_nb_pen_motions,
            chnum,
            "IgesGraphTextFontDef::nb_pen_motions",
        )
    }
    /// returns pen status(True if 1, False if 0) of Motionnum'th motion
    /// of Chnum'th character.
    /// Exception raised if Chnum at most 0 or Chnum &gt; NbCharacters or
    /// Motionnum at most 0 or Motionnum &gt; NbPenMotions
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn is_pen_up(&self, chnum: i32, motionnum: i32) -> Result<bool, Failure> {
        let mut motion_arr: Handle<HArray1<i32>> = iges_live(
            &self.state.read().the_pen_motions,
            "IgesGraphTextFontDef::is_pen_up",
        )?
        .value(chnum)?;
        let mut pen_status: i32 =
            iges_array1_item(&motion_arr, motionnum, "IgesGraphTextFontDef::is_pen_up")?;
        Ok(pen_status == 1)
    }
    /// `next_pen_position`.
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn next_pen_position(
        &self,
        chnum: i32,
        motionnum: i32,
        ix: &mut i32,
        iy: &mut i32,
    ) -> Result<(), Failure> {
        *ix = iges_array1_item(
            &iges_live(
                &self.state.read().the_pen_moves_to_x,
                "IgesGraphTextFontDef::next_pen_position",
            )?
            .value(chnum)?,
            motionnum,
            "IgesGraphTextFontDef::next_pen_position",
        )?;
        *iy = iges_array1_item(
            &iges_live(
                &self.state.read().the_pen_moves_to_y,
                "IgesGraphTextFontDef::next_pen_position",
            )?
            .value(chnum)?,
            motionnum,
            "IgesGraphTextFontDef::next_pen_position",
        )?;
        Ok(())
    }
}
