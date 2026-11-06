//! Iterator wrappers process its current record without advancing or replacing it.
use super::TopOpeBRepFacesFiller;
use crate::top_ope_brep::TopOpeBRepVPointInterIterator;
use crate::top_ope_brep_ds::TopOpeBRepDSTransition;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;

impl TopOpeBRepFacesFiller<'_> {
    /// Processes the iterator's current restriction point without advancing; `filler.process_vpion_r(&mut iterator,&transition,&face,1)?`.
    pub fn process_vpion_r(
        &mut self,
        iterator: &mut TopOpeBRepVPointInterIterator<'_>,
        transition: &TopOpeBRepDSTransition,
        face: &Shape,
        side: i32,
    ) -> Result<(), Failure> {
        let point = iterator.current_vp()?.clone();
        self.process_vpon_r(&point, transition, face, side, iterator.line_inter_mut()?)
    }

    /// Processes the current ordinary point, retaining iterator position; `filler.process_vpinoton_r(&mut iterator)?`.
    pub fn process_vpinoton_r(
        &mut self,
        iterator: &mut TopOpeBRepVPointInterIterator<'_>,
    ) -> Result<(), Failure> {
        let point = iterator.current_vp()?.clone();
        self.process_vpnoton_r(&point, iterator.line_inter_mut()?)
    }
}
