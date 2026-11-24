//! `StringType`, `ItemNumber` and `IsValid` of `Graphic3d_ArrayOfPrimitives`.
use super::{
    Graphic3dArrayOfPrimitives, Graphic3dBufferMembers, Graphic3dBufferState,
    Graphic3dIndexBufferSource, Graphic3dTypeOfPrimitiveArray,
};
use martensite_tkernel::{ncollection::BufferRevision, standard::SharedState};

/// Counts that `IsValid` reads.
struct ArrayCounts {
    vertexs: i32,
    bounds: i32,
    edges: i32,
}

impl Graphic3dArrayOfPrimitives {
    /// The class name of the type ("ArrayOfTriangles", ...); `array.string_type()`.
    pub fn string_type(&self) -> &'static str {
        use Graphic3dTypeOfPrimitiveArray as T;
        match self.kind {
            T::Points => "ArrayOfPoints",
            T::Segments => "ArrayOfSegments",
            T::Polylines => "ArrayOfPolylines",
            T::Triangles => "ArrayOfTriangles",
            T::Trianglestrips => "ArrayOfTriangleStrips",
            T::Trianglefans => "ArrayOfTriangleFans",
            T::LinesAdjacency => "ArrayOfLinesAdjacency",
            T::LineStripAdjacency => "ArrayOfLineStripAdjacency",
            T::TrianglesAdjacency => "ArrayOfTrianglesAdjacency",
            T::TriangleStripAdjacency => "ArrayOfTriangleStripAdjacency",
            T::Quadrangles => "ArrayOfQuadrangles",
            T::Quadranglestrips => "ArrayOfQuadrangleStrips",
            T::Polygons => "ArrayOfPolygons",
            T::Undefined => "UndefinedArray",
        }
    }
    /// The count over the indices when there are some, else over the vertices, per `size`.
    fn indexed_items(&self, size: i32) -> i32 {
        let edges = self.edge_number();
        if edges < 1 {
            self.vertex_number() / size
        } else {
            edges / size
        }
    }
    /// The strip count: vertices less `per_bound` per bound (or once without bounds).
    fn strip_items(&self, per_bound: i32) -> i32 {
        let bounds = if self.bounds.is_null() {
            1
        } else {
            self.bound_number()
        };
        self.vertex_number() - per_bound * bounds
    }
    /// The number of primitives, -1 without vertex buffer or type; `array.item_number()`.
    pub fn item_number(&self) -> i32 {
        use Graphic3dTypeOfPrimitiveArray as T;
        if self.attribs.is_null() {
            return -1;
        }
        match self.kind {
            T::Points => self.vertex_number(),
            T::Polylines | T::Polygons => {
                if self.bounds.is_null() {
                    1
                } else {
                    self.bound_number()
                }
            }
            T::Segments => self.indexed_items(2),
            T::Triangles => self.indexed_items(3),
            T::Quadrangles | T::LinesAdjacency => self.indexed_items(4),
            T::TrianglesAdjacency => self.indexed_items(6),
            T::Trianglestrips | T::Trianglefans => self.strip_items(2),
            T::LineStripAdjacency | T::TriangleStripAdjacency => self.strip_items(4),
            T::Quadranglestrips => {
                self.vertex_number() / 2
                    - if self.bounds.is_null() {
                        1
                    } else {
                        self.bound_number()
                    }
            }
            T::Undefined => -1,
        }
    }
}

/// Truncates a count to a multiple of `size`, or fails at `size` or below (the triangle and
/// quad rules of `IsValid`); `Some(new count)` when it changes, `None` when it is valid.
fn truncated(count: i32, size: i32) -> Result<Option<i32>, ()> {
    if count >= size && count % size == 0 {
        return Ok(None);
    }
    if count <= size {
        return Err(());
    }
    Ok(Some(size * (count / size)))
}

