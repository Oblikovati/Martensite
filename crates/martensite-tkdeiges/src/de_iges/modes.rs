// Enum-valued settings retain their integer representation.
macro_rules! iges_integer_mode {
    ($name:ident, $($label:ident = $value:expr),+ $(,)?) => {
        #[doc = concat!("IGES transfer setting; `", stringify!($name), "::from_raw(0)`.")]
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
iges_integer_mode!(DeIgesReadModeBSplineContinuity, C0 = 0, C1 = 1, C2 = 2);
iges_integer_mode!(DeIgesReadModePrecision, FILE = 0, USER = 1);
iges_integer_mode!(DeIgesReadModeMaxPrecision, PREFERRED = 0, FORCED = 1);
iges_integer_mode!(
    DeIgesReadModeSurfaceCurve,
    DEFAULT = 0,
    USE_2D_PREFERRED = 2,
    USE_2D_FORCED = -2,
    USE_3D_PREFERRED = 3,
    USE_3D_FORCED = -3
);
iges_integer_mode!(DeIgesWriteModeBRep, FACES = 0, B_REP = 1);
iges_integer_mode!(DeIgesWriteModeConvertSurface, OFF = 0, ON = 1);
iges_integer_mode!(
    DeIgesWriteModePrecisionMode,
    LEAST = -1,
    AVERAGE = 0,
    GREATEST = 1,
    SESSION = 2
);
iges_integer_mode!(DeIgesWriteModePlaneMode, PLANE = 0, B_SPLINE = 1);
