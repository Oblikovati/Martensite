//! The conversions of `STEPCAFControlGDTProperty` between XCAF values and STEP entities.
use super::*;

/// The form variance written for each value.
const FORM_VARIANCES: [(XCAFDimTolObjectsDimensionFormVariance, &str); 29] = [
    (XCAFDimTolObjectsDimensionFormVariance::None, ""),
    (XCAFDimTolObjectsDimensionFormVariance::A, "A"),
    (XCAFDimTolObjectsDimensionFormVariance::B, "B"),
    (XCAFDimTolObjectsDimensionFormVariance::C, "C"),
    (XCAFDimTolObjectsDimensionFormVariance::CD, "CD"),
    (XCAFDimTolObjectsDimensionFormVariance::D, "D"),
    (XCAFDimTolObjectsDimensionFormVariance::E, "E"),
    (XCAFDimTolObjectsDimensionFormVariance::EF, "EF"),
    (XCAFDimTolObjectsDimensionFormVariance::F, "F"),
    (XCAFDimTolObjectsDimensionFormVariance::FG, "FG"),
    (XCAFDimTolObjectsDimensionFormVariance::G, "G"),
    (XCAFDimTolObjectsDimensionFormVariance::H, "H"),
    (XCAFDimTolObjectsDimensionFormVariance::JS, "JS"),
    (XCAFDimTolObjectsDimensionFormVariance::J, "J"),
    (XCAFDimTolObjectsDimensionFormVariance::K, "K"),
    (XCAFDimTolObjectsDimensionFormVariance::M, "M"),
    (XCAFDimTolObjectsDimensionFormVariance::N, "N"),
    (XCAFDimTolObjectsDimensionFormVariance::P, "P"),
    (XCAFDimTolObjectsDimensionFormVariance::R, "R"),
    (XCAFDimTolObjectsDimensionFormVariance::S, "S"),
    (XCAFDimTolObjectsDimensionFormVariance::T, "T"),
    (XCAFDimTolObjectsDimensionFormVariance::U, "U"),
    (XCAFDimTolObjectsDimensionFormVariance::V, "V"),
    (XCAFDimTolObjectsDimensionFormVariance::X, "X"),
    (XCAFDimTolObjectsDimensionFormVariance::Y, "Y"),
    (XCAFDimTolObjectsDimensionFormVariance::Z, "Z"),
    (XCAFDimTolObjectsDimensionFormVariance::ZA, "ZA"),
    (XCAFDimTolObjectsDimensionFormVariance::ZB, "ZB"),
    (XCAFDimTolObjectsDimensionFormVariance::ZC, "ZC"),
];

/// The form variances that are read, in the order; there is no `j`.
const READ_FORM_VARIANCES: [(&str, XCAFDimTolObjectsDimensionFormVariance); 27] = [
    ("a", XCAFDimTolObjectsDimensionFormVariance::A),
    ("b", XCAFDimTolObjectsDimensionFormVariance::B),
    ("c", XCAFDimTolObjectsDimensionFormVariance::C),
    ("cd", XCAFDimTolObjectsDimensionFormVariance::CD),
    ("d", XCAFDimTolObjectsDimensionFormVariance::D),
    ("e", XCAFDimTolObjectsDimensionFormVariance::E),
    ("ef", XCAFDimTolObjectsDimensionFormVariance::EF),
    ("f", XCAFDimTolObjectsDimensionFormVariance::F),
    ("fg", XCAFDimTolObjectsDimensionFormVariance::FG),
    ("g", XCAFDimTolObjectsDimensionFormVariance::G),
    ("h", XCAFDimTolObjectsDimensionFormVariance::H),
    ("js", XCAFDimTolObjectsDimensionFormVariance::JS),
    ("k", XCAFDimTolObjectsDimensionFormVariance::K),
    ("m", XCAFDimTolObjectsDimensionFormVariance::M),
    ("n", XCAFDimTolObjectsDimensionFormVariance::N),
    ("p", XCAFDimTolObjectsDimensionFormVariance::P),
    ("r", XCAFDimTolObjectsDimensionFormVariance::R),
    ("s", XCAFDimTolObjectsDimensionFormVariance::S),
    ("t", XCAFDimTolObjectsDimensionFormVariance::T),
    ("u", XCAFDimTolObjectsDimensionFormVariance::U),
    ("v", XCAFDimTolObjectsDimensionFormVariance::V),
    ("x", XCAFDimTolObjectsDimensionFormVariance::X),
    ("y", XCAFDimTolObjectsDimensionFormVariance::Y),
    ("z", XCAFDimTolObjectsDimensionFormVariance::Z),
    ("za", XCAFDimTolObjectsDimensionFormVariance::ZA),
    ("zb", XCAFDimTolObjectsDimensionFormVariance::ZB),
    ("zc", XCAFDimTolObjectsDimensionFormVariance::ZC),
];

