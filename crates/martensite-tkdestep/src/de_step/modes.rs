// Enum-valued settings retain their integer representation.
macro_rules! step_integer_mode {
    ($name:ident, $($label:ident = $value:expr),+ $(,)?) => {
        #[doc = concat!("STEP transfer setting; `", stringify!($name), "::from_raw(0)`.")]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct $name(i32);
        impl $name {
            $(#[doc = concat!("Named value ", stringify!($label), ".")]
            pub const $label: Self = Self($value);)+
            /// Retains an integer setting; `let raw = 0;`.
            pub const fn from_raw(value: i32) -> Self { Self(value) }
            /// Returns the resource representation; `let raw = 0;`.
            pub const fn as_raw(self) -> i32 { self.0 }
        }
    };
}
step_integer_mode!(DeStepReadModeBSplineContinuity, C0 = 0, C1 = 1, C2 = 2);
step_integer_mode!(DeStepReadModePrecision, FILE = 0, USER = 1);
step_integer_mode!(DeStepReadModeMaxPrecision, PREFERRED = 0, FORCED = 1);
step_integer_mode!(
    DeStepReadModeSurfaceCurve,
    DEFAULT = 0,
    USE_2D_PREFERRED = 2,
    USE_2D_FORCED = -2,
    USE_3D_PREFERRED = 3,
    USE_3D_FORCED = -3
);
step_integer_mode!(DeStepAngleUnitMode, FILE = 0, RAD = 1, DEG = 2);
step_integer_mode!(
    DeStepReadModeProductContext,
    ALL = 1,
    DESIGN = 2,
    ANALYSIS = 3
);
step_integer_mode!(
    DeStepReadModeShapeRepr,
    ALL = 1,
    ABSR = 2,
    MSSR = 3,
    GBSSR = 4,
    FBSR = 5,
    EBWSR = 6,
    GBWSR = 7
);
step_integer_mode!(
    DeStepReadModeAssemblyLevel,
    ALL = 1,
    ASSEMBLY = 2,
    STRUCTURE = 3,
    SHAPE = 4
);
step_integer_mode!(DeStepRWModeTessellated, OFF = 0, ON = 1, ON_NO_B_REP = 2);
step_integer_mode!(
    DeStepWriteModePrecisionMode,
    LEAST = -1,
    AVERAGE = 0,
    GREATEST = 1,
    SESSION = 2
);
step_integer_mode!(DeStepWriteModeAssembly, OFF = 0, ON = 1, AUTO = 2);
step_integer_mode!(
    DeStepWriteModeStepSchema,
    AP214_CD = 1,
    AP214_DIS = 2,
    AP203 = 3,
    AP214_IS = 4,
    AP242_DIS = 5
);
step_integer_mode!(
    DeStepWriteModeVertexMode,
    ONE_COMPOUND = 0,
    SINGLE_VERTEX = 1
);
step_integer_mode!(
    StepControlStepModelType,
    AS_IS = 0,
    MANIFOLD_SOLID_BREP = 1,
    BREP_WITH_VOIDS = 2,
    FACETED_BREP = 3,
    FACETED_BREP_AND_BREP_WITH_VOIDS = 4,
    SHELL_BASED_SURFACE_MODEL = 5,
    GEOMETRIC_CURVE_SET = 6,
    HYBRID = 7
);
