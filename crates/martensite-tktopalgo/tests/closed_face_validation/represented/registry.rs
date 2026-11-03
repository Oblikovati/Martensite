//! Fixture references use declared keys, never coordinate matching.
use super::{locations, types::*};
use martensite_tkbrep::{brep::BRepCurveRepresentation, topods::Shape};
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg3d::geom::{GeomCurve, GeomSurface};
use martensite_tkmath::toploc::{Datum3, Location};
use std::collections::BTreeMap;
pub(super) struct FixtureOwners {
    pub shapes: BTreeMap<usize, Shape>,
    pub surfaces: BTreeMap<usize, Handle<dyn GeomSurface>>,
    pub curves3: BTreeMap<usize, Handle<dyn GeomCurve>>,
    pub curves2: BTreeMap<usize, Handle<dyn Geom2dCurve>>,
    pub representations: BTreeMap<usize, Handle<dyn BRepCurveRepresentation>>,
    pub datums: Vec<Handle<Datum3>>,
}
impl FixtureOwners {
    pub fn new(input: &GraphInput) -> Self {
        Self {
            shapes: BTreeMap::new(),
            surfaces: BTreeMap::new(),
            curves3: BTreeMap::new(),
            curves2: BTreeMap::new(),
            representations: BTreeMap::new(),
            datums: locations::datums(input.datums),
        }
    }
    pub fn shape(&self, owner: usize) -> Shape {
        self.shapes
            .get(&owner)
            .unwrap_or_else(|| panic!("owner {owner}: expected fixture topology"))
            .clone()
    }
    pub fn surface(&self, owner: usize) -> Handle<dyn GeomSurface> {
        self.surfaces
            .get(&owner)
            .unwrap_or_else(|| panic!("owner {owner}: expected fixture surface"))
            .clone()
    }
    pub fn curve3(&self, owner: usize) -> Handle<dyn GeomCurve> {
        self.curves3
            .get(&owner)
            .unwrap_or_else(|| panic!("owner {owner}: expected spatial fixture curve"))
            .clone()
    }
    pub fn curve2(&self, owner: usize) -> Handle<dyn Geom2dCurve> {
        self.curves2
            .get(&owner)
            .unwrap_or_else(|| panic!("owner {owner}: expected planar fixture curve"))
            .clone()
    }
    pub fn location(&self, input: LocationInput) -> Location {
        locations::location(input, &self.datums)
    }
    pub fn occurrence(&self, input: OccurrenceInput) -> Shape {
        let orientation = martensite_tkg3d::top_abs::Orientation::ALL[input.orientation as usize];
        self.shape(input.owner)
            .located(&self.location(input.location), false)
            .unwrap()
            .oriented(orientation)
    }
}
