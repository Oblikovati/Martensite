//

//! `TopologyEdge` and `TopologyFace`: the display records of a drawable shape.
use martensite_raster_core::PaletteColor;
use martensite_tkbrep::topods::{Edge, Face};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::geom_abs::IsoType;

/// `TopologyEdge`: an edge with the color it is drawn in.
#[derive(Debug, Clone)]
pub struct EdgeDrawRecord {
    edge: Edge,
    color: PaletteColor,
}

impl EdgeDrawRecord {
    /// `TopologyEdge(E, C)`.
    pub fn new(edge: Edge, color: PaletteColor) -> Self {
        Self { edge, color }
    }

    /// `Edge()`.
    pub fn edge(&self) -> &Edge {
        &self.edge
    }

    /// `Edge(E)`.
    pub fn set_edge(&mut self, edge: Edge) {
        self.edge = edge;
    }

    /// `Color()`.
    pub fn color(&self) -> PaletteColor {
        self.color
    }

    /// `Color(C)`.
    pub fn set_color(&mut self, color: PaletteColor) {
        self.color = color;
    }
}

/// One iso of a face: its kind, its parameter and its range on the other parameter.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FaceIso {
    /// `T`: `IsoU` or `IsoV`.
    pub kind: IsoType,
    /// `Par`.
    pub parameter: f64,
    /// `T1`.
    pub first: f64,
    /// `T2`.
    pub last: f64,
}

/// `TopologyFace`: a face with the color and the isos it is drawn with.
#[derive(Debug, Clone)]
pub struct FaceDrawRecord {
    face: Face,
    color: PaletteColor,
    isos: Vec<FaceIso>,
}

impl FaceDrawRecord {
    /// `TopologyFace(F, N, C)`: room for `n` isos (`IsoU` at 0 until set).
    ///
    /// ```
    /// use martensite_tkbrep::topods::Face;
    /// use martensite_geometry_raster::edge_face::FaceDrawRecord;
    /// use martensite_raster_core::{PaletteColor, PaletteKind};
    /// let face = FaceDrawRecord::new(Face::default(), 2, PaletteColor::new(PaletteKind::Bleu));
    /// assert_eq!(face.nb_isos(), 2);
    /// ```
    pub fn new(face: Face, n: i32, color: PaletteColor) -> Self {
        let unset = FaceIso {
            kind: IsoType::IsoU,
            parameter: 0.0,
            first: 0.0,
            last: 0.0,
        };
        Self {
            face,
            color,
            isos: vec![unset; usize::try_from(n).unwrap_or(0)],
        }
    }

    /// `Face()`.
    pub fn face(&self) -> &Face {
        &self.face
    }

    /// `Face(F)`.
    pub fn set_face(&mut self, face: Face) {
        self.face = face;
    }

    /// `NbIsos`.
    pub fn nb_isos(&self) -> i32 {
        self.isos.len() as i32
    }

    /// `Iso(I, T, Par, T1, T2)`: sets iso `index` (1-based); Standard_OutOfRange
    /// outside `1..=NbIsos`.
    pub fn set_iso(&mut self, index: i32, iso: FaceIso) -> Result<(), Failure> {
        *self.slot(index)? = iso;
        Ok(())
    }

    /// `GetIso(I, T, Par, T1, T2)`; Standard_OutOfRange outside `1..=NbIsos`.
    pub fn iso(&self, index: i32) -> Result<FaceIso, Failure> {
        let count = self.isos.len();
        usize::try_from(index - 1)
            .ok()
            .and_then(|i| self.isos.get(i).copied())
            .ok_or_else(|| out_of_range(index, count))
    }

    /// `Color()`.
    pub fn color(&self) -> PaletteColor {
        self.color
    }

    /// `Color(C)`.
    pub fn set_color(&mut self, color: PaletteColor) {
        self.color = color;
    }

    fn slot(&mut self, index: i32) -> Result<&mut FaceIso, Failure> {
        let count = self.isos.len();
        usize::try_from(index - 1)
            .ok()
            .and_then(|i| self.isos.get_mut(i))
            .ok_or_else(|| out_of_range(index, count))
    }
}

/// The Standard_OutOfRange of `NCollectionArray1` for an iso index.
fn out_of_range(index: i32, count: usize) -> Failure {
    Failure::with_message(
        "Standard_OutOfRange",
        &format!("DBRep_Face: iso {index} outside 1..{count}"),
    )
}
