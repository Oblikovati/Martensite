use super::*;
#[derive(Debug)]
pub(in super::super) enum HistoryFailure {
    SourceBinding,
    Clearance(Box<planar_clearance::ClearanceFailure>),
    DuplicateTarget(BindingKey),
    Corners {
        span: CurveSpan,
        actual: [CornerSource; 2],
        expected: [CornerSource; 2],
    },
    Targets {
        missing: Vec<BindingKey>,
        extra: Vec<BindingKey>,
    },
}
impl std::fmt::Display for HistoryFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SourceBinding => write!(
                f,
                "history sources changed; expected the complete original planar selection"
            ),
            Self::Clearance(cause) => write!(f, "history source intervals: {cause}"),
            Self::DuplicateTarget(key) => write!(
                f,
                "history target={key:?}; expected one semantic record per frozen entity"
            ),
            _ => self.correspondence(f),
        }
    }
}

impl HistoryFailure {
    fn correspondence(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Corners {
                span,
                actual,
                expected,
            } => write!(
                f,
                "history span={span:?} corners={actual:?}; expected authored/dyadic endpoints={expected:?}"
            ),
            Self::Targets { missing, extra } => write!(
                f,
                "history targets missing={missing:?}, extra={extra:?}; expected exactly the frozen binding keys"
            ),
            _ => write!(f, "{self:?}"),
        }
    }
}
