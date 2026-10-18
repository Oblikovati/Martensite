//! Distinct caller input arrays belong to the operation, separate from workspace.
use super::*;

/// A caller real array whose current values can change during iso evaluation.
/// `AdvApp2VarCallerRealArray::IsoRoots` names the caller's normalized roots.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdvApp2VarCallerRealArray {
    /// Normalized iso roots, not generated sample parameters.
    IsoRoots,
    /// Requested approximation limits for caller subspaces.
    SubspaceTolerances,
}
const ARRAYS: [AdvApp2VarCallerRealArray; 2] = [
    AdvApp2VarCallerRealArray::IsoRoots,
    AdvApp2VarCallerRealArray::SubspaceTolerances,
];
impl AdvApp2VarCallerRealArray {
    fn index(self) -> usize {
        match self {
            Self::IsoRoots => 0,
            Self::SubspaceTolerances => 1,
        }
    }
    pub(super) fn request_values<'a>(self, request: &IsoApproxRequest<'a>) -> &'a [f64] {
        match self {
            Self::IsoRoots => request.rootlg,
            Self::SubspaceTolerances => request.epsapr,
        }
    }
}
#[derive(Default)]
pub(super) struct OwnedCallerRealArrays {
    arrays: [Option<Vec<f64>>; 2],
    pub(super) slots: [Option<Vec<usize>>; 2],
}
impl OwnedCallerRealArrays {
    pub(super) fn seed_request(&mut self, request: &IsoApproxRequest<'_>) {
        for array in ARRAYS {
            if let Some(values) = &mut self.arrays[array.index()] {
                values.clear();
                values.extend_from_slice(array.request_values(request));
            }
        }
    }
    pub(super) fn validate_slot_shapes(
        &self,
        request: &IsoApproxRequest<'_>,
    ) -> Result<(), Failure> {
        for array in ARRAYS {
            let Some(slots) = &self.slots[array.index()] else {
                continue;
            };
            let length = array.request_values(request).len();
            if slots.len() != length {
                return Err(Failure::with_message(
                    "Standard_DomainError",
                    &format!(
                        "AdvApp2VarIsoRealFrame: caller array {array:?} input len={length}; expected {} values for configured slots",
                        slots.len()
                    ),
                ));
            }
        }
        Ok(())
    }
    fn array(&self, array: AdvApp2VarCallerRealArray) -> Result<&[f64], Failure> {
        self.arrays[array.index()]
            .as_deref()
            .ok_or_else(|| missing_caller_array(array))
    }
    fn array_mut(&mut self, array: AdvApp2VarCallerRealArray) -> Result<&mut [f64], Failure> {
        self.arrays[array.index()]
            .as_deref_mut()
            .ok_or_else(|| missing_caller_array(array))
    }
}
impl AdvApp2VarIsoRealFrame {
    /// Own a distinct caller array, seeded from the request at operation entry;
    /// `let frame = frame.with_owned_caller_array(AdvApp2VarCallerRealArray::IsoRoots)`.
    pub fn with_owned_caller_array(mut self, array: AdvApp2VarCallerRealArray) -> Self {
        self.inputs.arrays[array.index()] = Some(Vec::new());
        self.inputs.slots[array.index()] = None;
        for group in &mut self.incidence {
            group
                .fields
                .retain(|field| !matches!(field,CallerRealField::Array(kind,_) if *kind==array));
        }
        self.incidence.retain(|group| !group.fields.is_empty());
        self
    }
    pub(in crate::adv_app2var) fn caller_array_value_or(
        &self,
        array: AdvApp2VarCallerRealArray,
        fallback: &[f64],
        index: usize,
    ) -> f64 {
        if self.inputs.slots[array.index()].is_some() || self.inputs.arrays[array.index()].is_some()
        {
            return self.caller_array_value(array,index).expect("AdvApp2VarIsoRealFrame: caller array index invalid; expected validated initialized subspace tolerance");
        }
        fallback[index]
    }
    pub(in crate::adv_app2var) fn caller_array_projection<'a>(
        &self,
        array: AdvApp2VarCallerRealArray,
        fallback: &'a [f64],
    ) -> std::borrow::Cow<'a, [f64]> {
        if let Some(slots) = &self.inputs.slots[array.index()] {
            return std::borrow::Cow::Owned(self.values.selected_values(slots));
        }
        match &self.inputs.arrays[array.index()] {
            Some(values) => std::borrow::Cow::Owned(values.clone()),
            None => std::borrow::Cow::Borrowed(fallback),
        }
    }
    /// Inspect the operation's caller array; `frame.caller_array(AdvApp2VarCallerRealArray::IsoRoots)?`.
    pub fn caller_array(&self, array: AdvApp2VarCallerRealArray) -> Result<&[f64], Failure> {
        self.require_distinct_array(array)?;
        self.inputs.array(array)
    }
    /// Mutate distinct caller input storage without aliasing the workspace;
    /// `frame.caller_array_mut(AdvApp2VarCallerRealArray::SubspaceTolerances)?[0]=1e-6`.
    pub fn caller_array_mut(
        &mut self,
        array: AdvApp2VarCallerRealArray,
    ) -> Result<&mut [f64], Failure> {
        self.require_distinct_array(array)?;
        self.inputs.array_mut(array)
    }
}
impl AdvApp2VarIsoRealFrame {
    /// Bind array elements to initialized owned real slots, allowing shared caller identity;
    /// `let frame=frame.with_caller_array_slots(AdvApp2VarCallerRealArray::SubspaceTolerances,vec![4])?`.
    pub fn with_caller_array_slots(
        mut self,
        array: AdvApp2VarCallerRealArray,
        slots: Vec<usize>,
    ) -> Result<Self, Failure> {
        validate_array_slots(array, &slots)?;
        let mut candidate = self.inputs.slots.clone();
        candidate[array.index()] = Some(slots);
        let incidence = real_incidence(
            self.values.len(),
            self.bindings,
            &candidate,
            self.curve_region.clone(),
        )?;
        self.inputs.slots = candidate;
        self.inputs.arrays[array.index()] = None;
        self.incidence = incidence;
        Ok(self)
    }
    /// Read an indexed caller element, including preceding incident scalar writes;
    /// `frame.caller_array_value(AdvApp2VarCallerRealArray::SubspaceTolerances,0)?`.
    pub fn caller_array_value(
        &self,
        array: AdvApp2VarCallerRealArray,
        index: usize,
    ) -> Result<f64, Failure> {
        match &self.inputs.slots[array.index()] {
            Some(slots) => Ok(self.values.value(array_element(slots, array, index)?)),
            None => array_element(self.inputs.array(array)?, array, index),
        }
    }
    /// Write one caller element without creating an aliasing slice;
    /// `frame.set_caller_array_value(AdvApp2VarCallerRealArray::SubspaceTolerances,0,1e-6)?`.
    pub fn set_caller_array_value(
        &mut self,
        array: AdvApp2VarCallerRealArray,
        index: usize,
        value: f64,
    ) -> Result<(), Failure> {
        if let Some(slots) = &self.inputs.slots[array.index()] {
            let slot = array_element(slots, array, index)?;
            self.values.set_value(slot, value);
        } else {
            let values = self.inputs.array_mut(array)?;
            array_element(values, array, index)?;
            values[index] = value;
        }
        Ok(())
    }
    fn require_distinct_array(&self, array: AdvApp2VarCallerRealArray) -> Result<(), Failure> {
        if self.inputs.slots[array.index()].is_none() {
            return Ok(());
        }
        Err(Failure::with_message(
            "Standard_DomainError",
            &format!(
                "AdvApp2VarIsoRealFrame: caller array {array:?} uses indexed slots; expected distinct owned array for slice access; use indexed element access"
            ),
        ))
    }
}
pub(super) fn mapped_real_fields(
    arrays: &[Option<Vec<usize>>; 2],
) -> Vec<(CallerRealField, usize)> {
    ARRAYS
        .into_iter()
        .flat_map(|array| {
            arrays[array.index()].iter().flat_map(move |slots| {
                slots
                    .iter()
                    .enumerate()
                    .map(move |(index, slot)| (CallerRealField::Array(array, index), *slot))
            })
        })
        .collect()
}
fn validate_array_slots(array: AdvApp2VarCallerRealArray, slots: &[usize]) -> Result<(), Failure> {
    let unique: std::collections::BTreeSet<usize> = slots.iter().copied().collect();
    if !slots.is_empty() && unique.len() == slots.len() {
        return Ok(());
    }
    Err(Failure::with_message(
        "Standard_DomainError",
        &format!(
            "AdvApp2VarIsoRealFrame: caller array {array:?} slots={slots:?}; expected nonempty distinct slots for contiguous caller-array elements"
        ),
    ))
}
fn array_element<T: Copy>(
    values: &[T],
    array: AdvApp2VarCallerRealArray,
    index: usize,
) -> Result<T, Failure> {
    values.get(index).copied().ok_or_else(||Failure::with_message("Standard_OutOfRange",&format!("AdvApp2VarIsoRealFrame: caller array {array:?} index={index}; expected element in 0..{}",values.len())))
}

