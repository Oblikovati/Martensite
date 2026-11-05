//! `bcbuild`, `bcadd`, `bcremoveall` and `bcremoveint`: the cells builder over the session's arguments.
use super::options_bop::ScriptBop;
use crate::common_operation::report_result_alerts;
use crate::support::{commands::bound_shape, session::Session};
use martensite_tkbo::bop_algo::BOPAlgoCellsBuilder;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::ncollection::List;

/// The cells builder of one script; `let mut cells = ScriptCells::default();`.
#[derive(Default)]
pub struct ScriptCells {
    builder: Option<BOPAlgoCellsBuilder>,
}

impl ScriptCells {
    fn built(&mut self) -> &mut BOPAlgoCellsBuilder {
        self.builder.as_mut().expect("bcbuild first")
    }

    /// `bcbuild name`: every part of the session's arguments; `cells.bcbuild(s, &mut bop, "rx")`.
    pub fn bcbuild(&mut self, s: &mut dyn Session, bop: &mut ScriptBop, name: &str) {
        let mut cells = BOPAlgoCellsBuilder::new();
        for argument in bop.all_arguments() {
            cells.add_argument(&argument);
        }
        cells
            .options_mut()
            .set_run_parallel(bop.options.run_parallel);
        cells.options_mut().set_fuzzy_value(bop.options.fuzzy);
        cells.set_non_destructive(bop.options.non_destructive);
        cells.set_glue(bop.glue);
        cells.set_check_inverted(true);
        cells.options_mut().set_use_obb(false);
        cells.set_to_fill_history(bop.options.fill_history);
        cells.perform_with_filler(bop.prepared());
        report_result_alerts(cells.options());
        assert!(!cells.options().has_errors(), "bcbuild {name}: errors");
        s.bind(name, cells.all_parts());
        self.builder = Some(cells);
    }

    /// `bcaddall name -m material [-u]`; `cells.bcaddall(s, "res1", 1, true)`.
    pub fn bcaddall(&mut self, s: &mut dyn Session, name: &str, material: i32, update: bool) {
        let cells = self.built();
        cells
            .add_all_to_result(material, update)
            .unwrap_or_else(|failure| panic!("bcaddall {name}: {failure:?}"));
        report_result_alerts(cells.options());
        let result = cells.shape().clone();
        s.bind(name, &result);
    }

    /// `bcremoveall`; `cells.bcremoveall()`.
    pub fn bcremoveall(&mut self) {
        self.built()
            .remove_all_from_result()
            .unwrap_or_else(|failure| panic!("bcremoveall: {failure:?}"));
    }

    /// `bcadd name s1 flag s2 flag ... -m material`; the flag 1 takes a part and 0 avoids it.
    /// Example: `cells.bcadd(s, "res", &[("f2", true), ("f3", false)], 1)`.
    pub fn bcadd(
        &mut self,
        s: &mut dyn Session,
        name: &str,
        parts: &[(&str, bool)],
        material: i32,
    ) {
        let chosen = |flag: bool| -> List<Shape> {
            parts
                .iter()
                .filter(|(_, take)| *take == flag)
                .map(|(part, _)| bound_shape(s, part))
                .collect()
        };
        let (take, avoid) = (chosen(true), chosen(false));
        let cells = self.built();
        cells
            .add_to_result(&take, &avoid, material, false)
            .unwrap_or_else(|failure| panic!("bcadd {name}: {failure:?}"));
        report_result_alerts(cells.options());
        let result = cells.shape().clone();
        s.bind(name, &result);
    }

    /// `bcremoveint name`; `cells.bcremoveint(s, "res")`.
    pub fn bcremoveint(&mut self, s: &mut dyn Session, name: &str) {
        let cells = self.built();
        cells
            .remove_internal_boundaries()
            .unwrap_or_else(|failure| panic!("bcremoveint {name}: {failure:?}"));
        report_result_alerts(cells.options());
        let result = cells.shape().clone();
        s.bind(name, &result);
    }
}
