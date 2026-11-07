//! `BRepToIgesBRShell`: faces as trimmed surfaces, shells as groups of them.
use super::br_shell_reversed::forward_copy;
use super::{BRepToIgesBREntity, BRepToIgesBRWire, BRepToIgesOriginMap, entity_array};
use crate::geom_to_iges::GeomToIgesGeomSurface;
use crate::iges_basic::IgesBasicGroup;
use crate::iges_data::{IgesDataIgesEntity, iges_array1_set, iges_live, iges_new_array1};
use crate::iges_geom::{IgesGeomCurveOnSurface, IgesGeomTrimmedSurface};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_tools;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Face, Shape, Shell};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomConicalSurface, GeomCylindricalSurface, GeomPlane};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkshhealing::shape_algo::ShapeAlgo;
use std::ops::{Deref, DerefMut};

const WHERE: &str = "BRepToIgesBRShell";

/// Translates faces and shells.
///
/// ```
/// use martensite_tkbrep::topods::Shape;
/// use martensite_tkdeiges::brep_to_iges::BRepToIgesBRShell;
/// use martensite_tkernel::message::{ProgressRange, ProgressScope};
/// let tool = BRepToIgesBRShell::new();
/// assert!(tool.transfer_shell_shape(&Shape::default(), &ProgressRange::default()).unwrap().is_null());
/// ```
#[derive(Clone, Default)]
pub struct BRepToIgesBRShell {
    base: BRepToIgesBREntity,
}

