//! The modifier that removes the parametric or the 3D curves of faces.
use super::IgesSelectModelModifier;
use super::modifier_bases::{iges_model_modifier, modifier_base};
use crate::iges_basic::IgesBasicHArray1OfHArray1OfIgesEntity;
use crate::iges_data::{IgesDataIgesEntity, IgesDataIgesModel, iges_array1_set, iges_new_array1};
use crate::iges_geom::{
    IgesGeomBoundary, IgesGeomBoundedSurface, IgesGeomCurveOnSurface, IgesGeomTrimmedSurface,
};
use martensite_tkernel::{
    ncollection::HArray1,
    standard::{Failure, Handle, SharedState, StandardTransient},
    tcollection::AsciiString,
};
use martensite_tkxsbase::{if_select::IFSelectContextModif, interface::InterfaceCopyTool};

iges_model_modifier! {
    /// Removes the parametric curves of faces when `uv`, else the 3D curves, where both are given.
    ///
    /// ```
    /// use martensite_tkdeiges::iges_select::IgesSelectRemoveCurves;
    /// use martensite_tkxsbase::if_select::IFSelectGeneralModifier;
    /// assert_eq!(IgesSelectRemoveCurves::new(true).label().as_bytes(), b"Remove Curves UV on Face");
    /// ```
    IgesSelectRemoveCurves, "IgesSelectRemoveCurves",
    { the_uv: bool },
    self => AsciiString::new(if *self.the_uv.read() {
        "Remove Curves UV on Face"
    } else {
        "Remove Curves 3D on Face"
    })
}

impl IgesSelectRemoveCurves {
    /// The modifier.
    pub fn new(uv: bool) -> Self {
        Self {
            transient: StandardTransient::default(),
            base: modifier_base(true),
            the_uv: SharedState::new(uv),
        }
    }
}

/// Removes the curve of a curve on surface; false when one is missing.
fn edit_curve_on_surface(cons: &IgesGeomCurveOnSurface, uv: bool) -> Result<bool, Failure> {
    let (mut cuv, mut c3d) = (cons.curve_uv(), cons.curve3d());
    let mut pref = cons.preference_mode();
    if uv && c3d.is_some() {
        if cuv.is_null() {
            return Ok(false);
        }
        cuv.nullify();
        pref = match pref {
            1 => 0,
            3 => 2,
            other => other,
        };
    } else if cuv.is_some() {
        if c3d.is_null() {
            return Ok(false);
        }
        c3d.nullify();
        pref = match pref {
            2 => 0,
            3 => 1,
            other => other,
        };
    }
    cons.init(cons.creation_mode(), &cons.surface(), &cuv, &c3d, pref);
    Ok(true)
}

/// Removes the curves of a boundary. The arrays are filled only for what is kept, as a boundary
/// without any change stays untouched.
fn edit_boundary(bndy: &IgesGeomBoundary, uv: bool) -> Result<bool, Failure> {
    let nb = bndy.nb_model_space_curves()?;
    if nb == 0 {
        return Ok(false);
    }
    let arc3d: Handle<HArray1<Handle<IgesDataIgesEntity>>> = iges_new_array1(1, nb)?;
    let arcuv = Handle::new(IgesBasicHArray1OfHArray1OfIgesEntity::with_bounds(1, nb)?);
    let sens = iges_new_array1(1, nb)?;
    let mut res = false;
    for rank in 1..=nb {
        iges_array1_set(&sens, rank, bndy.sense(rank)?, "remove curves")?;
        let (cuv, c3d) = (bndy.parameter_curves(rank)?, bndy.model_space_curve(rank)?);
        if cuv.is_null() || c3d.is_null() {
            continue;
        }
        if uv {
            arcuv.set_value(rank, &Handle::default())?;
        } else {
            iges_array1_set(&arc3d, rank, Handle::default(), "remove curves")?;
            res = true;
        }
    }
    let pref = match (uv, bndy.preference_type()) {
        (true, 2) | (false, 1) => 0,
        (true, 3) => 1,
        (false, 3) => 2,
        (_, other) => other,
    };
    if res {
        bndy.init(
            bndy.boundary_type(),
            pref,
            &bndy.surface(),
            &arc3d,
            &sens,
            &arcuv,
        )?;
    }
    Ok(res)
}

/// Removes the curves an entity holds; true when something changed.
fn edit(ent: &Handle<StandardTransient>, uv: bool) -> Result<bool, Failure> {
    if let Some(trimmed) = ent.cast::<IgesGeomTrimmedSurface>().get() {
        let mut res = edit(&trimmed.outer_contour().cast(), uv)?;
        for rank in 1..=trimmed.nb_inner_contours() {
            res |= edit(&trimmed.inner_contour(rank)?.cast(), uv)?;
        }
        return Ok(res);
    }
    if let Some(bounded) = ent.cast::<IgesGeomBoundedSurface>().get() {
        let mut res = false;
        for rank in 1..=bounded.nb_boundaries() {
            res |= edit(&bounded.boundary(rank)?.cast(), uv)?;
        }
        return Ok(res);
    }
    if let Some(cons) = ent.cast::<IgesGeomCurveOnSurface>().get() {
        return edit_curve_on_surface(cons, uv);
    }
    if let Some(boundary) = ent.cast::<IgesGeomBoundary>().get() {
        return edit_boundary(boundary, uv);
    }
    Ok(false)
}

impl IgesSelectModelModifier for IgesSelectRemoveCurves {
    fn performing(
        &self,
        ctx: &mut IFSelectContextModif,
        _target: &Handle<IgesDataIgesModel>,
        _tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let uv = *self.the_uv.read();
        ctx.start();
        while ctx.more() {
            if edit(&ctx.value_result()?, uv)? {
                ctx.trace("")?;
            }
            ctx.next();
        }
        Ok(())
    }
}
