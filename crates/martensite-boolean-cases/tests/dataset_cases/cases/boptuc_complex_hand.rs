//! `boptuc_complex` bodies written by hand: the script loops over the solids of its result.
use crate::support::{
    beyond_script::shape_is_valid,
    beyond_script::{kind_is, printed_mass, view_mass_is_nonzero},
    checkprops::{Expected, checkprops},
    checkview::{CheckView, checkview},
    commands::explode,
    hooks::require_bopcheck_ok,
    nbshapes::{Counting, NbCount, checknbshapes},
    properties::Prop,
    restore::restore,
    session::Session,
    two_step::TwoStepBoolean,
};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkg3d::top_abs::ShapeEnum;

pub fn B5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b60", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(15000.));
    checkprops(s, "result", Prop::Volume, Expected::Mass(31250.));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Wires(24),
            NbCount::Faces(24),
            NbCount::Shells(6),
            NbCount::Solids(6),
        ],
    );
    for sol in explode(s, "result", Some(ShapeEnum::Solid)) {
        require_bopcheck_ok(
            s,
            &sol,
            &format!("Error: the result solid {sol} shape is self-interfered"),
        );
        // Beyond the script: an earlier test also checked each solid is valid.
        shape_is_valid(s, &sol);
    }
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a", Prop::Area, "15000");
    kind_is(s, "a", ShapeEnum::Compound);
    printed_mass(s, "b", Prop::Area, "15000");
    kind_is(s, "b", ShapeEnum::Compound);
    view_mass_is_nonzero(s, "result");
}
