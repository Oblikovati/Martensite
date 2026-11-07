//! `IgesControlReader`: reads an IGES file into a work session and transfers its roots.
use super::IgesControlController;
use crate::de_iges::DeIgesParameters;
use crate::iges_data::{IgesDataIgesEntity, IgesDataIgesModel, iges_live};
use crate::iges_to_brep::{IgesToBRepActor, one_transfer_at_a_time};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkshhealing::shape_process::{ShapeProcessOperation, ShapeProcessOperationsFlags};
use martensite_tkxsbase::if_select::IFSelectReturnStatus;
use martensite_tkxsbase::interface::{
    InterfaceInterfaceModel, InterfaceModelEntities, InterfaceShareFlags, InterfaceStatic,
};
use martensite_tkxsbase::xs_algo::DEShapeFixParameters;
use martensite_tkxsbase::xs_control::{XSControlReader, XSControlReaderOps, XSControlWorkSession};
use std::io::Read;
use std::sync::MutexGuard;

const WHERE: &str = "IgesControlReader";

/// Reading through this class follows this flow:
/// ```no_run
/// use martensite_tkdeiges::iges_control::IgesControlReader;
/// use martensite_tkernel::message::ProgressRange;
/// use martensite_tkxsbase::if_select::IFSelectReturnStatus;
/// use martensite_tkxsbase::xs_control::XSControlReaderOps;
/// let mut reader = IgesControlReader::new().unwrap();
/// assert_eq!(reader.read_file("part.igs").unwrap(), IFSelectReturnStatus::RetDone);
/// reader.transfer_roots(&ProgressRange::default()).unwrap();
/// let shape = reader.xs_reader().one_shape();
/// ```
pub struct IgesControlReader {
    pub(super) reader: XSControlReader,
    read_only_visible: bool,
}

impl IgesControlReader {
    /// A reader on a new session; `IgesControlReader::new()?`.
    ///
    /// # Errors
    /// A failure of the init of the norm.
    pub fn new() -> Result<Self, Failure> {
        IgesControlController::init()?;
        let _only_one = one_transfer_at_a_time();
        let mut reader = XSControlReader::new();
        reader.set_ws(&XSControlWorkSession::new(), true)?;
        reader.set_norm("IGES")?;
        Ok(Self {
            reader,
            read_only_visible: InterfaceStatic::i_val("read.iges.onlyvisible") == 1,
        })
    }
    /// A reader on a session; `IgesControlReader::with_session(&session, true)?`.
    ///
    /// # Errors
    /// A failure of the init of the norm.
    pub fn with_session(ws: &Handle<XSControlWorkSession>, scratch: bool) -> Result<Self, Failure> {
        IgesControlController::init()?;
        let _only_one = one_transfer_at_a_time();
        let mut reader = XSControlReader::new();
        reader.set_ws(ws, scratch)?;
        reader.set_norm("IGES")?;
        Ok(Self {
            reader,
            read_only_visible: InterfaceStatic::i_val("read.iges.onlyvisible") == 1,
        })
    }
    /// Sets whether only visible entities are read; `reader.set_read_visible(true)`.
    pub fn set_read_visible(&mut self, only_visible: bool) {
        self.read_only_visible = only_visible;
    }
    /// Whether only visible entities are read; `reader.get_read_visible()`.
    pub fn get_read_visible(&self) -> bool {
        self.read_only_visible
    }
    /// The model read; `reader.iges_model()?`.
    ///
    /// # Errors
    /// A failure of the session.
    pub fn iges_model(&self) -> Result<Handle<IgesDataIgesModel>, Failure> {
        Ok(self.reader.model()?.cast())
    }
    /// Reads a file with the given parameters as settings of the session;
    /// `reader.read_with_parameters(&parameters)`.
    pub fn read_with_parameters(&mut self, parameters: &DeIgesParameters) {
        self.read_only_visible = parameters.read_only_visible;
    }
}

