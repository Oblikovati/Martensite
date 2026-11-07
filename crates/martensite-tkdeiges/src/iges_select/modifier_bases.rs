//! What the modifiers of the norm share: one acts on the file writer, the other on the copied
//! model, and the class declarations.
use crate::iges_data::{IgesDataIgesModel, IgesDataIgesWriter, IgesDataProtocolOps};
use martensite_tkernel::{
    impl_transient, impl_transient_interface,
    standard::{Failure, Handle, StandardTransient},
    tcollection::AsciiString,
};
use martensite_tkxsbase::{
    if_select::{
        IFSelectContextModif, IFSelectContextWrite, IFSelectGeneralModifier,
        IFSelectGeneralModifierBase, IFSelectModifier,
    },
    interface::{InterfaceCopyTool, InterfaceInterfaceModel, InterfaceProtocol},
};

/// A modifier that acts on the writer, once the model is produced.
///
/// ```
/// use martensite_tkdeiges::iges_select::IgesSelectFileModifier;
/// use martensite_tkxsbase::if_select::IFSelectGeneralModifier;
/// fn may_change(modifier: &dyn IgesSelectFileModifier) -> bool { modifier.may_change_graph() }
/// ```
pub trait IgesSelectFileModifier: IFSelectGeneralModifier {
    /// Acts on the writer.
    ///
    /// # Errors
    /// A failure of the writer.
    fn perform(
        &self,
        ctx: &mut IFSelectContextWrite,
        writer: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure>;
}
impl_transient_interface!(
    dyn IgesSelectFileModifier,
    "IgesSelectFileModifier",
    dyn IFSelectGeneralModifier
);

/// A modifier of the copied model, for models of the norm.
///
/// ```
/// use martensite_tkdeiges::iges_select::IgesSelectModelModifier;
/// use martensite_tkxsbase::if_select::IFSelectGeneralModifier;
/// fn may_change(modifier: &dyn IgesSelectModelModifier) -> bool { modifier.may_change_graph() }
/// ```
pub trait IgesSelectModelModifier: IFSelectModifier {
    /// Sets the protocol in the context, then performs.
    ///
    /// # Errors
    /// A failure of the modifier.
    fn perform_protocol(
        &self,
        ctx: &mut IFSelectContextModif,
        target: &Handle<IgesDataIgesModel>,
        protocol: &Handle<dyn IgesDataProtocolOps>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        ctx.set_protocol(&protocol.cast());
        self.performing(ctx, target, tc)
    }
    /// The modification.
    ///
    /// # Errors
    /// A failure of the modifier.
    fn performing(
        &self,
        ctx: &mut IFSelectContextModif,
        target: &Handle<IgesDataIgesModel>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure>;
}
impl_transient_interface!(
    dyn IgesSelectModelModifier,
    "IgesSelectModelModifier",
    dyn IFSelectModifier
);

/// A modifier as the trace of the context reads it: its selection only.
struct TracedModifier {
    transient: StandardTransient,
    base: IFSelectGeneralModifierBase,
}
impl_transient!(
    TracedModifier,
    "IgesSelectTracedModifier",
    dyn IFSelectGeneralModifier,
    transient,
    interfaces: [dyn IFSelectGeneralModifier]
);
impl IFSelectGeneralModifier for TracedModifier {
    fn general_modifier_base(&self) -> &IFSelectGeneralModifierBase {
        &self.base
    }
    fn label(&self) -> AsciiString {
        AsciiString::default()
    }
}

/// What the modifiers of a copied model do on `perform`: the trace, the casts, then their own
/// `perform_protocol`.
///
/// # Errors
/// A failure of the modifier or of the check.
pub fn iges_model_modifier_perform<M: IgesSelectModelModifier + ?Sized>(
    modifier: &M,
    ctx: &mut IFSelectContextModif,
    target: &Handle<dyn InterfaceInterfaceModel>,
    protocol: &Handle<dyn InterfaceProtocol>,
    tc: &mut InterfaceCopyTool,
) -> Result<(), Failure> {
    let traced = TracedModifier {
        transient: StandardTransient::default(),
        base: IFSelectGeneralModifierBase::new(false),
    };
    traced.set_selection(&modifier.selection());
    ctx.trace_modifier(&Handle::new(traced).cast());
    let targ = target.cast::<IgesDataIgesModel>();
    if targ.is_null() {
        add_fail(ctx, "Model to Modify : unproper type")?;
        return Ok(());
    }
    modifier.perform_protocol(ctx, &targ, &protocol.cast(), tc)
}

/// Adds a failure to the general check of the context.
pub(super) fn add_fail(ctx: &mut IFSelectContextModif, text: &str) -> Result<(), Failure> {
    if let Some(check) = ctx.c_check(0)?.get() {
        check.add_fail_text(text, "");
    }
    Ok(())
}

/// Adds a warning to the general check of the context.
pub(super) fn add_warning(ctx: &mut IFSelectContextModif, text: &str) -> Result<(), Failure> {
    if let Some(check) = ctx.c_check(0)?.get() {
        check.add_warning_text(text, "");
    }
    Ok(())
}

/// Declares the shape of a modifier of a copied model: its fields, its runtime type, its `perform`
/// and its general members. The class gives its constructor and `performing`.
macro_rules! iges_model_modifier {
    ($(#[$meta:meta])* $class:ident, $recorded:literal, { $($field:ident: $ty:ty),* $(,)? }, $self_:ident => $label:expr) => {
        $(#[$meta])*
        pub struct $class {
            transient: martensite_tkernel::standard::StandardTransient,
            base: martensite_tkxsbase::if_select::IFSelectGeneralModifierBase,
            $($field: martensite_tkernel::standard::SharedState<$ty>,)*
        }
        martensite_tkernel::impl_transient!(
            $class,
            $recorded,
            dyn $crate::iges_select::IgesSelectModelModifier,
            transient,
            interfaces: [
                dyn martensite_tkxsbase::if_select::IFSelectGeneralModifier,
                dyn martensite_tkxsbase::if_select::IFSelectModifier,
                dyn $crate::iges_select::IgesSelectModelModifier
            ]
        );
        impl martensite_tkxsbase::if_select::IFSelectModifier for $class {
            fn perform(
                &self,
                ctx: &mut martensite_tkxsbase::if_select::IFSelectContextModif,
                target: &martensite_tkernel::standard::Handle<
                    dyn martensite_tkxsbase::interface::InterfaceInterfaceModel,
                >,
                protocol: &martensite_tkernel::standard::Handle<
                    dyn martensite_tkxsbase::interface::InterfaceProtocol,
                >,
                tc: &mut martensite_tkxsbase::interface::InterfaceCopyTool,
            ) -> Result<(), martensite_tkernel::standard::Failure> {
                $crate::iges_select::iges_model_modifier_perform(self, ctx, target, protocol, tc)
            }
        }
        impl martensite_tkxsbase::if_select::IFSelectGeneralModifier for $class {
            fn general_modifier_base(
                &self,
            ) -> &martensite_tkxsbase::if_select::IFSelectGeneralModifierBase {
                &self.base
            }
            fn label(&$self_) -> martensite_tkernel::tcollection::AsciiString {
                $label
            }
        }
    };
}
pub(super) use iges_model_modifier;

/// The base of a modifier, with the mark that it may change the graph of dependences.
pub(super) fn modifier_base(may_change_graph: bool) -> IFSelectGeneralModifierBase {
    IFSelectGeneralModifierBase::new(may_change_graph)
}
