use super::modes::*;
use martensite_tkde::de::DeShapeFixParameters;
use martensite_tkernel::tcollection::AsciiString;

macro_rules! step_parameter_fields {
    ($(($recorded:literal,$name:ident,$kind:ty,$initial:expr)),* $(,)?) => {
        /// Values controlling STEP transfers; `DeStepParameters::default()`.
        #[derive(Clone, Debug, PartialEq)]
        pub struct DeStepParameters { $(#[doc=$recorded] pub $name: $kind,)* }
        impl Default for DeStepParameters {
            fn default() -> Self { Self { $($name: $initial,)* } }
        }
    };
}
step_parameter_fields! {
    ("`ReadBSplineContinuity` transfer setting.",read_b_spline_continuity,DeStepReadModeBSplineContinuity,DeStepReadModeBSplineContinuity::C1),
    ("`ReadPrecisionMode` transfer setting.",read_precision_mode,DeStepReadModePrecision,DeStepReadModePrecision::FILE),
    ("`ReadPrecisionVal` transfer setting.",read_precision_val,f64,0.0001),
    ("`ReadMaxPrecisionMode` transfer setting.",read_max_precision_mode,DeStepReadModeMaxPrecision,DeStepReadModeMaxPrecision::PREFERRED),
    ("`ReadMaxPrecisionVal` transfer setting.",read_max_precision_val,f64,1.0),
    ("`ReadSameParamMode` transfer setting.",read_same_param_mode,bool,false),
    ("`ReadSurfaceCurveMode` transfer setting.",read_surface_curve_mode,DeStepReadModeSurfaceCurve,DeStepReadModeSurfaceCurve::DEFAULT),
    ("`EncodeRegAngle` transfer setting.",encode_reg_angle,f64,0.57295779513),
    ("`AngleUnit` transfer setting.",angle_unit,DeStepAngleUnitMode,DeStepAngleUnitMode::FILE),
    ("`ReadProductMode` transfer setting.",read_product_mode,bool,true),
    ("`ReadProductContext` transfer setting.",read_product_context,DeStepReadModeProductContext,DeStepReadModeProductContext::ALL),
    ("`ReadShapeRepr` transfer setting.",read_shape_repr,DeStepReadModeShapeRepr,DeStepReadModeShapeRepr::ALL),
    ("`ReadTessellated` transfer setting.",read_tessellated,DeStepRWModeTessellated,DeStepRWModeTessellated::ON),
    ("`ReadAssemblyLevel` transfer setting.",read_assembly_level,DeStepReadModeAssemblyLevel,DeStepReadModeAssemblyLevel::ALL),
    ("`ReadRelationship` transfer setting.",read_relationship,bool,true),
    ("`ReadShapeAspect` transfer setting.",read_shape_aspect,bool,true),
    ("`ReadConstrRelation` transfer setting.",read_constr_relation,bool,false),
    ("`ReadSubshapeNames` transfer setting.",read_subshape_names,bool,false),
    ("`ReadCodePage` transfer setting.",read_code_page,i32,martensite_tkernel::resource::FormatType::Utf8 as i32),
    ("`ReadNonmanifold` transfer setting.",read_nonmanifold,bool,false),
    ("`ReadIdeas` transfer setting.",read_ideas,bool,false),
    ("`ReadAllShapes` transfer setting.",read_all_shapes,bool,false),
    ("`ReadRootTransformation` transfer setting.",read_root_transformation,bool,true),
    ("`ReadColor` transfer setting.",read_color,bool,true),
    ("`ReadName` transfer setting.",read_name,bool,true),
    ("`ReadLayer` transfer setting.",read_layer,bool,true),
    ("`ReadProps` transfer setting.",read_props,bool,true),
    ("`ReadMetadata` transfer setting.",read_metadata,bool,true),
    ("`ReadProductMetadata` transfer setting.",read_product_metadata,bool,false),
    ("`WritePrecisionMode` transfer setting.",write_precision_mode,DeStepWriteModePrecisionMode,DeStepWriteModePrecisionMode::AVERAGE),
    ("`WritePrecisionVal` transfer setting.",write_precision_val,f64,0.0001),
    ("`WriteAssembly` transfer setting.",write_assembly,DeStepWriteModeAssembly,DeStepWriteModeAssembly::AUTO),
    ("`WriteSchema` transfer setting.",write_schema,DeStepWriteModeStepSchema,DeStepWriteModeStepSchema::AP214_IS),
    ("`WriteTessellated` transfer setting.",write_tessellated,DeStepRWModeTessellated,DeStepRWModeTessellated::ON_NO_B_REP),
    ("`WriteProductName` transfer setting.",write_product_name,AsciiString,AsciiString::default()),
    ("`WriteSurfaceCurMode` transfer setting.",write_surface_cur_mode,bool,true),
    ("`WriteUnit` transfer setting.",write_unit,i32,martensite_tkernel::units_methods::LengthUnit::Millimeter as i32),
    ("`WriteVertexMode` transfer setting.",write_vertex_mode,DeStepWriteModeVertexMode,DeStepWriteModeVertexMode::ONE_COMPOUND),
    ("`WriteSubshapeNames` transfer setting.",write_subshape_names,bool,false),
    ("`WriteColor` transfer setting.",write_color,bool,true),
    ("`WriteNonmanifold` transfer setting.",write_nonmanifold,bool,false),
    ("`WriteName` transfer setting.",write_name,bool,true),
    ("`WriteLayer` transfer setting.",write_layer,bool,true),
    ("`WriteProps` transfer setting.",write_props,bool,true),
    ("`WriteMetadata` transfer setting.",write_metadata,bool,true),
    ("`WriteMaterial` transfer setting.",write_material,bool,true),
    ("`WriteVisMaterial` transfer setting.",write_vis_material,bool,false),
    ("`WriteModelType` transfer setting.",write_model_type,StepControlStepModelType,StepControlStepModelType::AS_IS),
    ("`CleanDuplicates` transfer setting.",clean_duplicates,bool,false),
    ("`WriteScalingTrsf` transfer setting.",write_scaling_trsf,bool,true),
}
impl DeStepParameters {
    /// Restores defaults; `parameters.reset();`.
    pub fn reset(&mut self) {
        *self = Self::default();
    }
    /// Returns STEP-specific healing defaults; `DeStepParameters::default_shape_fix_parameters()`.
    pub fn default_shape_fix_parameters() -> DeShapeFixParameters {
        DeShapeFixParameters::default()
    }
    /// Returns the product-context label, or empty for an unnamed value.
    /// Example: `parameters.get_string(DeStepReadModeProductContext::ALL)`.
    pub fn get_string(&self, mode: DeStepReadModeProductContext) -> AsciiString {
        match mode.as_raw() {
            1 => "all".into(),
            2 => "design".into(),
            3 => "analysis".into(),
            _ => AsciiString::default(),
        }
    }
}
