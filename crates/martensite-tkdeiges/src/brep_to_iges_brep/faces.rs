//! Edges, wires as loops, and faces, shells and solids as their BRep entities.
use super::BRepToIgesBRepEntity;
use crate::brep_to_iges::{BRepToIgesOriginMap, count_subshapes};
use crate::geom_to_iges::GeomToIgesGeomSurface;
use crate::iges_basic::{
    IgesBasicHArray1OfHArray1OfIgesEntity, IgesBasicHArray1OfHArray1OfInteger,
};
use crate::iges_data::{IgesDataIgesEntity, iges_array1_set, iges_live, iges_new_array1};
use crate::iges_solid::{IgesSolidFace, IgesSolidLoop, IgesSolidManifoldSolid, IgesSolidShell};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_tools::{self, WireExplorer};
use martensite_tkbrep::topexp::{self, Explorer};
use martensite_tkbrep::topods::{self, Edge, Face, Shell, Solid, Wire};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomRectangularTrimmedSurface, GeomSurface};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkshhealing::shape_algo::ShapeAlgo;
use martensite_tkxsbase::interface::InterfaceStatic;

const WHERE: &str = "BRepToIgesBRepEntity";

/// What a wire gives to its loop, by edge.
#[derive(Default)]
struct LoopItems {
    curves_2d: Vec<Handle<IgesDataIgesEntity>>,
    types: Vec<i32>,
    indices: Vec<i32>,
    orientations: Vec<i32>,
}

fn int_array(values: &[i32]) -> Result<Handle<HArray1<i32>>, Failure> {
    let array = iges_new_array1(1, values.len() as i32)?;
    for (rank, value) in (1..).zip(values) {
        iges_array1_set(&array, rank, *value, WHERE)?;
    }
    Ok(array)
}

fn handle_array<T>(values: &[Handle<T>]) -> Result<Handle<HArray1<Handle<T>>>, Failure>
where
    T: ?Sized + martensite_tkernel::standard::RuntimeTyped,
    Handle<T>: Default + Clone + Send + Sync + 'static,
{
    let array = iges_new_array1(1, values.len() as i32)?;
    for (rank, value) in (1..).zip(values) {
        iges_array1_set(&array, rank, value.clone(), WHERE)?;
    }
    Ok(array)
}

