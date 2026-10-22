#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ChildLifecycle {
    Direct,
    Prepared,
}

impl ChildLifecycle {
    pub(crate) fn request_suffix(self) -> &'static str {
        match self {
            Self::Direct => "",
            Self::Prepared => "lifecycle=prepare-before-policy\n",
        }
    }

    pub(crate) fn description(self) -> &'static str {
        match self {
            Self::Direct => "entry,policy,body,complete",
            Self::Prepared => "entry,prepare,policy,body,complete",
        }
    }
}

pub(super) struct ChildWorkload<P, B> {
    pub(super) prepare: P,
    pub(super) body: B,
    pub(super) lifecycle: ChildLifecycle,
}
