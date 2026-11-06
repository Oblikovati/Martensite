//! IgesGraphLineFontDefPattern.
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
use crate::iges_data::{IgesDataLineFontEntity, iges_array1_item, iges_live};
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, exceptions};
use martensite_tkernel::tcollection::HAsciiString;

iges_entity! {
/// defines IGESLineFontDefPattern, Type `304` Form `2`
///
/// Line Font may be defined by repetition of a basic pattern
/// of visible-blank(or, on-off) segments superimposed on
/// a line or a curve. The line or curve is then displayed
/// according to the basic pattern.
IgesGraphLineFontDefPattern, IgesGraphLineFontDefPatternState, "IgesGraphLineFontDefPattern", kind dyn IgesDataLineFontEntity, {
the_segment_lengths: Handle<HArray1<f64>>,
the_display_pattern: Handle<HAsciiString>,
}
}

impl IgesDataLineFontEntity for IgesGraphLineFontDefPattern {}

impl IgesGraphLineFontDefPattern {
    iges_methods! {
    get display_pattern = the_display_pattern: Handle<HAsciiString>;
    }
    /// This method is used to set the fields of the class
    /// LineFontDefPattern
    /// - allSegLength : Containing lengths of respective segments
    /// - aPattern     : HAsciiString indicating visible-blank segments
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        all_seg_length: &Handle<HArray1<f64>>,
        a_pattern: &Handle<HAsciiString>,
    ) -> Result<(), Failure> {
        if iges_array1_lower(all_seg_length, "IgesGraphLineFontDefPattern::init")? != 1 {
            return Err(exceptions::dimension_mismatch(
                "IgesGraphLineFontDefPattern : Init",
            ));
        }
        {
            let mut state = self.state.write();
            state.the_segment_lengths = all_seg_length.clone();
            state.the_display_pattern = a_pattern.clone();
        }
        self.init_type_and_form(304, 2);
        Ok(())
    }
    /// returns the number of segments in the visible-blank pattern
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_segments(&self) -> Result<i32, Failure> {
        iges_array1_length(
            &self.state.read().the_segment_lengths,
            "IgesGraphLineFontDefPattern::nb_segments",
        )
    }
    /// returns the Length of Index'th segment of the basic pattern
    /// raises exception if Index at most 0 or Index &gt; NbSegments
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn length(&self, index: i32) -> Result<f64, Failure> {
        iges_array1_item(
            &self.state.read().the_segment_lengths,
            index,
            "IgesGraphLineFontDefPattern::length",
        )
    }
    /// Whether the segment of a rank from 1 is visible: the pattern's hexadecimal digits hold one bit per
    /// segment, right justified. False for a rank out of the segments.
    ///
    /// # Errors
    /// A failure when the lengths or the pattern are not set up, or the pattern is too short.
    pub fn is_visible(&self, index: i32) -> Result<bool, Failure> {
        let nb_segs = self.nb_segments()?;
        if index <= 0 || index > nb_segs {
            return Ok(false);
        }
        let pattern = self.display_pattern();
        let text = iges_live(&pattern, "IgesGraphLineFontDefPattern::is_visible")?.string();
        let rank = text.as_bytes().len() as i32 - (nb_segs - index) / 4;
        let digit = usize::try_from(rank - 1)
            .ok()
            .and_then(|at| text.as_bytes().get(at).copied());
        let Some(digit) = digit else {
            return Err(exceptions::out_of_range(&format!(
                "IgesGraphLineFontDefPattern::is_visible: rank {index} needs a digit at {rank} of the pattern"
            )));
        };
        // A character that is no hexadecimal digit reads as 0.
        let bits = char::from(digit).to_digit(16).unwrap_or(0);
        Ok(bits & (1 << ((nb_segs - index) % 4)) != 0)
    }
}
