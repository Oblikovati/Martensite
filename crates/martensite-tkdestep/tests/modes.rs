//! Independent named STEP configuration modes and integer representation contracts.
use martensite_tkdestep::de_step::*;
macro_rules! named_mode_contract {
    ($test:ident,$kind:ident,[$($label:ident),+],[$($value:expr),+]) => {
        #[test]
        fn $test() {
            let named = [$($kind::$label),+];
            assert_eq!(named.map($kind::as_raw), [$($value),+]);
            for raw in [i32::MIN,-3,0,7,i32::MAX] {
                assert_eq!($kind::from_raw(raw).as_raw(),raw);
            }
        }
    };
}
named_mode_contract!(
    continuity_modes,
    DeStepReadModeBSplineContinuity,
    [C0, C1, C2],
    [0, 1, 2]
);
named_mode_contract!(
    precision_modes,
    DeStepReadModePrecision,
    [FILE, USER],
    [0, 1]
);
named_mode_contract!(
    maximum_precision_modes,
    DeStepReadModeMaxPrecision,
    [PREFERRED, FORCED],
    [0, 1]
);
named_mode_contract!(
    surface_curve_modes,
    DeStepReadModeSurfaceCurve,
    [
        DEFAULT,
        USE_2D_PREFERRED,
        USE_2D_FORCED,
        USE_3D_PREFERRED,
        USE_3D_FORCED
    ],
    [0, 2, -2, 3, -3]
);
named_mode_contract!(
    angle_unit_modes,
    DeStepAngleUnitMode,
    [FILE, RAD, DEG],
    [0, 1, 2]
);
named_mode_contract!(
    product_context_modes,
    DeStepReadModeProductContext,
    [ALL, DESIGN, ANALYSIS],
    [1, 2, 3]
);
named_mode_contract!(
    shape_representation_modes,
    DeStepReadModeShapeRepr,
    [ALL, ABSR, MSSR, GBSSR, FBSR, EBWSR, GBWSR],
    [1, 2, 3, 4, 5, 6, 7]
);
named_mode_contract!(
    assembly_level_modes,
    DeStepReadModeAssemblyLevel,
    [ALL, ASSEMBLY, STRUCTURE, SHAPE],
    [1, 2, 3, 4]
);
named_mode_contract!(
    tessellation_modes,
    DeStepRWModeTessellated,
    [OFF, ON, ON_NO_B_REP],
    [0, 1, 2]
);
named_mode_contract!(
    write_precision_modes,
    DeStepWriteModePrecisionMode,
    [LEAST, AVERAGE, GREATEST, SESSION],
    [-1, 0, 1, 2]
);
named_mode_contract!(
    write_assembly_modes,
    DeStepWriteModeAssembly,
    [OFF, ON, AUTO],
    [0, 1, 2]
);
named_mode_contract!(
    write_schema_modes,
    DeStepWriteModeStepSchema,
    [AP214_CD, AP214_DIS, AP203, AP214_IS, AP242_DIS],
    [1, 2, 3, 4, 5]
);
named_mode_contract!(
    write_vertex_modes,
    DeStepWriteModeVertexMode,
    [ONE_COMPOUND, SINGLE_VERTEX],
    [0, 1]
);
named_mode_contract!(
    representation_model_modes,
    StepControlStepModelType,
    [
        AS_IS,
        MANIFOLD_SOLID_BREP,
        BREP_WITH_VOIDS,
        FACETED_BREP,
        FACETED_BREP_AND_BREP_WITH_VOIDS,
        SHELL_BASED_SURFACE_MODEL,
        GEOMETRIC_CURVE_SET,
        HYBRID
    ],
    [0, 1, 2, 3, 4, 5, 6, 7]
);
