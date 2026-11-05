//! The cells builder commands: `bcbuild`, `bcadd`, `bcaddall`, `bcremove`, `bcremoveall`,
//! `bcremoveint` and `bcmakecontainers`, after `bfillds`.
use super::{bop_session::BopSession, commands::bound_shape, session::Session};
use crate::common_operation::report_result_alerts;
use martensite_tkbo::bop_algo::BOPAlgoCellsBuilder;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::ncollection::List;

/// What `bcadd` and `bcremove` take: names with `1` to take the part, `0` to avoid it.
pub type CellParts<'a> = &'a [(&'a str, i32)];

/// The builder a script's cells commands share; `let mut cells = CellsSession::default();`.
#[derive(Default)]
pub struct CellsSession {
    builder: Option<BOPAlgoCellsBuilder>,
}

fn split(s: &dyn Session, parts: CellParts<'_>) -> (List<Shape>, List<Shape>) {
    let of = |wanted: bool| -> List<Shape> {
        parts
            .iter()
            .filter(|(_, flag)| (*flag != 0) == wanted)
            .map(|(name, _)| bound_shape(s, name))
            .collect()
    };
    (of(true), of(false))
}

impl CellsSession {
    fn built(&mut self) -> &mut BOPAlgoCellsBuilder {
        self.builder.as_mut().expect("bcbuild before this command")
    }

    fn publish(&mut self, s: &mut dyn Session, name: &str) {
        let builder = self.built();
        report_result_alerts(builder.options());
        let shape = builder.shape().clone();
        s.bind(name, &shape);
    }

    /// `bcbuild name`: splits the arguments with the prepared intersections, binds all parts.
    /// Example: `cells.bcbuild(s, &mut bop, "r");`.
    pub fn bcbuild(&mut self, s: &mut dyn Session, bop: &mut BopSession, name: &str) {
        let mut builder = BOPAlgoCellsBuilder::new();
        bop.configure_cells(&mut builder);
        builder.perform_with_filler(bop.prepared_filler());
        report_result_alerts(builder.options());
        if !builder.options().has_errors() {
            s.bind(name, builder.all_parts());
        }
        self.builder = Some(builder);
    }

    /// `bcadd name a 1 b 0 ... [-m material [-u]]`.
    /// Example: `cells.bcadd(s, "result", &[("s1", 1), ("s2", 0)], 0, false);`.
    pub fn bcadd(
        &mut self,
        s: &mut dyn Session,
        name: &str,
        parts: CellParts<'_>,
        material: i32,
        update: bool,
    ) {
        // The command skips a name with no shape, after saying so.
        let bound: Vec<(&str, i32)> = parts
            .iter()
            .copied()
            .filter(|(part, _)| {
                let found = s.shape(part).is_some();
                if !found {
                    println!("{part} is a null shape");
                }
                found
            })
            .collect();
        let (take, avoid) = split(s, &bound);
        let builder = self.built();
        builder.options_mut().clear_warnings();
        builder
            .add_to_result(&take, &avoid, material, update)
            .unwrap_or_else(|failure| panic!("bcadd {name}: {failure:?}"));
        self.publish(s, name);
    }

    /// `bcaddall name [-m material [-u]]`; `cells.bcaddall(s, "result", 0, false);`.
    pub fn bcaddall(&mut self, s: &mut dyn Session, name: &str, material: i32, update: bool) {
        let builder = self.built();
        builder.options_mut().clear_warnings();
        builder
            .add_all_to_result(material, update)
            .unwrap_or_else(|failure| panic!("bcaddall {name}: {failure:?}"));
        self.publish(s, name);
    }

    /// `bcremove name a 1 b 0 ...`; `cells.bcremove(s, "result", &[("s1", 1)]);`.
    pub fn bcremove(&mut self, s: &mut dyn Session, name: &str, parts: CellParts<'_>) {
        let (take, avoid) = split(s, parts);
        let builder = self.built();
        builder.options_mut().clear_warnings();
        builder
            .remove_from_result(&take, &avoid)
            .unwrap_or_else(|failure| panic!("bcremove {name}: {failure:?}"));
        self.publish(s, name);
    }

    /// `bcremoveall`: empties the result; `cells.bcremoveall();`.
    pub fn bcremoveall(&mut self) {
        self.built()
            .remove_all_from_result()
            .unwrap_or_else(|failure| panic!("bcremoveall: {failure:?}"));
    }

    /// `bcremoveint name`: removes the boundaries between parts of one material.
    /// Example: `cells.bcremoveint(s, "result");`.
    pub fn bcremoveint(&mut self, s: &mut dyn Session, name: &str) {
        let builder = self.built();
        builder.options_mut().clear_warnings();
        builder
            .remove_internal_boundaries()
            .unwrap_or_else(|failure| panic!("bcremoveint {name}: {failure:?}"));
        self.publish(s, name);
    }

    /// `bcmakecontainers name`; `cells.bcmakecontainers(s, "result");`.
    pub fn bcmakecontainers(&mut self, s: &mut dyn Session, name: &str) {
        let builder = self.built();
        builder
            .make_containers()
            .unwrap_or_else(|failure| panic!("bcmakecontainers {name}: {failure:?}"));
        let shape = builder.shape().clone();
        s.bind(name, &shape);
    }
}