fn text(value: &str) -> Handle<HAsciiString> {
    Handle::new(HAsciiString::new(value))
}

/// The lowered text of a string, a null dereference on a null string.
fn lowered(value: &Handle<HAsciiString>) -> Result<String, Failure> {
    let live = step_live(value, WHERE)?;
    Ok(String::from_utf8_lossy(live.string().as_bytes()).to_ascii_lowercase())
}

/// The value named by a lowered name.
fn value_named<T: Copy>(table: &[(&str, T)], name: &str) -> Option<T> {
    table
        .iter()
        .find(|(known, _)| *known == name)
        .map(|(_, value)| *value)
}

/// The name of a value, an empty string for one without a name.
fn name_of<T: PartialEq>(table: &[(&str, T)], value: &T) -> Handle<HAsciiString> {
    let found = table.iter().find(|(_, known)| known == value);
    Handle::new(found.map_or_else(HAsciiString::default, |(name, _)| HAsciiString::new(name)))
}

/// The form variance of a text compared with the lower case names, ignoring the case or not.
fn form_variance_of(
    form: &HAsciiString,
    case_sensitive: bool,
) -> Option<XCAFDimTolObjectsDimensionFormVariance> {
    let bytes = form.string();
    let bytes = bytes.as_bytes();
    READ_FORM_VARIANCES
        .iter()
        .find(|(name, _)| {
            if case_sensitive {
                bytes == name.as_bytes()
            } else {
                bytes.eq_ignore_ascii_case(name.as_bytes())
            }
        })
        .map(|(_, value)| *value)
}

/// `STEPCAFControlGDTProperty`: the static members.
///
/// ```
/// use martensite_tkdestep::stepcaf_control::STEPCAFControlGDTProperty;
/// use martensite_tkxcaf::xcaf_dim_tol_objects::XCAFDimTolObjectsDimensionType;
/// let name = STEPCAFControlGDTProperty::get_dim_type_name(XCAFDimTolObjectsDimensionType::SizeRadius);
/// assert_eq!(name.string().as_bytes(), b"radius");
/// ```
pub struct STEPCAFControlGDTProperty;

