//! Miri-checked lifetimes of the lent destructor borrow: nested destructor scopes and an
//! unwind through the scope guard. Run under Miri with the cargo-miri runner in place of the
//! workspace's validation runner:
//! `CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER="$(rustup +nightly-2026-09-25 which cargo-miri) runner"
//! cargo +nightly-2026-09-25 miri test -p martensite-tkv3d --lib retirement_stash`.
use super::*;
use crate::ais::{AisTexturedShape, AisTexturedShapeMembers as _};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Handle, WeakHandle};

use std::panic::{AssertUnwindSafe, catch_unwind};

type StashLog = Arc<SharedState<Vec<String>>>;
struct StashPixels {
    base: ImagePixMap,
    owner: WeakHandle<AisTexturedShape>,
    inner: SharedState<Handle<ImagePixMap>>,
    stage: &'static str,
    log: StashLog,
}
martensite_tkernel::impl_transient!(StashPixels, "StashPixels", ImagePixMap, base);
impl Drop for StashPixels {
    fn drop(&mut self) {
        let owner = self.owner.observe();
        let source = &**owner;
        let opened = match self.stage {
            "unwind" => catch_unwind(AssertUnwindSafe(|| {
                ais_textured_shape_with_retirement(source, self, |_| panic!("scope body unwinds"))
            }))
            .is_err(),
            _ => ais_textured_shape_with_retirement(source, self, |scope| self.run(&scope)).is_ok(),
        };
        let active = owner.textured_shape_base().retirement.is_active();
        self.log
            .write()
            .push(format!("{} {opened} {active}", self.stage));
    }
}
impl StashPixels {
    fn run(&self, scope: &AisTexturedShapeRetirement<'_, AisTexturedShape>) {
        assert!(scope.source.textured_shape_base().retirement.is_active());
        if self.stage == "outer" {
            // Hand the inner pixmap to the slot, release our owner, then nullify: the inner
            // destructor runs nested inside this scope and opens its own.
            let inner = self.inner.read().clone();
            scope.set_texture_pix_map(&inner).unwrap();
            *self.inner.write() = Handle::default();
            drop(inner);
        }
        // Nullify through the virtual pixmap setter (the file setter parses through libc,
        // which Miri cannot call).
        scope
            .set_texture_pix_map(&Handle::<ImagePixMap>::default())
            .unwrap();
    }
}
fn stash_pixels(
    owner: &Handle<AisTexturedShape>,
    stage: &'static str,
    inner: Handle<ImagePixMap>,
    log: &StashLog,
) -> Handle<ImagePixMap> {
    Handle::new(StashPixels {
        base: ImagePixMap::default(),
        owner: owner.downgrade(),
        inner: SharedState::new(inner),
        stage,
        log: log.clone(),
    })
    .into_cast()
}
fn retire(owner: &Handle<AisTexturedShape>, mut pixels: Handle<ImagePixMap>) {
    owner.set_texture_pix_map(&pixels);
    pixels.nullify();
    owner.set_texture_pix_map(&Handle::<ImagePixMap>::default());
}
#[test]
fn retirement_stash_nested_destructor_scopes_pop_in_order() {
    let log: StashLog = Arc::new(SharedState::new(Vec::new()));
    let owner = AisTexturedShape::new(&Shape::default());
    let inner = stash_pixels(&owner, "inner", Handle::default(), &log);
    retire(&owner, stash_pixels(&owner, "outer", inner, &log));
    assert_eq!(*log.read(), ["inner true true", "outer true false"]);
    assert!(!owner.textured_shape_base().retirement.is_active());
    assert!(owner.texture_pix_map().is_null());
}
#[test]
fn retirement_stash_unwind_through_the_guard_removes_the_borrow() {
    let log: StashLog = Arc::new(SharedState::new(Vec::new()));
    let owner = AisTexturedShape::new(&Shape::default());
    retire(
        &owner,
        stash_pixels(&owner, "unwind", Handle::default(), &log),
    );
    assert_eq!(*log.read(), ["unwind true false"]);
    assert!(!owner.textured_shape_base().retirement.is_active());
}