fn missing_caller_array(array: AdvApp2VarCallerRealArray) -> Failure {
    Failure::with_message(
        "Standard_DomainError",
        &format!(
            "AdvApp2VarIsoRealFrame: caller array {array:?}=None; expected configured operation-owned caller array"
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    fn owned_frame() -> AdvApp2VarIsoRealFrame {
        AdvApp2VarIsoRealFrame::new(
            vec![0.; 5],
            AdvApp2VarRealBindings {
                domain: [0, 1, 2, 3],
                constant_parameter: 4,
            },
        )
        .unwrap()
    }
    fn request<'a>(roots: &'a [f64], limits: &'a [f64]) -> IsoApproxRequest<'a> {
        IsoApproxRequest {
            ndimen: 1,
            nbsesp: 1,
            ndimse: &[1],
            uvfonc: [0., 1., 0., 1.],
            tconst: 0.5,
            isofav: 1,
            nbroot: roots.len() as i32,
            rootlg: roots,
            iordre: -1,
            ideriv: 0,
            ndgjac: 3,
            nbcrmx: 1,
            ncflim: 1,
            epsapr: limits,
        }
    }
    #[test]
    fn unconfigured_projection_borrows_the_input() {
        let frame = owned_frame();
        for array in ARRAYS {
            assert!(matches!(
                frame.caller_array_projection(array, &[0.5]),
                std::borrow::Cow::Borrowed(_)
            ));
        }
    }
    #[test]
    fn configured_projection_observes_current_owned_values() {
        let mut frame = owned_frame()
            .with_owned_caller_array(ARRAYS[0])
            .with_owned_caller_array(ARRAYS[1]);
        frame.seed_request(&request(&[0.25], &[0.5])).unwrap();
        for array in ARRAYS {
            frame.caller_array_mut(array).unwrap()[0] = 0.75;
            assert_eq!(frame.caller_array_projection(array, &[1.]).as_ref(), [0.75]);
            assert!(matches!(
                frame.caller_array_projection(array, &[1.]),
                std::borrow::Cow::Owned(_)
            ));
        }
    }
    #[test]
    fn arrays_are_opt_in_and_missing_context_is_typed() {
        let mut frame = owned_frame();
        for array in ARRAYS {
            let failure = frame.caller_array(array).unwrap_err();
            assert_eq!(failure.exception_type(), "Standard_DomainError");
            assert!(failure.message().contains(&format!("{array:?}=None")));
            assert!(frame.caller_array_mut(array).is_err());
        }
    }
    #[test]
    fn owned_arrays_preserve_bits_remain_distinct_and_reseed_per_operation() {
        let mut frame = owned_frame()
            .with_owned_caller_array(ARRAYS[0])
            .with_owned_caller_array(ARRAYS[1]);
        let (roots, limits) = ([-0., 0.75], [-0., 2.]);
        frame.seed_request(&request(&roots, &limits)).unwrap();
        assert_eq!(
            frame.caller_array(ARRAYS[0]).unwrap()[0].to_bits(),
            (-0_f64).to_bits()
        );
        frame.caller_array_mut(ARRAYS[0]).unwrap()[0] = 1.;
        assert_eq!(
            frame.caller_array(ARRAYS[1]).unwrap()[0].to_bits(),
            (-0_f64).to_bits()
        );
        frame.caller_array_mut(ARRAYS[1]).unwrap()[1] = 1e-12;
        assert_eq!(frame.caller_array(ARRAYS[0]).unwrap(), [1., 0.75]);
        frame.seed_request(&request(&[0.25], &[0.5])).unwrap();
        assert_eq!(frame.caller_array(ARRAYS[0]).unwrap(), [0.25]);
        assert_eq!(frame.caller_array(ARRAYS[1]).unwrap(), [0.5]);
        assert_eq!(roots, [-0., 0.75]);
        assert_eq!(limits, [-0., 2.]);
    }
    #[test]
    fn owning_one_array_does_not_enable_the_other() {
        let mut frame = owned_frame().with_owned_caller_array(ARRAYS[0]);
        frame.seed_request(&request(&[0.25], &[0.5])).unwrap();
        assert_eq!(frame.caller_array(ARRAYS[0]).unwrap(), [0.25]);
        assert!(frame.caller_array(ARRAYS[1]).is_err());
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<AdvApp2VarCallerRealArray>();
        assert_send_sync::<AdvApp2VarIsoRealFrame>();
    }
    #[test]
    fn indexed_tolerance_and_scalar_share_one_initialized_owned_value() {
        let mut frame = owned_frame()
            .with_caller_array_slots(ARRAYS[1], vec![4])
            .unwrap();
        frame.seed_request(&request(&[0.25], &[0.5])).unwrap();
        frame.set_caller_array_value(ARRAYS[1], 0, 1e-12).unwrap();
        assert_eq!(frame.real(AdvApp2VarRealField::ConstantParameter), 1e-12);
        frame.set_real(AdvApp2VarRealField::ConstantParameter, 0.75);
        assert_eq!(frame.caller_array_value(ARRAYS[1], 0).unwrap(), 0.75);
        assert_eq!(
            frame.caller_array_projection(ARRAYS[1], &[1.]).as_ref(),
            [0.75]
        );
        assert!(frame.caller_array(ARRAYS[1]).is_err());
        assert!(frame.caller_array_mut(ARRAYS[1]).is_err());
        assert_eq!(
            frame
                .caller_array_value(ARRAYS[1], 1)
                .unwrap_err()
                .exception_type(),
            "Standard_OutOfRange"
        );
        assert_eq!(
            frame
                .set_caller_array_value(ARRAYS[1], 1, 2.)
                .unwrap_err()
                .exception_type(),
            "Standard_OutOfRange"
        );
    }
    #[test]
    fn array_slot_bindings_reject_empty_duplicate_out_of_range_and_wrong_input_shape() {
        for (slots, kind) in [
            (vec![], "Standard_DomainError"),
            (vec![4, 4], "Standard_DomainError"),
            (vec![5], "Standard_OutOfRange"),
        ] {
            assert_eq!(
                owned_frame()
                    .with_caller_array_slots(ARRAYS[1], slots)
                    .err()
                    .unwrap()
                    .exception_type(),
                kind
            );
        }
        let mut frame = owned_frame()
            .with_caller_array_slots(ARRAYS[1], vec![4])
            .unwrap();
        let failure = frame
            .seed_request(&request(&[0.25], &[0.5, 1.]))
            .unwrap_err();
        assert!(failure.message().contains("input len=2; expected 1"));
        let failure = frame.seed_request(&request(&[0.25], &[0.25])).unwrap_err();
        assert!(
            failure
                .message()
                .contains("incident SubspaceTolerances[0] value=0.25")
        );
    }
    #[test]
    fn indexed_access_to_distinct_arrays_checks_bounds_without_sharing_scalar_storage() {
        let mut frame = owned_frame()
            .with_owned_caller_array(ARRAYS[0])
            .with_owned_caller_array(ARRAYS[1]);
        frame.seed_request(&request(&[0.25], &[0.5])).unwrap();
        for array in ARRAYS {
            frame.set_caller_array_value(array, 0, 0.75).unwrap();
            assert_eq!(frame.caller_array_value(array, 0).unwrap(), 0.75);
            assert_eq!(
                frame
                    .set_caller_array_value(array, 1, 2.)
                    .unwrap_err()
                    .exception_type(),
                "Standard_OutOfRange"
            );
        }
        assert_eq!(frame.real(AdvApp2VarRealField::ConstantParameter), 0.5);
        let mut missing = owned_frame();
        assert!(missing.caller_array_value(ARRAYS[0], 0).is_err());
        assert!(missing.set_caller_array_value(ARRAYS[1], 0, 0.).is_err());
    }
    #[test]
    fn replacing_indexed_with_distinct_ownership_removes_only_that_arrays_incidence() {
        let mut frame = owned_frame()
            .with_caller_array_slots(ARRAYS[0], vec![4])
            .unwrap()
            .with_caller_array_slots(ARRAYS[1], vec![3])
            .unwrap()
            .with_owned_caller_array(ARRAYS[1]);
        frame.seed_request(&request(&[0.5], &[0.25])).unwrap();
        frame.set_real(AdvApp2VarRealField::ConstantParameter, 0.75);
        assert_eq!(frame.caller_array_value(ARRAYS[0], 0).unwrap(), 0.75);
        assert_eq!(frame.caller_array(ARRAYS[1]).unwrap(), [0.25]);
        assert_eq!(frame.real(AdvApp2VarRealField::VEnd), 1.);
    }
    #[test]
    fn replacing_distinct_with_indexed_ownership_seeds_current_inputs_and_preserves_bit_checks() {
        let mut frame = owned_frame()
            .with_owned_caller_array(ARRAYS[1])
            .with_caller_array_slots(ARRAYS[1], vec![0])
            .unwrap();
        let failure = frame.seed_request(&request(&[0.25], &[-0.])).unwrap_err();
        assert!(
            failure
                .message()
                .contains("expected shared input value=0.0")
        );
        frame.seed_request(&request(&[0.25], &[0.])).unwrap();
        frame.set_caller_array_value(ARRAYS[1], 0, 0.75).unwrap();
        assert_eq!(frame.real(AdvApp2VarRealField::UStart), 0.75);
    }
}
