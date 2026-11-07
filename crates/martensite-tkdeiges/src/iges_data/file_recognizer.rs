//! The recognition of an entity's class from its type and form.
use super::{IgesDataIgesEntity, IgesDataIgesType};
use martensite_tkernel::{
    impl_transient_interface,
    standard::{Failure, Handle, SharedState, StandardTransient, Transient, exceptions},
};

/// The private fields of a recognizer, which embeds them.
#[derive(Default)]
pub struct IgesDataFileRecognizerBase {
    state: SharedState<(
        Handle<IgesDataIgesEntity>,
        Handle<dyn IgesDataFileRecognizer>,
    )>,
}

/// A recognizer: `eval` calls `set_ok` with an empty entity of the class it recognises.
///
/// ```
/// use martensite_tkdeiges::iges_data::IgesDataFileRecognizer;
/// fn found(recognizer: &dyn IgesDataFileRecognizer) -> bool { recognizer.result().is_ok() }
/// ```
pub trait IgesDataFileRecognizer: Transient {
    /// The embedded private fields.
    fn recognizer_base(&self) -> &IgesDataFileRecognizerBase;
    /// The recognition proper to this class.
    fn eval(&self, akey: &IgesDataIgesType);
    /// Its own `eval`, then the one of the recognizer added; true when one gave a result.
    fn evaluate(&self, akey: &IgesDataIgesType, res: &mut Handle<IgesDataIgesEntity>) -> bool {
        self.set_ko();
        self.eval(akey);
        let (found, next) = {
            let state = self.recognizer_base().state.read();
            (state.0.clone(), state.1.clone())
        };
        if found.is_some() {
            *res = found;
            return true;
        }
        next.get().is_some_and(|live| live.evaluate(akey, res))
    }
    /// The result of the last evaluation.
    ///
    /// # Errors
    /// A no such object failure when the evaluation gave nothing.
    fn result(&self) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        let (found, next) = {
            let state = self.recognizer_base().state.read();
            (state.0.clone(), state.1.clone())
        };
        if found.is_some() {
            return Ok(found);
        }
        match next.get() {
            Some(live) => live.result(),
            None => Err(exceptions::no_such_object(
                "Recognizer evaluation has failed; expected a call of Evaluate to succeed first",
            )),
        }
    }
    /// Adds a recognizer to try when this one finds nothing.
    fn add(&self, reco: &Handle<dyn IgesDataFileRecognizer>) {
        self.recognizer_base().state.write().1 = reco.clone();
    }
    /// Records the result of the recognition.
    fn set_ok(&self, aresult: &Handle<IgesDataIgesEntity>) {
        self.recognizer_base().state.write().0 = aresult.clone();
    }
    /// Records that the recognition gives no result.
    fn set_ko(&self) {
        self.recognizer_base().state.write().0.nullify();
    }
}
impl_transient_interface!(
    dyn IgesDataFileRecognizer,
    "IgesDataFileRecognizer",
    StandardTransient
);