impl BRepToIgesBRepEntity {
    /// The curve of an edge in space: the one it has in the list, or its translation;
    /// `tool.transfer_edge(&edge)?`.
    ///
    /// # Errors
    /// A failure of the curve of the edge.
    pub fn transfer_edge(&self, edge: &Edge) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        let rank = self.index_edge(edge);
        if rank != 0 {
            let known = self.curves[rank as usize - 1].clone();
            if known.is_some() {
                return Ok(known);
            }
        }
        self.wire_tool()?
            .transfer_edge(edge, &BRepToIgesOriginMap::default(), true)
    }
    /// The pcurve of an edge on a face; the edge goes in the list with its curve;
    /// `tool.transfer_edge_on_face(&edge, &face, 1.0)?`.
    ///
    /// # Errors
    /// A failure of the curves of the edge.
    pub fn transfer_edge_on_face(
        &mut self,
        edge: &Edge,
        face: &Face,
        length: f64,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        if edge.is_null() {
            return Ok(Handle::default());
        }
        let origins = BRepToIgesOriginMap::default();
        let curve_2d = self
            .wire_tool()?
            .transfer_edge_on_face(edge, face, &origins, length, true)?;
        if !BRepTool::degenerated(edge)? {
            let curve_3d = self.transfer_edge(edge)?;
            if curve_3d.is_null() {
                self.add_fail(edge, " Transfer Failed : no Curve 3D ")?;
            }
            let _ = self.add_edge(edge, &curve_3d);
        }
        Ok(curve_2d)
    }
    /// One edge of a wire in the items of its loop.
    fn add_loop_edge(
        &mut self,
        edge: &Edge,
        face: &Face,
        length: f64,
        items: &mut LoopItems,
    ) -> Result<(), Failure> {
        let curve_2d = self.transfer_edge_on_face(edge, face, length)?;
        items.curves_2d.push(curve_2d);
        let (first, last) = topexp::vertices(edge, false);
        let index = if BRepTool::degenerated(edge)? {
            items.types.push(1);
            self.add_vertex(&first)
        } else {
            if !first.is_null() {
                let _ = self.add_vertex(&first);
            }
            if !last.is_null() {
                let _ = self.add_vertex(&last);
            }
            items.types.push(0);
            self.index_edge(edge)
        };
        items.indices.push(index);
        match edge.orientation() {
            Orientation::Forward => items.orientations.push(1),
            Orientation::Reversed => items.orientations.push(0),
            _ => {}
        }
        Ok(())
    }
    fn loop_items(&mut self, wire: &Wire, face: &Face, length: f64) -> Result<LoopItems, Failure> {
        let mut items = LoopItems::default();
        if !Explorer::new(wire, ShapeEnum::Vertex, ShapeEnum::Shape)?.more() {
            self.add_warning(wire, " no Vertex associated to the Wire")?;
            return Ok(items);
        }
        let mut explorer = WireExplorer::with_wire_on_face(wire, face)?;
        while explorer.more() {
            let edge = explorer.current().clone();
            if edge.is_null() {
                self.add_warning(wire, "an Edge is a null entity")?;
            } else {
                self.add_loop_edge(&edge, face, length, &mut items)?;
            }
            explorer.next()?;
        }
        Ok(items)
    }
    /// The loop of a wire on a face; `tool.transfer_wire(&wire, &face, 1.0)?`.
    ///
    /// # Errors
    /// A failure of an edge or of the binding of the result.
    pub fn transfer_wire(
        &mut self,
        wire: &Wire,
        face: &Face,
        length: f64,
    ) -> Result<Handle<IgesSolidLoop>, Failure> {
        let result = Handle::new(IgesSolidLoop::new());
        if wire.is_null() {
            return Ok(result);
        }
        let items = self.loop_items(wire, face, length)?;
        let count = items.curves_2d.len() as i32;
        let edges: Handle<HArray1<Handle<IgesDataIgesEntity>>> = iges_new_array1(1, count)?;
        let nb_curves: Handle<HArray1<i32>> = iges_new_array1(1, count)?;
        let iso_flags = Handle::new(IgesBasicHArray1OfHArray1OfInteger::with_bounds(1, count)?);
        let curves = Handle::new(IgesBasicHArray1OfHArray1OfIgesEntity::with_bounds(
            1, count,
        )?);
        for rank in 1..=count {
            let at = rank as usize - 1;
            let list: Handle<IgesDataIgesEntity> = if items.types[at] == 0 {
                self.edge_list.cast()
            } else {
                self.vertex_list.cast()
            };
            iges_array1_set(&edges, rank, list, WHERE)?;
            let curve_2d = items.curves_2d[at].clone();
            iges_array1_set(&nb_curves, rank, i32::from(curve_2d.is_some()), WHERE)?;
            iges_live(&iso_flags, WHERE)?.set_value(rank, &int_array(&[0])?)?;
            iges_live(&curves, WHERE)?.set_value(rank, &handle_array(&[curve_2d])?)?;
        }
        iges_live(&result, WHERE)?.init(
            &int_array(&items.types)?,
            &edges,
            &int_array(&items.indices)?,
            &int_array(&items.orientations)?,
            &nb_curves,
            &iso_flags,
            &curves,
        )?;
        self.set_shape_result(wire, &result.cast())?;
        Ok(result)
    }
    /// The surface entity of a face in BRep mode and its generatrix length; null for no translation.
    fn face_surface(
        &self,
        start: &Face,
        surface: &Handle<dyn GeomSurface>,
    ) -> Result<(Handle<IgesDataIgesEntity>, f64), Failure> {
        let bounds = brep_tools::uv_bounds(start)?;
        let mut tool = GeomToIgesGeomSurface::new();
        tool.set_b_rep_mode(true);
        tool.set_analytic_mode(InterfaceStatic::i_val("write.convertsurface.mode") == 0);
        tool.set_model(&self.get_model())?;
        let trimmed = surface.cast::<GeomRectangularTrimmedSurface>();
        let basis = trimmed.get().map_or_else(
            || surface.clone(),
            GeomRectangularTrimmedSurface::basis_surface,
        );
        let entity = tool.transfer_surface(&basis, bounds)?;
        if entity.is_null() {
            self.add_warning(start, "the basic surface is a null entity")?;
        }
        Ok((entity, tool.length()))
    }
    /// The face entity of a face, with its outer loop first;
    /// `tool.transfer_face(&face)?`.
    ///
    /// # Errors
    /// A failure of the geometry of the face or of the binding of the result.
    pub fn transfer_face(&mut self, start: &Face) -> Result<Handle<IgesSolidFace>, Failure> {
        let result = Handle::new(IgesSolidFace::new());
        if start.is_null() {
            return Ok(result);
        }
        let surface = BRepTool::surface(start)?;
        let (mut entity, mut length) = (Handle::default(), 1.0);
        if surface.is_some() {
            (entity, length) = self.face_surface(start, &surface)?;
            if entity.is_null() {
                return Ok(result);
            }
        }
        let mut face = start.clone();
        if start.orientation() == Orientation::Reversed {
            face.reverse();
        }
        let container = ShapeAlgo::algo_container();
        let outer = iges_live(&container, WHERE)?.outer_wire(&face)?;
        let has_outer = !outer.is_null();
        let outer_loop = if has_outer {
            self.transfer_wire(&outer, &face, length)?
        } else {
            Handle::new(IgesSolidLoop::new())
        };
        let mut loops = vec![outer_loop];
        let mut wires = Explorer::new(&face, ShapeEnum::Wire, ShapeEnum::Shape)?;
        while wires.more() {
            let wire = topods::wire(wires.current())?;
            wires.next()?;
            if wire.is_null() {
                self.add_warning(start, " a Wire is a null entity")?;
            } else if !wire.is_same(&outer) {
                loops.push(self.transfer_wire(&wire, &face, length)?);
            }
        }
        let mut free_edges = Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Wire)?;
        while free_edges.more() {
            self.add_warning(
                free_edges.current(),
                "An edge alone is not transfer as an IGESBRep Entity",
            )?;
            free_edges.next()?;
        }
        iges_live(&result, WHERE)?.init(&entity, has_outer, &handle_array(&loops)?)?;
        self.set_shape_result(start, &result.cast())?;
        Ok(result)
    }
    /// The shell entity of a shell; `tool.transfer_shell(&shell, &range)?`.
    ///
    /// # Errors
    /// A failure of a face or of the binding of the result.
    pub fn transfer_shell(
        &mut self,
        start: &Shell,
        range: &ProgressRange<'_>,
    ) -> Result<Handle<IgesSolidShell>, Failure> {
        let result = Handle::new(IgesSolidShell::new());
        if start.is_null() {
            return Ok(result);
        }
        let count = count_subshapes(start, ShapeEnum::Face, ShapeEnum::Shape)?;
        let scope = ProgressScope::unnamed(range, f64::from(count), false);
        let (mut faces, mut flags) = (Vec::new(), Vec::new());
        let mut explorer = Explorer::new(start, ShapeEnum::Face, ShapeEnum::Shape)?;
        while explorer.more() && scope.more() {
            let face = topods::face(explorer.current())?;
            explorer.next()?;
            let _step = scope.next(1.0);
            if face.is_null() {
                self.add_warning(start, " a Face is a null entity")?;
                continue;
            }
            faces.push(self.transfer_face(&face)?);
            match face.orientation() {
                Orientation::Forward => flags.push(1),
                Orientation::Reversed => flags.push(0),
                _ => {}
            }
        }
        iges_live(&result, WHERE)?.init(&handle_array(&faces)?, &int_array(&flags)?)?;
        self.set_shape_result(start, &result.cast())?;
        Ok(result)
    }
    /// The manifold solid entity of a solid: its first shell, and the others as voids;
    /// `tool.transfer_solid(&solid, &range)?`.
    ///
    /// # Errors
    /// A failure of a shell or of the binding of the result.
    pub fn transfer_solid(
        &mut self,
        start: &Solid,
        range: &ProgressRange<'_>,
    ) -> Result<Handle<IgesSolidManifoldSolid>, Failure> {
        let result = Handle::new(IgesSolidManifoldSolid::new());
        if start.is_null() {
            return Ok(result);
        }
        let count = count_subshapes(start, ShapeEnum::Shell, ShapeEnum::Shape)?;
        let scope = ProgressScope::unnamed(range, f64::from(count), false);
        let (mut shells, mut flags) = (Vec::new(), Vec::new());
        let mut explorer = Explorer::new(start, ShapeEnum::Shell, ShapeEnum::Shape)?;
        while explorer.more() && scope.more() {
            let step = scope.next(1.0);
            let shell = topods::shell(explorer.current())?;
            explorer.next()?;
            if shell.is_null() {
                self.add_warning(start, " a Shell is a null entity")?;
                continue;
            }
            shells.push(self.transfer_shell(&shell, &step)?);
            match shell.orientation() {
                Orientation::Forward => flags.push(1),
                Orientation::Reversed => flags.push(0),
                _ => {}
            }
        }
        if shells.is_empty() {
            self.add_warning(start, " no Result ")?;
        } else {
            let (voids, void_flags) = if shells.len() > 1 {
                (handle_array(&shells[1..])?, int_array(&flags[1..])?)
            } else {
                (Handle::default(), Handle::default())
            };
            iges_live(&result, WHERE)?.init(&shells[0], flags[0] != 0, &voids, &void_flags)?;
        }
        self.set_shape_result(start, &result.cast())?;
        Ok(result)
    }
}
