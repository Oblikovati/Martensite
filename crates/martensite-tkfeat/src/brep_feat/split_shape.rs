//! `BRepFeatSplitShape`: the faces of a shape split by wires and edges lying on them
//! (`Spliter` over `WiresOnShape`), as a shape builder with the faces on
//! the left and right of the splitting wires.
use crate::errors::null_object;
use crate::loc_ope::{Spliter, WiresOnShape};
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{Compound, Edge, Face, Shape, Wire};
use martensite_tkbrep::toptools::MapOfShape;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::{List, Sequence};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeShape, BRepBuilderAPIMakeShapeBase,
};

/// `BRepFeatSplitShape`: the splitting of a shape by wires on its faces.
pub struct BRepFeatSplitShape {
    base: BRepBuilderAPIMakeShapeBase,
    s_shape: Spliter,
    w_on_shape: Option<WiresOnShape>,
    right: List<Shape>,
}

impl Default for BRepFeatSplitShape {
    fn default() -> Self {
        Self::new()
    }
}

impl BRepFeatSplitShape {
    /// No shape (the wires added only after `init`);
    /// `BRepFeatSplitShape::new()`.
    pub fn new() -> Self {
        Self {
            base: BRepBuilderAPIMakeShapeBase::new(),
            s_shape: Spliter::new(),
            w_on_shape: None,
            right: List::default(),
        }
    }

    /// The shape to split; `BRepFeatSplitShape::with_shape(&s)`.
    pub fn with_shape(s: &Shape) -> Self {
        Self {
            base: BRepBuilderAPIMakeShapeBase::new(),
            s_shape: Spliter::with_shape(s),
            w_on_shape: Some(WiresOnShape::new(s)),
            right: List::default(),
        }
    }

    /// The shape to split, the wires forgotten; `split.init(&s)`.
    pub fn init(&mut self, s: &Shape) {
        self.s_shape.init(s);
        match &mut self.w_on_shape {
            Some(w) => w.init(s),
            None => self.w_on_shape = Some(WiresOnShape::new(s)),
        }
    }

    /// The wires on the shape (null_object before a shape is given).
    fn wires(&mut self) -> Result<&mut WiresOnShape, Failure> {
        self.w_on_shape
            .as_mut()
            .ok_or_else(|| null_object("BRepFeat_SplitShape", "no shape to split"))
    }

    /// Whether the splitting edges inside the faces
    /// are checked; `split.set_check_interior(false)?`.
    pub fn set_check_interior(&mut self, to_check_interior: bool) -> Result<(), Failure> {
        self.wires()?.set_check_interior(to_check_interior);
        Ok(())
    }

    /// The splitting edges bound to the faces or edges they lie on;
    /// false when one cannot be placed; `split.add_edges(&edges)?`.
    pub fn add_edges(&mut self, the_edges: &Sequence<Shape>) -> Result<bool, Failure> {
        self.wires()?.add(the_edges)
    }

    /// The wire `w` splits the face `f`; `split.add_wire(&w, &f)?`.
    pub fn add_wire(&mut self, w: &Wire, f: &Face) -> Result<(), Failure> {
        self.wires()?.bind_wire(w, f)
    }

    /// The edge `e` splits the face `f`; `split.add_edge(&e, &f)?`.
    pub fn add_edge(&mut self, e: &Edge, f: &Face) -> Result<(), Failure> {
        self.wires()?.bind_edge(e, f)
    }

    /// The edges of `comp` split the face `f`;
    /// `split.add_compound(&comp, &f)?`.
    pub fn add_compound(&mut self, comp: &Compound, f: &Face) -> Result<(), Failure> {
        self.wires()?.bind_compound(comp, f)
    }

    /// The edge `e` lies on the edge `eon` of the shape;
    /// `split.add_edge_on_edge(&e, &eon)?`.
    pub fn add_edge_on_edge(&mut self, e: &Edge, eon: &Edge) -> Result<(), Failure> {
        self.wires()?.bind_edge_on_edge(e, eon);
        Ok(())
    }

    /// The faces on the left of the splitting wires;
    /// `split.direct_left()?`.
    pub fn direct_left(&self) -> Result<&List<Shape>, Failure> {
        self.s_shape.direct_left()
    }

    /// The faces on the left, with those connected to them;
    /// `split.left()?`.
    pub fn left(&self) -> Result<&List<Shape>, Failure> {
        self.s_shape.left()
    }

    /// The faces of the result not on the left (computed once after a build);
    /// `split.right()?`.
    pub fn right(&mut self) -> Result<&List<Shape>, Failure> {
        if self.right.is_empty() {
            let mut a_map_of_left = MapOfShape::default();
            for f in self.s_shape.left()?.iter() {
                a_map_of_left.add(f.clone());
            }
            for a_face in Explorer::new(&self.base.my_shape, ShapeEnum::Face, ShapeEnum::Shape)? {
                if !a_map_of_left.contains(&a_face) {
                    self.right.append(a_face);
                }
            }
        }
        Ok(&self.right)
    }
}

impl BRepBuilderAPIMakeShape for BRepFeatSplitShape {
    fn make_shape_base(&self) -> &BRepBuilderAPIMakeShapeBase {
        &self.base
    }

    fn make_shape_base_mut(&mut self) -> &mut BRepBuilderAPIMakeShapeBase {
        &mut self.base
    }

    /// The faces split; the result is the split shape.
    fn build(&mut self, _range: &ProgressRange<'_>) -> Result<(), Failure> {
        let w = self
            .w_on_shape
            .as_mut()
            .ok_or_else(|| null_object("BRepFeat_SplitShape::Build", "no shape to split"))?;
        self.s_shape.perform(w)?;
        if self.s_shape.is_done() {
            self.base.set_done();
            self.base.my_shape = self.s_shape.resulting_shape()?.clone();
            self.right.clear();
        }
        Ok(())
    }

    /// Whether the shape has no descendant.
    fn is_deleted(&mut self, f: &Shape) -> Result<bool, Failure> {
        Ok(self.s_shape.descendant_shapes(f)?.is_empty())
    }

    /// The descendants of the shape.
    fn modified(&mut self, f: &Shape) -> Result<&List<Shape>, Failure> {
        self.base.my_generated = self.s_shape.descendant_shapes(f)?;
        Ok(&self.base.my_generated)
    }
}
