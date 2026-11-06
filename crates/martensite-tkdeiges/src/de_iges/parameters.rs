use super::modes::{
    DeIgesReadModeBSplineContinuity, DeIgesReadModeMaxPrecision, DeIgesReadModePrecision,
    DeIgesReadModeSurfaceCurve, DeIgesWriteModeBRep, DeIgesWriteModeConvertSurface,
    DeIgesWriteModePlaneMode, DeIgesWriteModePrecisionMode,
};
use martensite_tkde::de::{DeShapeFixParameters, DeShapeFixParametersFixMode};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkxsbase::xs_algo::DEShapeFixParameters as XsShapeFixParameters;

macro_rules! iges_parameter_fields {
    ($(($doc:literal,$name:ident,$kind:ty,$initial:expr)),* $(,)?) => {
        /// Values controlling IGES transfers; `DeIgesParameters::default()`.
        #[derive(Clone, Debug, PartialEq)]
        pub struct DeIgesParameters { $(#[doc=$doc] pub $name: $kind,)* }
        impl Default for DeIgesParameters {
            fn default() -> Self { Self { $($name: $initial,)* } }
        }
    };
}
iges_parameter_fields! {
    ("`ReadBSplineContinuity` transfer setting.",read_b_spline_continuity,DeIgesReadModeBSplineContinuity,DeIgesReadModeBSplineContinuity::C1),
    ("`ReadPrecisionMode` transfer setting.",read_precision_mode,DeIgesReadModePrecision,DeIgesReadModePrecision::FILE),
    ("`ReadPrecisionVal` transfer setting.",read_precision_val,f64,0.0001),
    ("`ReadMaxPrecisionMode` transfer setting.",read_max_precision_mode,DeIgesReadModeMaxPrecision,DeIgesReadModeMaxPrecision::PREFERRED),
    ("`ReadMaxPrecisionVal` transfer setting.",read_max_precision_val,f64,1.0),
    ("`ReadSameParamMode` transfer setting.",read_same_param_mode,bool,false),
    ("`ReadSurfaceCurveMode` transfer setting.",read_surface_curve_mode,DeIgesReadModeSurfaceCurve,DeIgesReadModeSurfaceCurve::DEFAULT),
    ("`EncodeRegAngle` transfer setting.",encode_reg_angle,f64,0.57295779513),
    ("`ReadApproxd1` transfer setting.",read_approxd1,bool,false),
    ("`ReadFaultyEntities` transfer setting.",read_faulty_entities,bool,false),
    ("`ReadOnlyVisible` transfer setting.",read_only_visible,bool,false),
    ("`ReadColor` transfer setting.",read_color,bool,true),
    ("`ReadName` transfer setting.",read_name,bool,true),
    ("`ReadLayer` transfer setting.",read_layer,bool,true),
    ("`WriteBRepMode` transfer setting.",write_b_rep_mode,DeIgesWriteModeBRep,DeIgesWriteModeBRep::FACES),
    ("`WriteConvertSurfaceMode` transfer setting.",write_convert_surface_mode,DeIgesWriteModeConvertSurface,DeIgesWriteModeConvertSurface::OFF),
    ("`WriteHeaderAuthor` transfer setting.",write_header_author,AsciiString,AsciiString::default()),
    ("`WriteHeaderCompany` transfer setting.",write_header_company,AsciiString,AsciiString::default()),
    ("`WriteHeaderProduct` transfer setting.",write_header_product,AsciiString,AsciiString::default()),
    ("`WriteHeaderReciever` transfer setting.",write_header_reciever,AsciiString,AsciiString::default()),
    ("`WritePrecisionMode` transfer setting.",write_precision_mode,DeIgesWriteModePrecisionMode,DeIgesWriteModePrecisionMode::AVERAGE),
    ("`WritePrecisionVal` transfer setting.",write_precision_val,f64,0.0001),
    ("`WritePlaneMode` transfer setting.",write_plane_mode,DeIgesWriteModePlaneMode,DeIgesWriteModePlaneMode::PLANE),
    ("`WriteOffsetMode` transfer setting.",write_offset_mode,bool,false),
    ("`WriteColor` transfer setting.",write_color,bool,true),
    ("`WriteName` transfer setting.",write_name,bool,true),
    ("`WriteLayer` transfer setting.",write_layer,bool,true),
}
impl DeIgesParameters {
    /// Restores the defaults; `parameters.reset();`.
    pub fn reset(&mut self) {
        *self = Self::default();
    }
    /// Returns IGES-specific healing defaults; `DeIgesParameters::default_shape_fix_parameters()`.
    pub fn default_shape_fix_parameters() -> DeShapeFixParameters {
        DeShapeFixParameters {
            detalization_level: ShapeEnum::Edge,
            create_open_solid_mode: DeShapeFixParametersFixMode::FIX,
            fix_tail_mode: DeShapeFixParametersFixMode::FIX_OR_NOT,
            max_tail_angle: DeShapeFixParametersFixMode::FIX_OR_NOT,
            ..DeShapeFixParameters::default()
        }
    }
}

/// The healing defaults of the exchange as the shape processing of the reader and writer takes them.
pub fn xs_shape_fix_parameters() -> XsShapeFixParameters {
    super::transfer_shape_fix_parameters(&DeIgesParameters::default_shape_fix_parameters())
}
