use martensite_tkernel::impl_transient;
use martensite_tkernel::standard::{Handle, StandardTransient, Transient};
trait TestBase: Transient {}
martensite_tkernel::impl_transient_interface!(dyn TestBase, "TestBase", StandardTransient);
#[repr(C)]
#[derive(Default)]
struct TestDerived {}
impl_transient!(TestDerived, "TestDerived", dyn TestBase, interfaces: [dyn TestBase]);
impl TestBase for TestDerived {}
#[repr(C)]
#[derive(Default)]
struct TestOther {}
impl_transient!(TestOther, "TestOther", StandardTransient);