impl Deref for BRepToIgesBRShell {
    type Target = BRepToIgesBREntity;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for BRepToIgesBRShell {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

/// The preferred representation of a curve on surface from what it has; a wire with neither
/// keeps the previous one, as all share the variable.
fn preference(
    curve_3d: &Handle<IgesDataIgesEntity>,
    curve_2d: &Handle<IgesDataIgesEntity>,
    previous: i32,
) -> i32 {
    match (curve_3d.is_some(), curve_2d.is_some()) {
        (true, true) => 3,
        (true, false) => 2,
        (false, true) => 1,
        (false, false) => previous,
    }
}

impl BRepToIgesBRShell {
    /// A tool at the initial state; `BRepToIgesBRShell::new()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// A tool with the process and the model of `base`; `BRepToIgesBRShell::with_entity(&base)`.
    pub fn with_entity(base: &BRepToIgesBREntity) -> Self {
        Self { base: base.clone() }
    }
    /// The entity of a face or a shell; `tool.transfer_shell_shape(&shape, &range)?`.
    ///
    /// # Errors
    /// A failure of the geometry of the shape.
    pub fn transfer_shell_shape(
        &self,
        start: &Shape,
        range: &ProgressRange,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        if start.is_null() {
            return Ok(Handle::default());
        }
        match start.shape_type()? {
            ShapeEnum::Face => self.transfer_face(&topods::face(start)?, range),
            ShapeEnum::Shell => self.transfer_shell(&topods::shell(start)?, range),
            _ => Ok(Handle::default()),
        }
    }
    /// The curves on surface of the free wires and edges of a face.
    fn inner_curves(
        &self,
        start: &Face,
        (face, outer): (&Face, &Shape),
        tool: &BRepToIgesBRWire,
        (origins, surface, length): (&BRepToIgesOriginMap, &Handle<IgesDataIgesEntity>, f64),
        state: &mut (Handle<IgesDataIgesEntity>, i32),
    ) -> Result<Vec<Handle<IgesGeomCurveOnSurface>>, Failure> {
        let mut curves = Vec::new();
        let mut wires = Explorer::new(face, ShapeEnum::Wire, ShapeEnum::Shape)?;
        while wires.more() {
            let wire = topods::wire(wires.current())?;
            wires.next()?;
            if wire.is_null() {
                self.add_warning(start, " an Wire is a null entity")?;
            } else if !wire.is_same(outer) {
                let curve_3d =
                    tool.transfer_wire_on_face(&wire, face, origins, &mut state.0, length)?;
                state.1 = preference(&curve_3d, &state.0, state.1);
                let on_surface = IgesGeomCurveOnSurface::new();
                on_surface.init(0, surface, &state.0, &curve_3d, state.1);
                curves.push(Handle::new(on_surface));
            }
        }
        let mut edges = Explorer::new(face, ShapeEnum::Edge, ShapeEnum::Wire)?;
        while edges.more() {
            let edge = topods::edge(edges.current())?;
            edges.next()?;
            if edge.is_null() {
                self.add_warning(start, " an Edge is a null entity")?;
                continue;
            }
            let curve_3d = tool.transfer_edge(&edge, origins, false)?;
            let curve_2d = tool.transfer_edge_on_face(&edge, face, origins, length, false)?;
            state.1 = preference(&curve_3d, &curve_2d, state.1);
            let on_surface = IgesGeomCurveOnSurface::new();
            on_surface.init(0, surface, &curve_2d, &curve_3d, state.1);
            curves.push(Handle::new(on_surface));
        }
        Ok(curves)
    }
    /// The entity of the surface of a face and the length of its generatrix; null for a face whose
    /// surface has no translation, with a warning.
    fn face_surface_entity(
        &self,
        start: &Face,
        face: &Face,
    ) -> Result<(Handle<IgesDataIgesEntity>, f64), Failure> {
        let surface = BRepTool::surface(face)?;
        if surface.is_null() {
            return Ok((Handle::default(), 1.0));
        }
        let bounds = brep_tools::uv_bounds(face)?;
        let mut tool = GeomToIgesGeomSurface::new();
        tool.set_model(&self.get_model())?;
        let entity = tool.transfer_surface(&surface, (bounds.0, bounds.1, bounds.2, bounds.3))?;
        if entity.is_null() {
            self.add_warning(start, "the basic surface is a null entity")?;
        }
        Ok((entity, tool.length()))
    }
    /// The trimmed surface of a face; a reversed face goes through a copy on the reversed surface;
    /// `tool.transfer_face(&face, &range)?`.
    ///
    /// # Errors
    /// A failure of the geometry of the face or of the binding of the result.
    pub fn transfer_face(
        &self,
        start: &Face,
        _range: &ProgressRange,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        if start.is_null() {
            return Ok(Handle::default());
        }
        let mut origins = BRepToIgesOriginMap::default();
        let face = if start.orientation() == Orientation::Reversed {
            forward_copy(start, &mut origins)?
        } else {
            start.clone()
        };
        let surface = BRepTool::surface(&face)?;
        let (entity, length) = self.face_surface_entity(start, &face)?;
        if surface.is_some() && entity.is_null() {
            return Ok(Handle::default());
        }
        let tool = BRepToIgesBRWire::with_entity(self);
        let container = ShapeAlgo::algo_container();
        let outer = iges_live(&container, WHERE)?.outer_wire(&face)?;
        let mut state = (Handle::default(), 0);
        let outer_curve = IgesGeomCurveOnSurface::new();
        if !outer.is_null() {
            let curve_3d =
                tool.transfer_wire_on_face(&outer, &face, &origins, &mut state.0, length)?;
            state.1 = preference(&curve_3d, &state.0, state.1);
            outer_curve.init(0, &entity, &state.0, &curve_3d, state.1);
        }
        let outer_shape = outer.clone().into_shape();
        let inner = self.inner_curves(
            start,
            (&face, &outer_shape),
            &tool,
            (&origins, &entity, length),
            &mut state,
        )?;
        let holes = curve_array(&inner)?;
        let mut whole = BRepTool::natural_restriction(start)?;
        if surface.cast::<GeomPlane>().is_some()
            || surface.cast::<GeomCylindricalSurface>().is_some()
            || surface.cast::<GeomConicalSurface>().is_some()
        {
            whole = false;
        }
        let trimmed = IgesGeomTrimmedSurface::new();
        if whole {
            trimmed.init(&entity, 0, &Handle::default(), &holes)?;
        } else {
            trimmed.init(&entity, 1, &Handle::new(outer_curve), &holes)?;
        }
        let result: Handle<IgesDataIgesEntity> = Handle::new(trimmed).cast();
        self.set_shape_result(start, &result.cast())?;
        Ok(result)
    }
    /// The entity of a shell: its only face, or a group; `tool.transfer_shell(&shell, &range)?`.
    ///
    /// # Errors
    /// A failure of the geometry of a face or of the binding of the result.
    pub fn transfer_shell(
        &self,
        start: &Shell,
        range: &ProgressRange,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        if start.is_null() {
            return Ok(Handle::default());
        }
        let count = super::count_subshapes(start, ShapeEnum::Face, ShapeEnum::Shape)?;
        let scope = ProgressScope::unnamed(range, f64::from(count), false);
        let mut faces = Vec::new();
        let mut last = Handle::default();
        let mut explorer = Explorer::new(start, ShapeEnum::Face, ShapeEnum::Shape)?;
        while explorer.more() && scope.more() {
            let step = scope.next(1.0);
            let face = topods::face(explorer.current())?;
            explorer.next()?;
            if face.is_null() {
                self.add_warning(start, " a Face is a null entity")?;
                continue;
            }
            last = self.transfer_face(&face, &step)?;
            if last.is_some() {
                faces.push(last.clone());
            }
        }
        let result = if faces.len() == 1 {
            last
        } else {
            let group = IgesBasicGroup::new();
            group.init(&entity_array(&faces)?)?;
            Handle::new(group).cast()
        };
        self.set_shape_result(start, &result.cast())?;
        Ok(result)
    }
}

/// An array from 1 of curves on surface, null without one.
fn curve_array(
    curves: &[Handle<IgesGeomCurveOnSurface>],
) -> Result<Handle<HArray1<Handle<IgesGeomCurveOnSurface>>>, Failure> {
    if curves.is_empty() {
        return Ok(Handle::default());
    }
    let array = iges_new_array1(1, curves.len() as i32)?;
    for (rank, curve) in (1..).zip(curves) {
        iges_array1_set(&array, rank, curve.clone(), WHERE)?;
    }
    Ok(array)
}
