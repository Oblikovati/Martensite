//! Array validation carries owner and slot provenance instead of retained addresses.
use super::*;
use line_binding::BoundLine;
use martensite_tkernel::ncollection::{Array1, HArray1};
use std::{
    ops::{Deref, DerefMut},
    sync::RwLockWriteGuard,
};

/// A shared line array with its owner retained throughout mutation and validation.
/// The write guard is thread-confined until its lock is released.
/// Example: `let lines=pair.lines(); let mut slots=TopOpeBRepFacesFillerArrayMut::new(&lines)?;`.
pub struct TopOpeBRepFacesFillerArrayMut<'a> {
    owner: &'a Handle<HArray1<TopOpeBRepLineInter>>,
    array: RwLockWriteGuard<'a, Array1<'static, TopOpeBRepLineInter>>,
}
impl<'a> TopOpeBRepFacesFillerArrayMut<'a> {
    /// Locks one array once while retaining its owner; `Self::new(&published_lines)?`.
    pub fn new(owner: &'a Handle<HArray1<TopOpeBRepLineInter>>) -> Result<Self, Failure> {
        let array = owner
            .get()
            .ok_or_else(|| {
                Failure::with_message(
                    "Standard_NullObject",
                    "FacesFillerArrayMut: owner=null; expected shared line array",
                )
            })?
            .change_array1();
        Ok(Self { owner, array })
    }
}
impl Deref for TopOpeBRepFacesFillerArrayMut<'_> {
    type Target = Array1<'static, TopOpeBRepLineInter>;
    fn deref(&self) -> &Self::Target {
        &self.array
    }
}
impl DerefMut for TopOpeBRepFacesFillerArrayMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.array
    }
}
impl TopOpeBRepFacesFiller<'_> {
    /// Validates a shared input slot using its retained slot under the existing lock.
    /// Example: `filler.check_line_in_array(&mut slots,1)?`.
    pub fn check_line_in_array(
        &mut self,
        slots: &mut TopOpeBRepFacesFillerArrayMut<'_>,
        input: i32,
    ) -> Result<bool, Failure> {
        let retained = retained_shared_index(self.bound_line.as_ref(), slots.owner)?;
        check_index(slots, input)?;
        check_index(slots, retained)?;
        if input == retained {
            return self.check_line_with_retained(
                slots
                    .array
                    .change_value(input)
                    .expect("validated input slot"),
                None,
            );
        }
        let (input, retained) = disjoint_lines(&mut slots.array, input, retained);
        self.check_line_with_retained(input, Some(retained))
    }
    /// Validates an independent external line against the guarded retained shared slot.
    /// Example: `filler.check_external_line_with_array(&mut slots,&mut candidate)?`.
    pub fn check_external_line_with_array(
        &mut self,
        slots: &mut TopOpeBRepFacesFillerArrayMut<'_>,
        candidate: &mut TopOpeBRepLineInter,
    ) -> Result<bool, Failure> {
        let retained = retained_shared_index(self.bound_line.as_ref(), slots.owner)?;
        check_index(slots, retained)?;
        let current = slots
            .array
            .change_value(retained)
            .expect("validated retained slot");
        self.check_line_with_retained(candidate, Some(current))
    }
}
fn retained_shared_index(
    bound: Option<&BoundLine<'_>>,
    owner: &Handle<HArray1<TopOpeBRepLineInter>>,
) -> Result<i32, Failure> {
    match bound {
        Some(BoundLine::Shared { lines, index }) if lines == owner => Ok(*index),
        Some(BoundLine::Shared { .. }) => Err(Failure::with_message(
            "Standard_DomainError",
            "CheckLine: shared owner_matches=false; expected owner_matches=true for retained array",
        )),
        _ => Err(Failure::with_message(
            "Standard_ProgramError",
            "CheckLine: no shared binding; expected retained shared line slot",
        )),
    }
}
fn check_index(slots: &Array1<'static, TopOpeBRepLineInter>, index: i32) -> Result<(), Failure> {
    slots.value(index).map(|_| ()).map_err(|_| {
        Failure::with_message(
            "Standard_OutOfRange",
            &format!(
                "CheckLine: slot={index}; expected {}..={}",
                slots.lower(),
                slots.upper()
            ),
        )
    })
}
fn disjoint_lines<'a>(
    slots: &'a mut Array1<'static, TopOpeBRepLineInter>,
    input: i32,
    retained: i32,
) -> (&'a mut TopOpeBRepLineInter, &'a mut TopOpeBRepLineInter) {
    let (input, retained) = (
        (input - slots.lower()) as usize,
        (retained - slots.lower()) as usize,
    );
    if input < retained {
        let (before, after) = slots.as_mut_slice().split_at_mut(retained);
        return (&mut before[input], &mut after[0]);
    }
    let (before, after) = slots.as_mut_slice().split_at_mut(input);
    (&mut after[0], &mut before[retained])
}
pub(super) fn shared_line_requires_guard() -> Failure {
    Failure::with_message(
        "Standard_ProgramError",
        "CheckLine: raw input has no shared-slot provenance; expected check_line_in_array or check_external_line_with_array with the retained array guard",
    )
}
#[cfg(test)]
#[path = "line_array_tests.rs"]
mod tests;
