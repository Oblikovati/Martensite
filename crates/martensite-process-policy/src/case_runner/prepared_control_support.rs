use super::{CaseSpec, FactorProfile, child_policy::PolicyInstaller, context::CaseContext};
use std::{cell::RefCell, error::Error, io, rc::Rc};

pub(super) type Events = Rc<RefCell<Vec<&'static str>>>;

pub(super) struct PreparedOwner {
    pub(super) events: Events,
}

impl Drop for PreparedOwner {
    fn drop(&mut self) {
        self.events.borrow_mut().push("drop");
    }
}

pub(super) fn prepare(events: &Events) -> Result<PreparedOwner, Box<dyn Error>> {
    events.borrow_mut().push("prepare");
    Ok(PreparedOwner {
        events: events.clone(),
    })
}

pub(super) struct OrderedPolicy<'a> {
    pub(super) context: &'a CaseContext,
    pub(super) events: Events,
    pub(super) refuse: bool,
}

impl PolicyInstaller for OrderedPolicy<'_> {
    fn install(&self, seconds: i64) -> Result<(), Box<dyn Error>> {
        assert_eq!(seconds, 60);
        self.context.require_record("prepared.txt").unwrap();
        self.events.borrow_mut().push("policy");
        if self.refuse {
            return Err(Box::new(io::Error::from_raw_os_error(1)));
        }
        Ok(())
    }
}

pub(super) fn spec() -> CaseSpec<'static> {
    CaseSpec::new("controls::prepared", 60, FactorProfile::RequireUnset)
}

pub(super) fn assert_errno(error: &super::CaseFailure, errno: i32) {
    let original = error.source().unwrap().downcast_ref::<io::Error>().unwrap();
    assert_eq!(original.raw_os_error(), Some(errno));
}