impl Graphic3dArrayOfPrimitives {
    fn counts(&self) -> ArrayCounts {
        let bounds = if self.bounds.is_null() {
            0
        } else {
            self.bound_number()
        };
        let edges = if self.indices.is_null() {
            0
        } else {
            self.edge_number()
        };
        ArrayCounts {
            vertexs: self.vertex_number(),
            bounds,
            edges,
        }
    }
    /// The triangle or quad rule: truncates the indices (or vertices) to whole primitives.
    fn check_whole_primitives(&self, counts: &ArrayCounts, size: i32) -> bool {
        let (count, indices) = if counts.edges > 0 {
            (counts.edges, true)
        } else {
            (counts.vertexs, false)
        };
        match truncated(count, size) {
            Err(()) => false,
            Ok(None) => true,
            Ok(Some(count)) => {
                self.set_count(indices, count);
                true
            }
        }
    }
    fn set_count(&self, indices: bool, count: i32) {
        if indices {
            self.indices
                .get()
                .into_iter()
                .for_each(|buffer| buffer.set_nb_elements(count));
        } else {
            self.attribs
                .get()
                .into_iter()
                .for_each(|buffer| buffer.set_nb_elements(count));
        }
    }
    /// The per-type minimum counts of `IsValid`.
    fn check_type(&self, counts: &ArrayCounts) -> bool {
        use Graphic3dTypeOfPrimitiveArray as T;
        let (v, e) = (counts.vertexs, counts.edges);
        match self.kind {
            T::Points => v >= 1,
            T::Polylines => !(e > 0 && e < 2) && v >= 2,
            T::Segments => v >= 2,
            T::Polygons => !(e > 0 && e < 3) && v >= 3,
            T::Triangles => self.check_whole_primitives(counts, 3),
            T::Quadrangles => self.check_whole_primitives(counts, 4),
            T::Trianglefans | T::Trianglestrips => v >= 3,
            T::Quadranglestrips | T::LinesAdjacency | T::LineStripAdjacency => v >= 4,
            T::TrianglesAdjacency | T::TriangleStripAdjacency => v >= 6,
            T::Undefined => false,
        }
    }
    /// The bound rule: the bound counts must cover the indices (or vertices), which are
    /// truncated to their sum when larger.
    fn check_bounds(&self, counts: &ArrayCounts) -> bool {
        if counts.bounds <= 0 {
            return true;
        }
        let total: i32 = (1..=counts.bounds)
            .map(|rank| self.bound(rank).unwrap_or(0))
            .fold(0, i32::wrapping_add);
        let (count, indices) = if counts.edges > 0 {
            (counts.edges, true)
        } else {
            (counts.vertexs, false)
        };
        if total == count {
            return true;
        }
        if count <= total {
            return false;
        }
        self.set_count(indices, total);
        true
    }
    /// `IsValid`: whether the counts make whole primitives, truncating partial ones and the
    /// indices or vertices past the bounds, and whether every index is a vertex. The counts
    /// are those read before the truncation, as natively. Example: `array.is_valid()`.
    pub fn is_valid(&self) -> bool {
        if self.attribs.is_null() {
            return false;
        }
        let counts = self.counts();
        if !self.check_type(&counts) || !self.check_bounds(&counts) {
            return false;
        }
        let Some(indices) = self.indices.get().filter(|_| counts.edges > 0) else {
            return true;
        };
        self.index_extent
            .maximum(indices, counts.edges)
            .is_some_and(|index| index < self.vertex_number())
    }
}

#[derive(Default)]
pub(super) struct IndexExtentMemo(SharedState<Option<IndexExtent>>);

struct IndexExtent {
    revision: BufferRevision,
    stride: i32,
    count: i32,
    maximum: Option<i32>,
}

impl IndexExtentMemo {
    fn maximum(&self, indices: &dyn Graphic3dIndexBufferSource, count: i32) -> Option<i32> {
        let state = indices.buffer_base().state.read();
        let stride = state.stride;
        if let Some(cached) = self.cached_maximum(&state, count) {
            return cached;
        }
        let maximum = index_maximum(state.data.as_slice(), stride, count as usize);
        if let Some(revision) = state.data.content_revision() {
            *self.0.write() = Some(IndexExtent {
                revision,
                stride,
                count,
                maximum,
            });
        }
        maximum
    }

    fn cached_maximum(&self, state: &Graphic3dBufferState, count: i32) -> Option<Option<i32>> {
        self.0
            .read()
            .as_ref()
            .filter(|cached| {
                cached.stride == state.stride
                    && cached.count == count
                    && state.data.matches_content_revision(&cached.revision)
            })
            .map(|cached| cached.maximum)
    }
}

fn index_maximum(bytes: &[u8], stride: i32, count: usize) -> Option<i32> {
    match stride {
        2 => packed_index_maximum(bytes, count, |word| i32::from(u16::from_le_bytes(word))),
        4 => packed_index_maximum(bytes, count, i32::from_le_bytes),
        stride => (0..count).try_fold(i32::MIN, |maximum, edge| {
            // Graphic3dIndexBuffer::index reads a signed 32-bit word for every non-2 stride.
            let offset = (stride as usize).wrapping_mul(edge);
            let word = bytes.get(offset..offset.checked_add(4)?)?;
            Some(maximum.max(i32::from_le_bytes(word.try_into().ok()?)))
        }),
    }
}

fn packed_index_maximum<const N: usize>(
    bytes: &[u8],
    count: usize,
    decode: impl Fn([u8; N]) -> i32,
) -> Option<i32> {
    bytes
        .as_chunks::<N>()
        .0
        .get(..count)?
        .iter()
        .copied()
        .map(decode)
        .max()
}
