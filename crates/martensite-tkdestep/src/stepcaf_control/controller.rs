//! `STEPCAFControlController`: the STEP controller, with the write actor adapted for assemblies of
//! an XDE document. The read actor of STEPControl is inherited.
use super::STEPCAFControlActorWrite;
use crate::step_control::StepControlController;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::impl_transient;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkxsbase::if_select::IFSelectReturnStatus;
use martensite_tkxsbase::interface::{InterfaceInterfaceModel, InterfaceStatic};
use martensite_tkxsbase::transfer::{TransferActorOfTransientProcessOps, TransferFinderProcess};
use martensite_tkxsbase::xs_algo::XSAlgo;
use martensite_tkxsbase::xs_control::{XSControlController, XSControlControllerBase};
use std::sync::Mutex;

/// ```no_run
/// use martensite_tkdestep::stepcaf_control::STEPCAFControlController;
/// assert!(STEPCAFControlController::init().unwrap());
/// ```
pub struct STEPCAFControlController {
    transient: StandardTransient,
    inner: Handle<dyn XSControlController>,
}
impl_transient!(STEPCAFControlController, "STEPCAFControlController", StandardTransient, transient, interfaces: [dyn XSControlController]);

/// A static of two values, 0 by default.
fn define_off_on_static(name: &str) -> Result<(), Failure> {
    InterfaceStatic::init_char("stepcaf", name, b'e', "")?;
    InterfaceStatic::init_char("stepcaf", name, b'&', "enum 0")?;
    InterfaceStatic::init_char("stepcaf", name, b'&', "eval Off")?; // 0
    InterfaceStatic::init_char("stepcaf", name, b'&', "eval On")?; // 1
    InterfaceStatic::set_i_val(name, 0)?; // Disabled by default
    Ok(())
}

impl STEPCAFControlController {
    /// `STEPCAFControlController()`: the use of the STEP norm with the write actor of this package.
    #[allow(
        clippy::new_ret_no_self,
        reason = "a controller lives behind its trait handle"
    )]
    pub fn new() -> Result<Handle<dyn XSControlController>, Failure> {
        let inner = StepControlController::new()?;
        inner
            .controller_base()
            .set_adaptor_write(&Handle::new(STEPCAFControlActorWrite::new()).cast());
        let controller: Handle<dyn XSControlController> = Handle::new(Self {
            transient: StandardTransient::default(),
            inner,
        })
        .cast();
        controller.controller_base().bind_self(&controller);
        Ok(controller)
    }
    /// `init`: creates a controller for STEP-XCAF, once in the process, and records it to various
    /// names; true when done.
    pub fn init() -> Result<bool, Failure> {
        static DONE: Mutex<bool> = Mutex::new(false);
        let mut done = DONE
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if *done {
            return Ok(true);
        }
        *done = true;
        // self-registering
        let controller = Self::new()?;
        // do XSAlgo::init, cause it does not called before.
        XSAlgo::init()?;
        // do something to avoid warnings...
        controller.auto_record()?;
        // Whether to write sub-shape names to 'Name' attributes of STEP Representation Items
        define_off_on_static("write.stepcaf.subshapes.name")?;
        // Whether to read sub-shape names from 'Name' attributes of STEP Representation Items
        define_off_on_static("read.stepcaf.subshapes.name")?;
        Ok(true)
    }
}

impl XSControlController for STEPCAFControlController {
    fn controller_base(&self) -> &XSControlControllerBase {
        self.inner.controller_base()
    }
    fn new_model(&self) -> Handle<dyn InterfaceInterfaceModel> {
        self.inner.new_model()
    }
    fn actor_read(
        &self,
        model: &Handle<dyn InterfaceInterfaceModel>,
    ) -> Handle<dyn TransferActorOfTransientProcessOps> {
        self.inner.actor_read(model)
    }
    fn transfer_write_shape(
        &self,
        shape: &Shape,
        fp: &Handle<TransferFinderProcess>,
        model: &Handle<dyn InterfaceInterfaceModel>,
        modetrans: i32,
        progress: &ProgressRange<'_>,
    ) -> Result<IFSelectReturnStatus, Failure> {
        self.inner
            .transfer_write_shape(shape, fp, model, modetrans, progress)
    }
}
