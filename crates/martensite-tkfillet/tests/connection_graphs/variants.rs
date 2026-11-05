//! Only local orientation/location values change; authored shared graph geometry stays untouched.
use super::{roles::GraphRoles, spec::CaseRow};
use martensite_tkbrep::topods::{self, Edge, Face};
use martensite_tkmath::{gp::Transform3, toploc::Location};

pub(super) struct ClassifierInputs {
    pub edge: Edge,
    pub first: Face,
    pub second: Face,
}

fn located(inputs: &mut ClassifierInputs) {
    let mut transform = Transform3::default();
    transform
        .set_values([[0., 1., 0., 7.], [0., 0., 1., -11.], [1., 0., 0., 13.]])
        .unwrap();
    let location = Location::new(transform);
    inputs.edge = topods::edge(&inputs.edge.moved(&location, false).unwrap()).unwrap();
    inputs.first = topods::face(&inputs.first.moved(&location, false).unwrap()).unwrap();
    inputs.second = topods::face(&inputs.second.moved(&location, false).unwrap()).unwrap();
}

fn orient(result: &mut ClassifierInputs, flags: [bool; 5]) {
    if flags[1] {
        std::mem::swap(&mut result.first, &mut result.second);
    }
    if flags[2] {
        result.first.reverse();
    }
    if flags[3] {
        result.second.reverse();
    }
    if flags[4] {
        result.edge.reverse();
    }
}

/// Build one local authored variant without changing original roles; `inputs(&roles, &row)`.
pub(super) fn inputs(roles: &GraphRoles, row: &CaseRow) -> ClassifierInputs {
    let mut result = ClassifierInputs {
        edge: roles.edge.clone(),
        first: roles.first.clone(),
        second: roles.second.clone(),
    };
    orient(&mut result, row.flags);
    if row.flags[0] {
        located(&mut result);
    }
    result
}
