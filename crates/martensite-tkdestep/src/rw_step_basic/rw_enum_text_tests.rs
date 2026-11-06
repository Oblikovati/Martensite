use crate::step_rw::*;

#[test]
fn enum_text_round_trip_rw_step_basic_rw_ahead_or_behind() {
    for value in [
        StepBasicAheadOrBehind::AobAhead,
        StepBasicAheadOrBehind::AobExact,
        StepBasicAheadOrBehind::AobBehind,
    ] {
        let mut read = StepBasicAheadOrBehind::default();
        assert!(RwStepBasicRwAheadOrBehind::convert_to_enum(
            RwStepBasicRwAheadOrBehind::convert_to_string(value),
            &mut read
        ));
        assert_eq!(read, value);
    }
    let mut read = StepBasicAheadOrBehind::default();
    assert!(!RwStepBasicRwAheadOrBehind::convert_to_enum(
        b".NOT_A_VALUE.",
        &mut read
    ));
}

#[test]
fn enum_text_round_trip_rw_step_basic_rw_si_prefix() {
    for value in [
        StepBasicSiPrefix::SpExa,
        StepBasicSiPrefix::SpPico,
        StepBasicSiPrefix::SpMega,
        StepBasicSiPrefix::SpFemto,
        StepBasicSiPrefix::SpAtto,
        StepBasicSiPrefix::SpCenti,
        StepBasicSiPrefix::SpNano,
        StepBasicSiPrefix::SpHecto,
        StepBasicSiPrefix::SpMicro,
        StepBasicSiPrefix::SpTera,
        StepBasicSiPrefix::SpGiga,
        StepBasicSiPrefix::SpMilli,
        StepBasicSiPrefix::SpPeta,
        StepBasicSiPrefix::SpDeci,
        StepBasicSiPrefix::SpKilo,
        StepBasicSiPrefix::SpDeca,
    ] {
        let mut read = StepBasicSiPrefix::default();
        assert!(RwStepBasicRwSiPrefix::convert_to_enum(
            RwStepBasicRwSiPrefix::convert_to_string(value),
            &mut read
        ));
        assert_eq!(read, value);
    }
    let mut read = StepBasicSiPrefix::default();
    assert!(!RwStepBasicRwSiPrefix::convert_to_enum(
        b".NOT_A_VALUE.",
        &mut read
    ));
}

#[test]
fn enum_text_round_trip_rw_step_basic_rw_si_unit_name() {
    for value in [
        StepBasicSiUnitName::SunHertz,
        StepBasicSiUnitName::SunDegreeCelsius,
        StepBasicSiUnitName::SunSiemens,
        StepBasicSiUnitName::SunSievert,
        StepBasicSiUnitName::SunLux,
        StepBasicSiUnitName::SunWatt,
        StepBasicSiUnitName::SunOhm,
        StepBasicSiUnitName::SunSecond,
        StepBasicSiUnitName::SunBecquerel,
        StepBasicSiUnitName::SunPascal,
        StepBasicSiUnitName::SunHenry,
        StepBasicSiUnitName::SunTesla,
        StepBasicSiUnitName::SunVolt,
        StepBasicSiUnitName::SunJoule,
        StepBasicSiUnitName::SunKelvin,
        StepBasicSiUnitName::SunAmpere,
        StepBasicSiUnitName::SunGram,
        StepBasicSiUnitName::SunSteradian,
        StepBasicSiUnitName::SunMole,
        StepBasicSiUnitName::SunLumen,
        StepBasicSiUnitName::SunGray,
        StepBasicSiUnitName::SunCandela,
        StepBasicSiUnitName::SunFarad,
        StepBasicSiUnitName::SunRadian,
        StepBasicSiUnitName::SunNewton,
        StepBasicSiUnitName::SunMetre,
        StepBasicSiUnitName::SunWeber,
        StepBasicSiUnitName::SunCoulomb,
    ] {
        let mut read = StepBasicSiUnitName::default();
        assert!(RwStepBasicRwSiUnitName::convert_to_enum(
            RwStepBasicRwSiUnitName::convert_to_string(value),
            &mut read
        ));
        assert_eq!(read, value);
    }
    let mut read = StepBasicSiUnitName::default();
    assert!(!RwStepBasicRwSiUnitName::convert_to_enum(
        b".NOT_A_VALUE.",
        &mut read
    ));
}

#[test]
fn enum_text_round_trip_rw_step_basic_rw_source() {
    for value in [
        StepBasicSource::SBought,
        StepBasicSource::SNotKnown,
        StepBasicSource::SMade,
    ] {
        let mut read = StepBasicSource::default();
        assert!(RwStepBasicRwSource::convert_to_enum(
            RwStepBasicRwSource::convert_to_string(value),
            &mut read
        ));
        assert_eq!(read, value);
    }
    let mut read = StepBasicSource::default();
    assert!(!RwStepBasicRwSource::convert_to_enum(
        b".NOT_A_VALUE.",
        &mut read
    ));
}
