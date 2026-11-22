//! Concrete owner type checks and fixed-order metadata queries have separate responsibilities.
mod observations;
mod roster;
use martensite_tkernel::standard::{RuntimeType, RuntimeTyped, Transient};
use observations::StaticObservation;

struct DescriptorQuery {
    native_name: &'static str,
    native_parent: &'static str,
    register: fn(),
    observe: fn() -> StaticObservation,
    dynamic_signature: fn(),
}

impl DescriptorQuery {
    const fn of<T: RuntimeTyped + Transient>(name: &'static str, parent: &'static str) -> Self {
        Self {
            native_name: name,
            native_parent: parent,
            register: register::<T>,
            observe: observations::observe::<T>,
            dynamic_signature: dynamic_signature::<T>,
        }
    }
    fn verify(&self) {
        (self.dynamic_signature)();
        let observation = (self.observe)();
        let _: &(dyn Send + Sync) = &observation;
        println!("{}", observation.render(self.native_name));
        observation.verify(self.native_name, self.native_parent);
    }
}

fn register<T: RuntimeTyped>() {
    // Register the whole roster before checking its descriptor relationships.
    let _: &'static RuntimeType = T::static_type();
}

fn dynamic_signature<T: Transient>() {
    let query: for<'a> fn(&'a T) -> &'static RuntimeType = T::dynamic_type;
    // This validates the concrete callable type; it does not execute a dynamic query.
    let _ = std::hint::black_box(query);
}

/// Check every explicit owner without manufacturing an instance; `verify_all()`.
pub(super) fn verify_all() {
    for group in roster::GROUPS {
        for query in *group {
            (query.register)();
        }
    }
    for group in roster::GROUPS {
        for query in *group {
            query.verify();
        }
    }
}