impl STEPCAFControlGDTProperty {
    /// `get_dim_modifiers`: appends to `modifiers` those named in the items of `cri`.
    pub fn get_dim_modifiers(
        cri: &Handle<StepReprCompoundRepresentationItem>,
        modifiers: &mut Vec<XCAFDimTolObjectsDimensionModif>,
    ) -> Result<(), Failure> {
        let elements = step_live(cri, WHERE)?.item_element();
        let count = step_live(&elements, WHERE)?.array1().length();
        for index in 1..=count {
            let item = step_array1_item(&elements, index, WHERE)?;
            let Some(description) = item
                .cast::<StepReprDescriptiveRepresentationItem>()
                .get()
                .map(StepReprDescriptiveRepresentationItem::description)
            else {
                continue;
            };
            let name = step_live(&description, WHERE)?.string();
            let name = String::from_utf8_lossy(name.as_bytes()).into_owned();
            if let Some(modifier) = value_named(&DIM_MODIFIERS, &name) {
                modifiers.push(modifier);
            }
        }
        Ok(())
    }
    /// `get_dim_class_of_tolerance`: `hole` when the form variance is not all in lower case (which is
    /// not verified information).
    pub fn get_dim_class_of_tolerance(
        laf: &Handle<StepShapeLimitsAndFits>,
        hole: &mut bool,
        form_variance: &mut XCAFDimTolObjectsDimensionFormVariance,
        grade: &mut XCAFDimTolObjectsDimensionGrade,
    ) -> Result<(), Failure> {
        let live = step_live(laf, WHERE)?;
        *form_variance = XCAFDimTolObjectsDimensionFormVariance::None;
        *hole = false;
        let form = live.form_variance();
        if let Some(form) = form.get() {
            if let Some(found) = form_variance_of(form, false) {
                *form_variance = found;
            }
            match form_variance_of(form, true) {
                Some(found) => *form_variance = found,
                None => *hole = true,
            }
        }
        *grade = XCAFDimTolObjectsDimensionGrade::IT01;
        let text = live.grade();
        let Some(text) = text.get() else {
            return Ok(());
        };
        if text.string().as_bytes() != b"01" && text.is_integer_value() {
            // An out of range grade is an invalid enumerator: it stays the first one here.
            let value = text.integer_value()? + 1;
            *grade = XCAFDimTolObjectsDimensionGrade::from_i32(value)
                .unwrap_or(XCAFDimTolObjectsDimensionGrade::IT01);
        }
        Ok(())
    }
    /// `get_dim_type`: true when the name is known.
    pub fn get_dim_type(
        name: &Handle<HAsciiString>,
        kind: &mut XCAFDimTolObjectsDimensionType,
    ) -> Result<bool, Failure> {
        let found = value_named(&DIM_TYPES, &lowered(name)?);
        *kind = found.unwrap_or(XCAFDimTolObjectsDimensionType::LocationNone);
        Ok(found.is_some())
    }
    /// `get_datum_target_type`: true when the name is known; `kind` stays otherwise.
    pub fn get_datum_target_type(
        description: &Handle<HAsciiString>,
        kind: &mut XCAFDimTolObjectsDatumTargetType,
    ) -> Result<bool, Failure> {
        let found = value_named(&DATUM_TARGETS, &lowered(description)?);
        if let Some(found) = found {
            *kind = found;
        }
        Ok(found.is_some())
    }
    /// `get_dim_qualifier_type`: true when the name is known.
    pub fn get_dim_qualifier_type(
        description: &Handle<HAsciiString>,
        kind: &mut XCAFDimTolObjectsDimensionQualifier,
    ) -> Result<bool, Failure> {
        *kind = match lowered(description)?.as_str() {
            "maximum" => XCAFDimTolObjectsDimensionQualifier::Max,
            "minimum" => XCAFDimTolObjectsDimensionQualifier::Min,
            "average" => XCAFDimTolObjectsDimensionQualifier::Avg,
            _ => XCAFDimTolObjectsDimensionQualifier::None,
        };
        Ok(*kind != XCAFDimTolObjectsDimensionQualifier::None)
    }
    /// `get_tol_value_type(description, type)`: true when the name is known.
    pub fn get_tol_value_type(
        description: &Handle<HAsciiString>,
        kind: &mut XCAFDimTolObjectsGeomToleranceTypeValue,
    ) -> Result<bool, Failure> {
        *kind = match lowered(description)?.as_str() {
            "cylindrical or circular" => XCAFDimTolObjectsGeomToleranceTypeValue::Diameter,
            "spherical" => XCAFDimTolObjectsGeomToleranceTypeValue::SphericalDiameter,
            _ => XCAFDimTolObjectsGeomToleranceTypeValue::None,
        };
        Ok(*kind != XCAFDimTolObjectsGeomToleranceTypeValue::None)
    }
    /// `get_tol_value_type(type)`: its name.
    pub fn get_tol_value_type_name(
        kind: XCAFDimTolObjectsGeomToleranceTypeValue,
    ) -> Handle<HAsciiString> {
        text(match kind {
            XCAFDimTolObjectsGeomToleranceTypeValue::Diameter => "cylindrical or circular",
            XCAFDimTolObjectsGeomToleranceTypeValue::SphericalDiameter => "spherical",
            _ => "unknown",
        })
    }
    /// `get_dim_type_name`: empty for the other entities.
    pub fn get_dim_type_name(kind: XCAFDimTolObjectsDimensionType) -> Handle<HAsciiString> {
        name_of(&DIM_TYPES, &kind)
    }
    /// `get_dim_qualifier_name`: empty when there is none.
    pub fn get_dim_qualifier_name(
        qualifier: XCAFDimTolObjectsDimensionQualifier,
    ) -> Handle<HAsciiString> {
        match qualifier {
            XCAFDimTolObjectsDimensionQualifier::Min => text("minimum"),
            XCAFDimTolObjectsDimensionQualifier::Avg => text("average"),
            XCAFDimTolObjectsDimensionQualifier::Max => text("maximum"),
            _ => Handle::new(HAsciiString::default()),
        }
    }
    /// `get_dim_modifier_name`.
    pub fn get_dim_modifier_name(
        modifier: XCAFDimTolObjectsDimensionModif,
    ) -> Handle<HAsciiString> {
        name_of(&DIM_MODIFIERS, &modifier)
    }
    /// `get_limits_and_fits`: the form is in lower case for a shaft.
    pub fn get_limits_and_fits(
        hole: bool,
        form_variance: XCAFDimTolObjectsDimensionFormVariance,
        grade: XCAFDimTolObjectsDimensionGrade,
    ) -> Handle<StepShapeLimitsAndFits> {
        let grade_text = if grade == XCAFDimTolObjectsDimensionGrade::IT01 {
            text("01")
        } else {
            Handle::new(HAsciiString::from_integer(grade as i32 + 1))
        };
        let form = FORM_VARIANCES
            .iter()
            .find(|(value, _)| *value == form_variance)
            .map_or("", |(_, name)| *name);
        let form = if hole {
            form.to_owned()
        } else {
            form.to_ascii_lowercase()
        };
        let limits = Handle::new(StepShapeLimitsAndFits::new());
        limits.init(
            &text(&form),
            &text(if hole { "hole" } else { "shaft" }),
            &grade_text,
            &Handle::new(HAsciiString::default()),
        );
        limits
    }
    /// `get_datum_target_name`: empty when there is none.
    pub fn get_datum_target_name(
        datum_type: XCAFDimTolObjectsDatumTargetType,
    ) -> Handle<HAsciiString> {
        name_of(&DATUM_TARGETS, &datum_type)
    }
    /// `GetGeomToleranceType(XCAF type)`: position for the others.
    pub fn get_step_tolerance_type(
        kind: XCAFDimTolObjectsGeomToleranceType,
    ) -> StepDimTolGeometricToleranceType {
        TOLERANCE_TYPES
            .iter()
            .find(|(xcaf, _)| *xcaf == kind)
            .map_or(
                StepDimTolGeometricToleranceType::GTTPositionTolerance,
                |(_, step)| *step,
            )
    }
    /// `GetGeomToleranceType(STEP type)`: position for the others.
    pub fn get_xcaf_tolerance_type(
        kind: StepDimTolGeometricToleranceType,
    ) -> XCAFDimTolObjectsGeomToleranceType {
        TOLERANCE_TYPES
            .iter()
            .find(|(_, step)| *step == kind)
            .map_or(XCAFDimTolObjectsGeomToleranceType::Position, |(xcaf, _)| {
                *xcaf
            })
    }
    /// `get_geom_tolerance`: a new tolerance of the type; null for those which need a datum system.
    pub fn get_geom_tolerance(
        kind: XCAFDimTolObjectsGeomToleranceType,
    ) -> Handle<StepDimTolGeometricTolerance> {
        match kind {
            XCAFDimTolObjectsGeomToleranceType::CircularityOrRoundness => {
                Handle::new(StepDimTolRoundnessTolerance::new()).cast()
            }
            XCAFDimTolObjectsGeomToleranceType::Cylindricity => {
                Handle::new(StepDimTolCylindricityTolerance::new()).cast()
            }
            XCAFDimTolObjectsGeomToleranceType::Flatness => {
                Handle::new(StepDimTolFlatnessTolerance::new()).cast()
            }
            XCAFDimTolObjectsGeomToleranceType::Position => {
                Handle::new(StepDimTolPositionTolerance::new()).cast()
            }
            XCAFDimTolObjectsGeomToleranceType::ProfileOfLine => {
                Handle::new(StepDimTolLineProfileTolerance::new()).cast()
            }
            XCAFDimTolObjectsGeomToleranceType::ProfileOfSurface => {
                Handle::new(StepDimTolSurfaceProfileTolerance::new()).cast()
            }
            XCAFDimTolObjectsGeomToleranceType::Straightness => {
                Handle::new(StepDimTolStraightnessTolerance::new()).cast()
            }
            _ => Handle::default(),
        }
    }
    /// `get_geom_tolerance_modifier`: maximum material requirement for the others.
    pub fn get_geom_tolerance_modifier(
        modifier: XCAFDimTolObjectsGeomToleranceModif,
    ) -> StepDimTolGeometricToleranceModifier {
        TOLERANCE_MODIFIERS
            .iter()
            .find(|(xcaf, _)| *xcaf == modifier)
            .map_or(
                StepDimTolGeometricToleranceModifier::GTMMaximumMaterialRequirement,
                |(_, step)| *step,
            )
    }
    /// The modifier with a value of `get_datum_ref_modifiers`.
    fn modifier_with_value(
        modif_with_val: XCAFDimTolObjectsDatumModifWithValue,
        value: f64,
        unit: &StepBasicUnit,
    ) -> Result<StepDimTolDatumReferenceModifier, Failure> {
        let kind = match modif_with_val {
            XCAFDimTolObjectsDatumModifWithValue::CircularOrCylindrical => {
                StepDimTolDatumReferenceModifierType::CircularOrCylindrical
            }
            XCAFDimTolObjectsDatumModifWithValue::Projected => {
                StepDimTolDatumReferenceModifierType::Projected
            }
            XCAFDimTolObjectsDatumModifWithValue::Spherical => {
                StepDimTolDatumReferenceModifierType::Spherical
            }
            _ => StepDimTolDatumReferenceModifierType::Distance,
        };
        let measure = Handle::new(StepBasicLengthMeasureWithUnit::new());
        let member = Handle::new(StepBasicMeasureValueMember::new());
        member.set_name("LENGTH_MEASURE");
        member.set_real(value);
        measure.init(&member, unit);
        let with_value = Handle::new(StepDimTolDatumReferenceModifierWithValue::new());
        with_value.init(kind, &measure);
        let mut modifier = StepDimTolDatumReferenceModifier::default();
        modifier.set_value(&with_value.cast())?;
        Ok(modifier)
    }
    /// `get_datum_ref_modifiers`: the modifiers of a datum reference, null without any; adds nothing to
    /// the model.
    pub fn get_datum_ref_modifiers(
        modifiers: &[XCAFDimTolObjectsDatumSingleModif],
        modif_with_val: XCAFDimTolObjectsDatumModifWithValue,
        value: f64,
        unit: &StepBasicUnit,
    ) -> Result<Handle<HArray1<StepDimTolDatumReferenceModifier>>, Failure> {
        let has_value = modif_with_val != XCAFDimTolObjectsDatumModifWithValue::None;
        if modifiers.is_empty() && !has_value {
            return Ok(Handle::default());
        }
        let mut all = Vec::with_capacity(modifiers.len() + 1);
        // Simple modifiers
        for single in modifiers {
            let member = Handle::new(StepDimTolSimpleDatumReferenceModifierMember::new());
            if let Some((_, name)) = SINGLE_MODIFIERS.iter().find(|(known, _)| known == single) {
                member.set_enum_text(0, name.as_bytes());
            }
            let mut modifier = StepDimTolDatumReferenceModifier::default();
            modifier.set_value(&member.cast())?;
            all.push(modifier);
        }
        // Modifier with value
        if has_value {
            all.push(Self::modifier_with_value(modif_with_val, value, unit)?);
        }
        Ok(shared_array1(all))
    }
    /// `get_tessellation`: the tessellated geometric set of a presentation shape.
    pub fn get_tessellation(
        shape: &Shape,
    ) -> Result<Handle<StepVisualTessellatedGeometricSet>, Failure> {
        tessellation_of(shape)
    }
}
