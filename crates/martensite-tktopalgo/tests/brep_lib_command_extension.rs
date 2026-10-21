//! External composed makers retain the protected command completion transitions.
use martensite_tkernel::standard::Failure;
use martensite_tktopalgo::brep_lib::BRepLibCommand;

struct ComposedConstruction {
    command: BRepLibCommand,
}

impl ComposedConstruction {
    fn new() -> Self {
        Self {
            command: BRepLibCommand::new(),
        }
    }

    fn finish(&mut self) -> Result<(), Failure> {
        self.command.set_done();
        self.command.check()
    }

    fn invalidate(&mut self) -> Result<(), Failure> {
        self.command.set_not_done();
        self.command.check()
    }
}

#[test]
fn external_construction_starts_not_done_with_typed_failure() {
    let construction = ComposedConstruction::new();
    assert!(!construction.command.is_done());
    let failure = construction.command.check().unwrap_err();
    assert_eq!(failure.exception_type(), "StdFail_NotDone");
    assert_eq!(failure.message(), "BRep_API: command not done");
}

#[test]
fn external_completion_is_idempotent_and_allows_check() {
    let mut construction = ComposedConstruction::new();
    construction.finish().unwrap();
    construction.finish().unwrap();
    assert!(construction.command.is_done());
}

#[test]
fn external_invalidation_restores_failure_after_completion() {
    let mut construction = ComposedConstruction::new();
    construction.finish().unwrap();
    assert_eq!(
        construction.invalidate().unwrap_err().exception_type(),
        "StdFail_NotDone"
    );
    assert!(!construction.command.is_done());
    assert_eq!(
        construction.invalidate().unwrap_err().exception_type(),
        "StdFail_NotDone"
    );
    construction.finish().unwrap();
}

#[test]
fn command_completion_is_owned_by_each_construction() {
    let mut first = ComposedConstruction::new();
    let second = ComposedConstruction::new();
    first.finish().unwrap();
    assert!(first.command.is_done());
    assert!(!second.command.is_done());
    assert!(second.command.check().is_err());
}