impl IgesControlReader {
    /// The lock of the format, with the reading actor on this reader's model: the actor is one per
    /// process, and reading another file on another thread moves it to that file's model.
    fn locked_on_own_model(&mut self) -> Result<MutexGuard<'static, ()>, Failure> {
        let guard = one_transfer_at_a_time();
        let generic: Handle<dyn InterfaceInterfaceModel> = self.iges_model()?.cast();
        let ws = self.reader.ws();
        let transfer = iges_live(&ws, WHERE)?.transfer_reader();
        let actor: Handle<IgesToBRepActor> = iges_live(&transfer, WHERE)?.actor().cast();
        if let (Some(live), true) = (actor.get(), generic.is_some()) {
            // A model without a global section keeps the tolerance of the actor.
            let _ = live.set_model(&generic);
        }
        Ok(guard)
    }
    /// The entities no other one refers to, that the actor takes and that are visible as asked.
    fn count_roots(&mut self) -> Result<i32, Failure> {
        if self.reader.therootsta {
            return Ok(self.reader.theroots.len() as i32);
        }
        self.reader.therootsta = true;
        let model = self.iges_model()?;
        let Some(live) = model.get() else {
            return Ok(0);
        };
        let ws = self.reader.ws();
        let session = iges_live(&ws, WHERE)?;
        let protocol = session.protocol();
        let controller = session.norm_adaptor();
        let generic: Handle<dyn InterfaceInterfaceModel> = model.cast();
        let actor = iges_live(&controller, WHERE)?.actor_read(&generic);
        let actor_live = iges_live(&actor, WHERE)?;
        let shares = InterfaceShareFlags::with_protocol(&generic, &protocol)?;
        InterfaceStatic::set_i_val("read.iges.onlyvisible", i32::from(self.read_only_visible))?;
        for rank in 1..=live.nb_entities() {
            let entity: Handle<IgesDataIgesEntity> = live.entity(rank);
            if shares.is_shared(&entity.cast())? || !actor_live.recognize(&entity.cast()) {
                continue;
            }
            let visible = iges_live(&entity, WHERE)?.blank_status() == 0;
            if !self.read_only_visible || visible {
                self.reader.theroots.push(entity.cast());
            }
        }
        Ok(self.reader.theroots.len() as i32)
    }
}

/// A reader inside the lock of its transfer: the provided transfers run on it and call each other
/// without taking the lock again.
struct LockedReader<'reader>(&'reader mut IgesControlReader);

impl XSControlReaderOps for LockedReader<'_> {
    fn xs_reader(&self) -> &XSControlReader {
        &self.0.reader
    }
    fn xs_reader_mut(&mut self) -> &mut XSControlReader {
        &mut self.0.reader
    }
    fn nb_roots_for_transfer(&mut self) -> Result<i32, Failure> {
        self.0.count_roots()
    }
    fn default_shape_fix_parameters(&self) -> DEShapeFixParameters {
        self.0.default_shape_fix_parameters()
    }
    fn default_shape_process_flags(&self) -> ShapeProcessOperationsFlags {
        self.0.default_shape_process_flags()
    }
}

/// Every transfer takes the lock of the format: its reading actor is one per process.
impl XSControlReaderOps for IgesControlReader {
    fn xs_reader(&self) -> &XSControlReader {
        &self.reader
    }
    fn xs_reader_mut(&mut self) -> &mut XSControlReader {
        &mut self.reader
    }
    fn read_file(&mut self, filename: &str) -> Result<IFSelectReturnStatus, Failure> {
        let _only_one = one_transfer_at_a_time();
        LockedReader(self).read_file(filename)
    }
    fn read_stream(
        &mut self,
        name: &str,
        istream: &mut dyn Read,
    ) -> Result<IFSelectReturnStatus, Failure> {
        let _only_one = one_transfer_at_a_time();
        LockedReader(self).read_stream(name, istream)
    }
    fn nb_roots_for_transfer(&mut self) -> Result<i32, Failure> {
        let _only_one = one_transfer_at_a_time();
        self.count_roots()
    }
    fn transfer_one_root(
        &mut self,
        num: i32,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let _only_one = self.locked_on_own_model()?;
        LockedReader(self).transfer_one_root(num, progress)
    }
    fn transfer_one(&mut self, num: i32, progress: &ProgressRange<'_>) -> Result<bool, Failure> {
        let _only_one = self.locked_on_own_model()?;
        LockedReader(self).transfer_one(num, progress)
    }
    fn transfer_entity(
        &mut self,
        start: &Handle<StandardTransient>,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let _only_one = self.locked_on_own_model()?;
        LockedReader(self).transfer_entity(start, progress)
    }
    fn transfer_list(
        &mut self,
        list: Option<&[Handle<StandardTransient>]>,
        progress: &ProgressRange<'_>,
    ) -> Result<i32, Failure> {
        let _only_one = self.locked_on_own_model()?;
        LockedReader(self).transfer_list(list, progress)
    }
    fn transfer_roots(&mut self, progress: &ProgressRange<'_>) -> Result<i32, Failure> {
        let _only_one = self.locked_on_own_model()?;
        LockedReader(self).transfer_roots(progress)
    }
    fn default_shape_fix_parameters(&self) -> DEShapeFixParameters {
        crate::de_iges::xs_shape_fix_parameters()
    }
    fn default_shape_process_flags(&self) -> ShapeProcessOperationsFlags {
        let mut flags = ShapeProcessOperationsFlags::default();
        flags.set(ShapeProcessOperation::FixShape);
        flags
    }
}
