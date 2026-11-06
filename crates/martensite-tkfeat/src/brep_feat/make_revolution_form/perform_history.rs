//! The history of a rib of revolution through the cut by the planes at its heights: the
//! end faces become the faces of the planes, the other faces their pieces; the sliding
//! faces are glued on the pieces of the faces swept from their edges.
use super::BRepFeatMakeRevolutionForm;
use crate::errors::no_such_object;
use crate::loc_ope::RevolutionForm;
use martensite_tkbo::brep_algo_api::BRepAlgoAPICut;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{Face, Shape};
use martensite_tkbrep::toptools::ShapeMapHasher;
use martensite_tkernel::ncollection::{DataMap, List};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;

/// The list of the shapes the cut modified `s` into.
fn modified(tr_p: &BRepAlgoAPICut, s: &Shape) -> Result<List<Shape>, Failure> {
    Ok(tr_p.modified(s)?.into_iter().collect())
}

impl BRepFeatMakeRevolutionForm {
    /// The history of the key `orig` (first descendant `sh`) through the cut: the end
    /// wires re-bound to the pieces of the planes, `orig` to the pieces of its face.
    fn cut_key(
        &mut self,
        tr_p: &BRepAlgoAPICut,
        vrai_form: &Shape,
        (f1, f2): (&Face, &Face),
        orig: &Shape,
        sh: &Shape,
    ) -> Result<(), Failure> {
        for fac in Explorer::new(vrai_form, ShapeEnum::Face, ShapeEnum::Shape)? {
            let thew = martensite_tkbrep::topods::wire(
                Explorer::new(&fac, ShapeEnum::Wire, ShapeEnum::Shape)?.current(),
            )?;
            let r = &mut self.rib;
            if thew.is_same(&r.f_shape) {
                let desfaces = modified(tr_p, f2)?;
                *r.map.change_find(&r.f_shape.clone())? = desfaces;
                continue;
            } else if thew.is_same(&r.l_shape) {
                let desfaces = modified(tr_p, f1)?;
                *r.map.change_find(&r.l_shape.clone())? = desfaces;
                continue;
            }
            if fac.is_same(sh) && !tr_p.is_deleted(&fac)? {
                let desfaces = modified(tr_p, &fac)?;
                if !desfaces.is_empty() {
                    *r.map.change_find(orig)? = desfaces;
                    break;
                }
            }
        }
        Ok(())
    }

    /// The history of the faces through the cut of the revolution by the planes (each
    /// key followed from its first descendant before the cut).
    pub(super) fn cut_history(
        &mut self,
        tr_p: &BRepAlgoAPICut,
        vrai_form: &Shape,
        planes: (&Face, &Face),
    ) -> Result<(), Failure> {
        let keys: Vec<Shape> = self.rib.map.items().map(|(k, _)| k.clone()).collect();
        for orig in keys {
            let Some(sh) = self.rib.map.find(&orig)?.first().cloned() else {
                continue;
            };
            self.cut_key(tr_p, vrai_form, planes, &orig, &sh)?;
        }
        Ok(())
    }

    /// The first piece of the
    /// face swept from each sliding edge glued on its sliding face.
    pub(super) fn set_glued_faces(
        &mut self,
        the_prism: &RevolutionForm,
        sliding_map: &DataMap<Shape, List<Shape>, ShapeMapHasher>,
    ) -> Result<(), Failure> {
        for (fac, ledg) in self.slface.items() {
            for e in ledg.iter() {
                let gfac = the_prism.shapes(e)?;
                // in the reference a list of another length is only reported in debug
                for (ff, lfaces) in sliding_map.items() {
                    let Some(fff) = lfaces.first() else {
                        continue;
                    };
                    let first = gfac.first().ok_or_else(|| {
                        no_such_object("SetGluedFaces", "no face swept from the edge")
                    })?;
                    if first.is_same(ff) {
                        self.rib.glued_f.bind(fff.clone(), fac.clone());
                    }
                }
            }
        }
        Ok(())
    }
}
