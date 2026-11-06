//! `IgesToBRepTopoSurface`: surface entities as faces. This file holds the state, the choice among
//! the kinds, and the basic, offset, trimmed and bounded surfaces.
use super::{IgesToBRep, IgesToBRepBasicSurface, IgesToBRepCurveAndSurface, IgesToBRepTopoCurve};
use crate::iges_basic::IgesBasicSingleParent;
use crate::iges_data::{IgesDataIgesEntity, IgesDataToolLocation, iges_live};
use crate::iges_geom::{
    IgesGeomBoundedSurface, IgesGeomOffsetSurface, IgesGeomPlane, IgesGeomRuledSurface,
    IgesGeomSurfaceOfRevolution, IgesGeomTabulatedCylinder, IgesGeomTrimmedSurface,
};
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::brep_tools;
use martensite_tkbrep::topods::{self, Face, Shape, ShapeIterator};
use martensite_tkernel::message::Msg;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, exceptions};
use martensite_tkg3d::geom::{
    GeomOffsetSurface, GeomPlane, GeomRectangularTrimmedSurface, GeomSurface,
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::geom_abs::Shape as Continuity;
use martensite_tkmath::gp::Transform3;
use martensite_tkmath::toploc::Location;
use martensite_tkshhealing::shape_algo::ShapeAlgo;
use martensite_tkshhealing::shape_construct::ShapeConstructSurfaceBounds;
use martensite_tktopalgo::brep_lib::BRepLibMakeFace;
use std::ops::{Deref, DerefMut};

const WHERE: &str = "IgesToBRepTopoSurface";

/// Translates surface entities.
///
/// ```
/// use martensite_tkdeiges::iges_to_brep::IgesToBRepTopoSurface;
/// let tool = IgesToBRepTopoSurface::new();
/// assert_eq!(tool.get_eps_geom(), 1.0e-4);
/// ```
#[derive(Clone, Default)]
pub struct IgesToBRepTopoSurface {
    base: IgesToBRepCurveAndSurface,
    pub(super) u_length: f64,
}

impl Deref for IgesToBRepTopoSurface {
    type Target = IgesToBRepCurveAndSurface;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for IgesToBRepTopoSurface {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

/// The single face of a face or of a shell of one face; `None` for a shell of another count.
pub(super) fn face_of_shape(shape: &Shape) -> Result<Option<Face>, Failure> {
    if shape.shape_type()? == ShapeEnum::Face {
        return topods::face(shape).map(Some);
    }
    let mut iterator = ShapeIterator::new(shape, true, true);
    let (mut count, mut last) = (0, Face::default());
    while iterator.more() {
        count += 1;
        last = topods::face(iterator.value()?)?;
        iterator.next();
    }
    Ok((count == 1).then_some(last))
}

impl IgesToBRepTopoSurface {
    /// A tool at the default settings; `IgesToBRepTopoSurface::new()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// A tool with the settings of `base`; `IgesToBRepTopoSurface::with_base(&base)`.
    pub fn with_base(base: &IgesToBRepCurveAndSurface) -> Self {
        Self {
            base: base.clone(),
            u_length: 0.0,
        }
    }
    /// A tool with the given settings;
    /// `IgesToBRepTopoSurface::with_settings(1e-4, 1e-6, 1e-4, true, false, false)`.
    pub fn with_settings(
        eps: f64,
        eps_coeff: f64,
        eps_geom: f64,
        mode: bool,
        mode_approx: bool,
        optimized: bool,
    ) -> Self {
        Self::with_base(&IgesToBRepCurveAndSurface::with_settings(
            eps,
            eps_coeff,
            eps_geom,
            mode,
            mode_approx,
            optimized,
        ))
    }
    /// The curve tool of this surface tool.
    pub(super) fn curve_tool(&self) -> IgesToBRepTopoCurve {
        IgesToBRepTopoCurve::with_base(self)
    }
    /// The message that an entity of a surface did not give a shape.
    pub(super) fn label_message(
        &self,
        entity: &Handle<IgesDataIgesEntity>,
        what: &[u8],
    ) -> Result<Msg, Failure> {
        self.base.label_message(entity, what)
    }
    /// A fail for a part that did not give a shape.
    pub(super) fn label_fail<T: ?Sized + RuntimeTyped>(
        &self,
        start: &Handle<T>,
        entity: &Handle<IgesDataIgesEntity>,
        what: &[u8],
    ) -> Result<(), Failure> {
        self.send_fail(start, &self.label_message(entity, what)?);
        Ok(())
    }
    /// A shape moved by the location of its entity, in the file unit; a warning when it is not a
    /// similarity.
    pub(super) fn moved_to_location(
        &mut self,
        start: &Handle<IgesDataIgesEntity>,
        shape: &mut Shape,
    ) -> Result<(), Failure> {
        let entity = iges_live(start, WHERE)?;
        if !entity.has_transf() {
            return Ok(());
        }
        let mut similarity = Transform3::default();
        self.set_epsilon(1.0e-4);
        let location = entity.compound_location()?;
        let unit = self.get_unit_factor();
        if IgesDataToolLocation::convert_location(
            self.get_epsilon(),
            &location,
            &mut similarity,
            unit,
        )? {
            shape.move_by(&Location::new(similarity), false)?;
        } else {
            self.warn_key(start, "IGES_1035");
        }
        Ok(())
    }
    /// The shape of any surface entity, or the result it already has;
    /// `tool.transfer_topo_surface(&entity)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the shape.
    pub fn transfer_topo_surface(
        &mut self,
        start: &Handle<IgesDataIgesEntity>,
    ) -> Result<Shape, Failure> {
        self.u_length = 1.0;
        if start.is_null() {
            self.fail_key(start, "IGES_1005");
            return Ok(Shape::default());
        }
        if self.has_shape_result(start)? {
            return self.get_shape_result(start);
        }
        let result = self.dispatch_topo_surface(start)?;
        self.set_shape_result(start, &result)?;
        Ok(result)
    }
    fn dispatch_topo_surface(
        &mut self,
        start: &Handle<IgesDataIgesEntity>,
    ) -> Result<Shape, Failure> {
        if IgesToBRep::is_basic_surface(start) {
            return self.transfer_topo_basic_surface(start);
        }
        let trimmed = start.cast::<IgesGeomTrimmedSurface>();
        if trimmed.is_some() {
            return self.transfer_trimmed_surface(&trimmed);
        }
        let revolution = start.cast::<IgesGeomSurfaceOfRevolution>();
        if revolution.is_some() {
            return self.transfer_surface_of_revolution(&revolution);
        }
        let tabulated = start.cast::<IgesGeomTabulatedCylinder>();
        if tabulated.is_some() {
            return self.transfer_tabulated_cylinder(&tabulated);
        }
        let ruled = start.cast::<IgesGeomRuledSurface>();
        if ruled.is_some() {
            return self.transfer_ruled_surface(&ruled);
        }
        let plane = start.cast::<IgesGeomPlane>();
        if plane.is_some() {
            return self.transfer_plane(&plane);
        }
        let bounded = start.cast::<IgesGeomBoundedSurface>();
        if bounded.is_some() {
            return self.transfer_bounded_surface(&bounded);
        }
        let offset = start.cast::<IgesGeomOffsetSurface>();
        if offset.is_some() {
            return self.transfer_offset_surface(&offset);
        }
        // A limit, planes only.
        let parent = start.cast::<IgesBasicSingleParent>();
        if parent.is_some() {
            return self.transfer_perforate(&parent);
        }
        Ok(Shape::default())
    }
    /// The face of a basic surface entity, a warning for a surface under C1;
    /// `tool.transfer_topo_basic_surface(&entity)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the face.
    pub fn transfer_topo_basic_surface(
        &mut self,
        start: &Handle<IgesDataIgesEntity>,
    ) -> Result<Shape, Failure> {
        if start.is_null() {
            self.fail_key(start, "IGES_1005");
            return Ok(Shape::default());
        }
        if !IgesToBRep::is_basic_surface(start) {
            return Ok(Shape::default());
        }
        let surface = IgesToBRepBasicSurface::with_base(self).transfer_basic_surface(start)?;
        let Some(live) = surface.get() else {
            return Ok(Shape::default());
        };
        // A surface under C1 is told about.
        if live.continuity() < Continuity::C1 {
            self.warn_key(start, "IGES_1250");
        }
        let mut result = if surface.cast::<GeomPlane>().is_some() {
            let builder = BRepBuilder::default();
            let mut plane = Face::default();
            builder.make_face(&mut plane);
            builder.update_face(
                &plane,
                &surface,
                &Location::default(),
                precision::confusion(),
            )?;
            plane.into_shape()
        } else {
            BRepLibMakeFace::from_surface(&surface, precision::confusion())?
                .face()?
                .into_shape()
        };
        self.moved_to_location(start, &mut result)?;
        Ok(result)
    }
    /// The first face of the shape of a basis surface; `None` after a fail.
    fn offset_basis_face(
        &self,
        start: &Handle<IgesGeomOffsetSurface>,
        basis: &Handle<IgesDataIgesEntity>,
        shape: &Shape,
    ) -> Result<Option<Face>, Failure> {
        let kind = shape.shape_type()?;
        if kind == ShapeEnum::Face {
            return topods::face(shape).map(Some);
        }
        if kind == ShapeEnum::Shell {
            let iterator = ShapeIterator::new(shape, true, true);
            if iterator.more() {
                let mut message = Msg::new();
                message.set_text(b"The First Surface only will be transferred.");
                self.send_warning(start, &message);
                return topods::face(iterator.value()?).map(Some);
            }
        }
        self.label_fail(start, basis, b"basis surface")?;
        Ok(None)
    }
    /// A surface of C1 under the face, with a warning if a change was needed; `None` after a fail.
    fn c1_support(
        &self,
        start: &Handle<IgesGeomOffsetSurface>,
        face: &Face,
        distance: f64,
    ) -> Result<Option<Shape>, Failure> {
        let support = BRepTool::surface(face)?;
        if iges_live(&support, WHERE)?.continuity() != Continuity::C0 {
            return Ok(Some(face.clone().into_shape()));
        }
        let container = ShapeAlgo::algo_container();
        let shape = iges_live(&container, WHERE)?.c0_shape_to_c1_shape(face, distance.abs())?;
        if shape.shape_type()? != ShapeEnum::Face {
            self.fail_key(start, "IGES_1266");
            return Ok(None);
        }
        let support = BRepTool::surface(&topods::face(&shape)?)?;
        if iges_live(&support, WHERE)?.continuity() == Continuity::C0 {
            self.fail_key(start, "IGES_1266");
            return Ok(None);
        }
        self.warn_key(start, "IGES_1267");
        Ok(Some(shape))
    }
    /// The support of the offset, as a bounded B-spline if it is not bounded; `None` after a fail.
    fn bounded_support(
        &self,
        start: &Handle<IgesGeomOffsetSurface>,
        face: &Face,
        support: Handle<dyn GeomSurface>,
    ) -> Result<Option<Handle<dyn GeomSurface>>, Failure> {
        let bounds = iges_live(&support, WHERE)?.bounds();
        let infinite = [bounds.0, bounds.1, bounds.2, bounds.3]
            .iter()
            .any(|value| precision::is_infinite(*value));
        if !infinite {
            return Ok(Some(support));
        }
        let uv = brep_tools::uv_bounds(face)?;
        let trimmed =
            GeomRectangularTrimmedSurface::new(&support, (uv.0, uv.1), (uv.2, uv.3), (true, true))?;
        let container = ShapeAlgo::algo_container();
        let bspline = iges_live(&container, WHERE)?.convert_surface_to_bspline(
            &Handle::new(trimmed).cast(),
            ShapeConstructSurfaceBounds {
                uf: uv.0,
                ul: uv.1,
                vf: uv.2,
                vl: uv.3,
            },
        )?;
        let usable = bspline
            .get()
            .is_some_and(|live| live.continuity() != Continuity::C0);
        if !usable {
            self.fail_key(start, "IGES_1265");
            return Ok(None);
        }
        Ok(Some(bspline.cast()))
    }
    /// The face of an offset surface. An offset of an offset has no translation;
    /// `tool.transfer_offset_surface(&surface)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the face.
    pub fn transfer_offset_surface(
        &mut self,
        start: &Handle<IgesGeomOffsetSurface>,
    ) -> Result<Shape, Failure> {
        let Some(live) = start.get() else {
            self.fail_key(start, "IGES_1005");
            return Ok(Shape::default());
        };
        let basis = live.surface();
        if basis.is_null() || !IgesToBRep::is_topo_surface(&basis) {
            self.fail_key(start, "XSTEP_164");
            return Ok(Shape::default());
        }
        let shape = self.transfer_topo_surface(&basis)?;
        if shape.is_null() {
            self.label_fail(start, &basis, b"basis surface")?;
            return Ok(Shape::default());
        }
        let Some(face) = self.offset_basis_face(start, &basis, &shape)? else {
            return Ok(Shape::default());
        };
        if BRepTool::surface(&face)?
            .cast::<GeomOffsetSurface>()
            .is_some()
        {
            // The offset value of the nested surface is read from a surface not yet made.
            return Err(exceptions::null_object(&format!(
                "{WHERE}: an offset surface over the offset surface of entity {}",
                basis.type_number()
            )));
        }
        let distance = live.distance() * self.get_unit_factor();
        let Some(current) = self.c1_support(start, &face, distance)? else {
            return Ok(Shape::default());
        };
        let support = BRepTool::surface(&topods::face(&current)?)?;
        let Some(support) = self.bounded_support(start, &face, support)? else {
            return Ok(current);
        };
        let offset: Handle<dyn GeomSurface> =
            Handle::new(GeomOffsetSurface::new(&support, distance, false)?).cast();
        let maker = BRepLibMakeFace::from_surface(&offset, precision::confusion())?;
        if !maker.is_done() {
            self.fail_key(start, "IGES_1265");
            return Ok(current);
        }
        let mut result = maker.face()?.into_shape();
        self.moved_to_location(&start.cast(), &mut result)?;
        Ok(result)
    }
}
