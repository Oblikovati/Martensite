//! The `bugs_modalg_6` case whose script lowers the group's budget with `cpulimit 100`.
use crate::support::{
    bop_session::BopOptions,
    checkshape::checkshape_named,
    checkview::screenshot,
    commands::explode,
    named_checks::bopargcheck_faulties,
    restore::restore,
    session::{Session, ViewStep},
    two_step::TwoStepBoolean,
};
use martensite_tkbo::bop_algo::Operation;

pub fn bug24161(s: &mut dyn Session) {
    println!("==========");
    println!("OCC24161");
    println!("==========");
    println!("CPU and elapsed time limit set to 100 seconds");
    restore(s, "bug24161_MirrorSolids.brep", "m");
    explode(s, "m", None);
    let mut two_step = TwoStepBoolean::default();
    two_step.bop(s, "m_1", "m_2");
    two_step.result(s, "result", Operation::Fuse);
    checkshape_named(s, "result");
    let result = s.shape("result").expect("bopfuse result").clone();
    bopargcheck_faulties(&result, &BopOptions::default());
    s.view(ViewStep::SmallView);
    s.view(ViewStep::Donly(&["result"]));
    screenshot(s, "photo");
}
