//! `BRepAlgoAPICut`, `BRepAlgoAPIFuse` and `BRepAlgoAPICommon`: the Boolean operations
//! with their operation fixed at construction.
use super::boolean_operation::BRepAlgoAPIBooleanOperation;
use crate::bop_algo::{BOPAlgoPaveFiller, Operation, ProgressCompletion};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::Failure;
use std::ops::{Deref, DerefMut};

/// Defines one Boolean operation API over [`BRepAlgoAPIBooleanOperation`].
macro_rules! boolean_api {
    ($(#[$doc:meta])* $api:ident, $native:literal, $operation:expr) => {
        $(#[$doc])*
        pub struct $api {
            operation: BRepAlgoAPIBooleanOperation,
        }

        impl Default for $api {
            fn default() -> Self {
                Self::new()
            }
        }

        impl Deref for $api {
            type Target = BRepAlgoAPIBooleanOperation;
            fn deref(&self) -> &BRepAlgoAPIBooleanOperation {
                &self.operation
            }
        }

        impl DerefMut for $api {
            fn deref_mut(&mut self) -> &mut BRepAlgoAPIBooleanOperation {
                &mut self.operation
            }
        }

        impl $api {
            #[doc = concat!("An empty operation (`", $native, "()`); `", stringify!($api), "::new()`.")]
            pub fn new() -> Self {
                let mut operation = BRepAlgoAPIBooleanOperation::new();
                operation.set_operation($operation);
                Self { operation }
            }

            #[doc = concat!("An operation on the results of a performed filler (`", $native, "(PF)`).")]
            pub fn with_filler(filler: BOPAlgoPaveFiller) -> Self {
                let mut operation = BRepAlgoAPIBooleanOperation::with_filler(filler);
                operation.set_operation($operation);
                Self { operation }
            }

            #[doc = concat!("The operation of `object` and `tool`, performed (`", $native, "(S1, S2)`).")]
            pub fn from_shapes(object: &Shape, tool: &Shape) -> Result<Self, Failure> {
                Self::performed(object, tool, None, $operation)
            }

            #[doc = concat!("The operation of `object` and `tool`, performed in the caller's range (`", $native, "(S1, S2, Range)`).")]
            /// The completion is about execution, not validity.
            pub fn from_shapes_with_progress(
                object: &Shape,
                tool: &Shape,
                range: &ProgressRange<'_>,
            ) -> (Self, ProgressCompletion) {
                Self::performed_with_progress(object, tool, None, $operation, range)
            }

            fn performed_with_progress(
                object: &Shape,
                tool: &Shape,
                filler: Option<BOPAlgoPaveFiller>,
                operation: Operation,
                range: &ProgressRange<'_>,
            ) -> (Self, ProgressCompletion) {
                let operation =
                    BRepAlgoAPIBooleanOperation::with_shapes(object, tool, filler, operation);
                let mut api = Self { operation };
                let completion = api.operation.build_with_progress(range);
                (api, completion)
            }

            fn performed(
                object: &Shape,
                tool: &Shape,
                filler: Option<BOPAlgoPaveFiller>,
                operation: Operation,
            ) -> Result<Self, Failure> {
                let operation =
                    BRepAlgoAPIBooleanOperation::with_shapes(object, tool, filler, operation);
                let mut api = Self { operation };
                api.operation.build()?;
                Ok(api)
            }
        }
    };
}

boolean_api!(
    /// The part of the objects outside the tools.
    ///
    /// ```
    /// use martensite_tkbo::brep_algo_api::BRepAlgoAPICut;
    /// use martensite_tkbo::bop_algo::Operation;
    /// assert_eq!(BRepAlgoAPICut::new().operation(), Operation::Cut);
    /// ```
    BRepAlgoAPICut,
    "BRepAlgoAPICut",
    Operation::Cut
);

boolean_api!(
    /// The union of the objects and the tools.
    ///
    /// ```
    /// use martensite_tkbo::brep_algo_api::BRepAlgoAPIFuse;
    /// use martensite_tkbo::bop_algo::Operation;
    /// assert_eq!(BRepAlgoAPIFuse::new().operation(), Operation::Fuse);
    /// ```
    BRepAlgoAPIFuse,
    "BRepAlgoAPIFuse",
    Operation::Fuse
);

boolean_api!(
    /// The part common to the objects and the tools.
    ///
    /// ```
    /// use martensite_tkbo::brep_algo_api::BRepAlgoAPICommon;
    /// use martensite_tkbo::bop_algo::Operation;
    /// assert_eq!(BRepAlgoAPICommon::new().operation(), Operation::Common);
    /// ```
    BRepAlgoAPICommon,
    "BRepAlgoAPICommon",
    Operation::Common
);

impl BRepAlgoAPICut {
    /// The cut of `object` by `tool` (`forward`) or of `tool` by `object` on the results of
    /// a performed filler (`BRepAlgoAPICut(S1, S2, PF, bFWD)`).
    /// `BRepAlgoAPICut::from_shapes_with_filler(&a, &b, filler, true)?`.
    pub fn from_shapes_with_filler(
        object: &Shape,
        tool: &Shape,
        filler: BOPAlgoPaveFiller,
        forward: bool,
    ) -> Result<Self, Failure> {
        let operation = if forward {
            Operation::Cut
        } else {
            Operation::Cut21
        };
        Self::performed(object, tool, Some(filler), operation)
    }

    /// `from_shapes_with_filler` in the caller's range (`BRepAlgoAPICut(S1, S2, PF, bFWD, Range)`).
    /// Example: `BRepAlgoAPICut::from_shapes_with_filler_progress(&a, &b, filler, true, &range)`.
    pub fn from_shapes_with_filler_progress(
        object: &Shape,
        tool: &Shape,
        filler: BOPAlgoPaveFiller,
        forward: bool,
        range: &ProgressRange<'_>,
    ) -> (Self, ProgressCompletion) {
        let operation = if forward {
            Operation::Cut
        } else {
            Operation::Cut21
        };
        Self::performed_with_progress(object, tool, Some(filler), operation, range)
    }
}

impl BRepAlgoAPIFuse {
    /// The fuse of `object` and `tool` on the results of a performed filler
    /// (`BRepAlgoAPIFuse(S1, S2, PF)`); `BRepAlgoAPIFuse::from_shapes_with_filler(&a, &b, pf)?`.
    pub fn from_shapes_with_filler(
        object: &Shape,
        tool: &Shape,
        filler: BOPAlgoPaveFiller,
    ) -> Result<Self, Failure> {
        Self::performed(object, tool, Some(filler), Operation::Fuse)
    }

    /// `from_shapes_with_filler` in the caller's range (`BRepAlgoAPIFuse(S1, S2, PF, Range)`).
    /// Example: `BRepAlgoAPIFuse::from_shapes_with_filler_progress(&a, &b, filler, &range)`.
    pub fn from_shapes_with_filler_progress(
        object: &Shape,
        tool: &Shape,
        filler: BOPAlgoPaveFiller,
        range: &ProgressRange<'_>,
    ) -> (Self, ProgressCompletion) {
        Self::performed_with_progress(object, tool, Some(filler), Operation::Fuse, range)
    }
}

impl BRepAlgoAPICommon {
    /// The common of `object` and `tool` on the results of a performed filler
    /// (`BRepAlgoAPICommon(S1, S2, PF)`); `BRepAlgoAPICommon::from_shapes_with_filler(&a, &b, pf)?`.
    pub fn from_shapes_with_filler(
        object: &Shape,
        tool: &Shape,
        filler: BOPAlgoPaveFiller,
    ) -> Result<Self, Failure> {
        Self::performed(object, tool, Some(filler), Operation::Common)
    }

    /// `from_shapes_with_filler` in the caller's range (`BRepAlgoAPICommon(S1, S2, PF, Range)`).
    /// Example: `BRepAlgoAPICommon::from_shapes_with_filler_progress(&a, &b, filler, &range)`.
    pub fn from_shapes_with_filler_progress(
        object: &Shape,
        tool: &Shape,
        filler: BOPAlgoPaveFiller,
        range: &ProgressRange<'_>,
    ) -> (Self, ProgressCompletion) {
        Self::performed_with_progress(object, tool, Some(filler), Operation::Common, range)
    }
}
