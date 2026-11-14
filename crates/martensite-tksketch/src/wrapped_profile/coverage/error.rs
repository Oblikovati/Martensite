//! Structural refusals preserve semantic owners and offending native uses.
use super::*;
pub(super) type Checked<T> = Result<T, CoverageFailure>;
#[derive(Debug)]
pub(in super::super) struct CoverageFailure {
    pub owner: BindingKey,
    pub cause: CoverageCause,
}
#[derive(Debug)]
pub(in super::super) enum CoverageCause {
    Native(Box<Failure>),
    Count {
        role: &'static str,
        actual: usize,
        expected: usize,
    },
    Kind {
        actual: ShapeEnum,
        expected: ShapeEnum,
    },
    DuplicateBinding {
        key: BindingKey,
        role: &'static str,
    },
    MissingBinding(BindingKey),
    AliasedBindings {
        first: BindingKey,
        second: BindingKey,
    },
    UnboundUse {
        actual: ShapeEnum,
    },
    RootBinding,
    Placement {
        orientation: Orientation,
        identity_location: bool,
    },
    ClosedFlag,
    Uses {
        actual: Vec<BoundUse>,
        expected: Vec<BoundUse>,
    },
    Contour {
        actual: Vec<BoundUse>,
    },
    EdgeFaces {
        actual: Vec<BoundUse>,
    },
    Representation {
        actual: String,
        matched_faces: Vec<BindingKey>,
    },
}
impl std::fmt::Display for CoverageFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "frozen coverage {:?}: {}; expected exactly the declared canonical solid/shell/face/wire/edge/vertex uses and representations",
            self.owner, self.cause
        )
    }
}
impl std::fmt::Display for CoverageCause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Native(cause) => write!(f, "{cause}"),
            Self::Count {
                role,
                actual,
                expected,
            } => write!(f, "{role}: {actual}, expected {expected}"),
            Self::Kind { actual, expected } => write!(f, "kind {actual:?}, expected {expected:?}"),
            Self::DuplicateBinding { key, role } => write!(f, "duplicate {role} key {key:?}"),
            Self::MissingBinding(key) => write!(f, "missing key {key:?}"),
            Self::AliasedBindings { first, second } => {
                write!(f, "aliased keys {first:?}/{second:?}")
            }
            _ => self.fmt_uses(f),
        }
    }
}
impl CoverageCause {
    fn fmt_uses(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Uses { actual, expected } => write!(f, "uses {actual:?}, expected {expected:?}"),
            Self::Contour { actual } => write!(f, "unexpected contour {actual:?}"),
            Self::EdgeFaces { actual } => write!(f, "non-opposite face uses {actual:?}"),
            Self::Representation {
                actual,
                matched_faces,
            } => write!(
                f,
                "representation {actual}, matching faces {matched_faces:?}"
            ),
            _ => self.fmt_placement(f),
        }
    }
    fn fmt_placement(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnboundUse { actual } => write!(f, "unbound {actual:?}"),
            Self::Placement {
                orientation,
                identity_location,
            } => write!(
                f,
                "orientation {orientation:?}, identity location {identity_location}"
            ),
            _ => write!(f, "{self:?}"),
        }
    }
}

pub(super) fn fail(owner: BindingKey, cause: CoverageCause) -> CoverageFailure {
    CoverageFailure { owner, cause }
}
pub(super) fn native<T>(owner: BindingKey, result: Result<T, Failure>) -> Checked<T> {
    result.map_err(|cause| fail(owner, CoverageCause::Native(Box::new(cause))))
}
pub(super) fn count(
    owner: BindingKey,
    role: &'static str,
    actual: usize,
    expected: usize,
) -> Checked<()> {
    if actual != expected {
        return Err(fail(
            owner,
            CoverageCause::Count {
                role,
                actual,
                expected,
            },
        ));
    }
    Ok(())
}
