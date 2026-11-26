use super::DimensionKind;

#[test]
fn dimension_kind_flags_match_the_command_grammar() {
    let cases = [
        (DimensionKind::Length, "-length"),
        (DimensionKind::Angle, "-angle"),
        (DimensionKind::Radius, "-radius"),
        (DimensionKind::Diameter, "-diameter"),
    ];
    for (kind, flag) in cases {
        assert_eq!(kind.flag(), flag);
    }
}
