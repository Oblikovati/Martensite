// SPDX-License-Identifier: GPL-2.0-only
//! Local changed-entity validation with cost measurement alongside full publication checks (#16, #213).
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::osd::MonotonicInstant;
use martensite_tkernel::standard::Failure;
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;

/// Scope of a B-rep validation pass (#16).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ValidationScope {
    /// Fast incremental check over only created or modified entities.
    LocalChangedOnly,
    /// Complete ordered check over the entire published body.
    FullPublication,
}

/// Timed validation report for local or full checks (#16, #213).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalCheckReport {
    scope: ValidationScope,
    checked_entities: usize,
    is_valid: bool,
    elapsed_micros: u128,
    defects: Vec<String>,
}

impl LocalCheckReport {
    /// Scope executed.
    pub fn scope(&self) -> ValidationScope {
        self.scope
    }
    /// Number of entities checked.
    pub fn checked_entities(&self) -> usize {
        self.checked_entities
    }
    /// True when all checked entities passed `BRepCheckAnalyzer`.
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }
    /// Measured wall-clock cost in microseconds.
    pub fn elapsed_micros(&self) -> u128 {
        self.elapsed_micros
    }
    /// Recorded defect descriptions.
    pub fn defects(&self) -> &[String] {
        &self.defects
    }
}

/// Validates only `changed_entities` and reports check cost (#16).
pub fn validate_changed_entities(changed_entities: &[Shape]) -> Result<LocalCheckReport, Failure> {
    let start = MonotonicInstant::now();
    let mut defects = Vec::new();
    let mut all_valid = true;
    for (idx, entity) in changed_entities.iter().enumerate() {
        if entity.is_null() {
            all_valid = false;
            defects.push(format!("changed entity {idx} is null"));
            continue;
        }
        let analyzer = BRepCheckAnalyzer::new(entity, true, false, false)?;
        if !analyzer.is_valid()? {
            all_valid = false;
            defects.push(format!("changed entity {idx} failed BRepCheckAnalyzer"));
        }
    }
    Ok(LocalCheckReport {
        scope: ValidationScope::LocalChangedOnly,
        checked_entities: changed_entities.len(),
        is_valid: all_valid,
        elapsed_micros: start.elapsed().as_micros(),
        defects,
    })
}

/// Runs the full ordered publication check on `shape` and reports check cost (#16, #213).
pub fn validate_full_publication(shape: &Shape) -> Result<LocalCheckReport, Failure> {
    let start = MonotonicInstant::now();
    let mut defects = Vec::new();
    let is_valid = if shape.is_null() {
        defects.push("published shape is null".to_string());
        false
    } else {
        let analyzer = BRepCheckAnalyzer::new(shape, true, false, false)?;
        let ok = analyzer.is_valid()?;
        if !ok {
            defects.push("published shape failed full BRepCheckAnalyzer".to_string());
        }
        ok
    };
    Ok(LocalCheckReport {
        scope: ValidationScope::FullPublication,
        checked_entities: usize::from(!shape.is_null()),
        is_valid,
        elapsed_micros: start.elapsed().as_micros(),
        defects,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkbrep::topexp::Explorer;
    use martensite_tkg3d::top_abs::ShapeEnum;
    use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

    #[test]
    fn local_changed_check_and_full_publication_check_measure_cost_and_detect_null() {
        let solid = BRepPrimAPIMakeBox::from_sizes(2.0, 2.0, 2.0)
            .expect("box")
            .shape()
            .expect("shape")
            .clone();
        let mut faces = Vec::new();
        let mut exp = Explorer::new(&solid, ShapeEnum::Face, ShapeEnum::Shape).expect("exp");
        while exp.more() && faces.len() < 2 {
            faces.push(exp.current().clone());
            exp.next().expect("next");
        }
        let local = validate_changed_entities(&faces).expect("local check");
        assert!(local.is_valid());
        assert_eq!(local.scope(), ValidationScope::LocalChangedOnly);
        assert_eq!(local.checked_entities(), 2);

        let full = validate_full_publication(&solid).expect("full check");
        assert!(full.is_valid());
        assert_eq!(full.scope(), ValidationScope::FullPublication);

        let bad = validate_changed_entities(&[Shape::default()]).expect("null check");
        assert!(!bad.is_valid());
        assert_eq!(bad.defects().len(), 1);
    }
}
