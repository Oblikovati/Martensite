//! A transformer that finds the spline forms to convert.
use crate::iges_data::IgesDataIgesEntity;
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, SharedState, StandardTransient},
    tcollection::AsciiString,
};
use martensite_tkxsbase::{
    if_select::IFSelectTransformer,
    interface::{
        InterfaceCheckIterator, InterfaceCopyControl, InterfaceGraph, InterfaceInterfaceModel,
        InterfaceProtocol,
    },
};

#[derive(Default)]
struct SplineFields {
    thefound: bool,
    themap: Handle<dyn InterfaceCopyControl>,
}

/// Finds the entities of type 112 and 126 of a model. Their conversion is not done: when there
/// are some, the transformation fails with a message.
///
/// ```
/// use martensite_tkdeiges::iges_select::IgesSelectSplineToBSpline;
/// use martensite_tkxsbase::if_select::IFSelectTransformer;
/// let transformer = IgesSelectSplineToBSpline::new(true);
/// assert!(transformer.option_try_c2());
/// assert!(transformer.label().as_bytes().ends_with(b"continuity"));
/// ```
pub struct IgesSelectSplineToBSpline {
    transient: StandardTransient,
    thetryc2: bool,
    state: SharedState<SplineFields>,
}
impl_transient!(
    IgesSelectSplineToBSpline,
    "IgesSelectSplineToBSpline",
    dyn IFSelectTransformer,
    transient,
    interfaces: [dyn IFSelectTransformer]
);

impl IgesSelectSplineToBSpline {
    /// A transformer; `try_c2` would try to recover the continuity.
    pub fn new(try_c2: bool) -> Self {
        Self {
            transient: StandardTransient::default(),
            thetryc2: try_c2,
            state: SharedState::new(SplineFields::default()),
        }
    }
    /// Whether it would try to recover the continuity.
    pub fn option_try_c2(&self) -> bool {
        self.thetryc2
    }
}

impl IFSelectTransformer for IgesSelectSplineToBSpline {
    fn perform(
        &self,
        g: &InterfaceGraph,
        _protocol: &Handle<dyn InterfaceProtocol>,
        checks: &mut InterfaceCheckIterator,
        newmod: &mut Handle<dyn InterfaceInterfaceModel>,
    ) -> Result<bool, Failure> {
        let mut found = false;
        for i in 1..=g.size() {
            if let Some(entity) = g.entity(i)?.cast::<IgesDataIgesEntity>().get() {
                found |= matches!(entity.type_number(), 112 | 126);
            }
        }
        {
            let mut state = self.state.write();
            state.thefound = found;
            state.themap.nullify();
        }
        newmod.nullify();
        if !found {
            return Ok(true);
        }
        if let Some(check) = checks.c_check(0).get() {
            check.add_fail_text("Spline to BSpline : not yet implemented", "");
        }
        Ok(false)
    }
    fn updated(
        &self,
        entfrom: &Handle<StandardTransient>,
        entto: &mut Handle<StandardTransient>,
    ) -> bool {
        let state = self.state.read();
        if !state.thefound {
            *entto = entfrom.clone();
            return true;
        }
        state
            .themap
            .get()
            .is_some_and(|map| map.search(entfrom, entto).unwrap_or(false))
    }
    fn label(&self) -> AsciiString {
        AsciiString::new(if self.thetryc2 {
            "Convert Spline Forms to BSpline, trying to recover C1-C2 continuity"
        } else {
            "Convert Spline Forms to BSpline"
        })
    }
}
