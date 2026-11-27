//! Graphic3d buffers are read at frame extraction so shared buffer mutations stay visible.
use crate::draw::{DrawStream, MutableGeometry};
use crate::primitive_buffer_read::{read_color, read_index_words, read_vector_attribute};
use crate::primitive_cache::{
    AttributeStamp, BoundsStamp, GeometryStamp, GeometryStyle, IndexStamp, RetainedPrimitive,
};
use crate::{PrimitiveDraw, RenderError, RenderVertex, error::rejected};
use martensite_tkernel::{ncollection::Vec4, standard::Handle};
use martensite_tkservice::graphic3d::*;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct PrimitiveSubmission {
    pub kind: Graphic3dTypeOfPrimitiveArray,
    pub indices: Handle<dyn Graphic3dIndexBufferSource>,
    pub attributes: Handle<dyn Graphic3dBufferSource>,
    pub bounds: Handle<Graphic3dBoundBuffer>,
    /// Native buffer residency recorded by frame submissions; read by frame statistics.
    pub upload: crate::element_upload::ElementUploadSlot,
    /// The driver id of the native `GlPrimitiveArray` (`GetUID`); 0 when none was taken.
    pub uid: usize,
    /// Group presentations may share immutable lowering while the same source is live.
    pub retained: Option<Arc<RetainedPrimitive>>,
    pub mesh_levels: Option<Arc<crate::mesh_lod::SubmittedMeshLevels>>,
}

#[derive(Clone, Copy)]
struct DrawStyle<'a> {
    color: [f32; 4],
    matrix: [f32; 16],
    line: Option<crate::LineStyle>,
    point: Option<&'a crate::PointStyle>,
}

type DrawSnapshot = (Vec<PrimitiveDraw>, Option<GeometryStamp>);

pub(crate) enum PrimitiveBounds {
    Finite(Graphic3dBndBox4f),
    Unchecked(Vec<[f32; 4]>),
}

impl PrimitiveSubmission {
    pub(crate) fn observe_inputs(&self, scope: &GraphicInputRevision) -> bool {
        if let Some(bounds) = self.bounds.get() {
            bounds.observe_input(scope);
        }
        self.attributes
            .get()
            .is_some_and(|buffer| buffer.observe_buffer_input(scope))
            && self
                .indices
                .get()
                .is_none_or(|buffer| buffer.observe_buffer_input(scope))
    }

    pub fn positions(&self) -> Result<Vec<[f32; 4]>, RenderError> {
        self.position_snapshot(false)
            .map(|(positions, _)| positions)
    }

    pub(crate) fn group_bounds(&self) -> Result<PrimitiveBounds, RenderError> {
        let read = self.attribute_read()?;
        let stamp = self
            .retained
            .as_ref()
            .and_then(|_| AttributeStamp::capture(&read));
        if let Some((cache, stamp)) = self.retained.as_ref().zip(stamp.as_ref())
            && let Some(bounds) = cache.bounds.get(stamp)
        {
            return Ok(PrimitiveBounds::Finite(bounds));
        }
        let positions = read_positions(&read)?;
        drop(read);
        let computed = PrimitiveBounds::from_positions(positions);
        if let (Some(cache), Some(stamp), PrimitiveBounds::Finite(bounds)) =
            (&self.retained, stamp, &computed)
        {
            cache.bounds.record(stamp, *bounds);
        }
        Ok(computed)
    }

    fn attribute_read(&self) -> Result<Graphic3dBufferRead<'_>, RenderError> {
        self.attributes
            .get()
            .map(Graphic3dBufferMembers::read_buffer)
            .ok_or_else(|| rejected("null attributes", "a Graphic3d vertex buffer"))
    }

    fn position_snapshot(
        &self,
        track: bool,
    ) -> Result<(Vec<[f32; 4]>, Option<AttributeStamp>), RenderError> {
        let read = self.attribute_read()?;
        Ok((read_positions(&read)?, attribute_stamp(&read, track)))
    }

    pub(crate) fn normals(&self) -> Result<Option<Vec<[f32; 3]>>, RenderError> {
        read_normals(&self.attribute_read()?)
    }

    pub(crate) fn shared_normals(&self) -> Result<Option<DrawStream<[f32; 3]>>, RenderError> {
        let read = self.attribute_read()?;
        let stamp = attribute_stamp(&read, self.retained.is_some());
        if let Some((cache, stamp)) = self.retained.as_ref().zip(stamp.as_ref())
            && let Some(normals) = cache.normals.get(stamp)
        {
            return Ok(normals);
        }
        let normals = read_normals(&read)?.map(DrawStream::from);
        drop(read);
        if let Some((cache, stamp)) = self.retained.as_ref().zip(stamp) {
            cache.normals.record(stamp, normals.clone());
        }
        Ok(normals)
    }

    pub(crate) fn shared_sphere_normals(&self) -> Result<DrawStream<[f32; 3]>, RenderError> {
        if let Some(normals) = self.shared_normals()? {
            return Ok(normals);
        }
        let read = self.attribute_read()?;
        let cache = self
            .retained
            .as_ref()
            .map(|retained| &retained.texture_streams);
        let tracked = cache.and_then(|cache| Some((cache, AttributeStamp::capture(&read)?)));
        if let Some((cache, stamp)) = &tracked
            && let Some(normals) = cache.zero_normals(stamp)
        {
            return Ok(normals);
        }
        let count = read_positions(&read)?.len();
        drop(read);
        let normals = vec![[0.; 3]; count].into();
        Ok(match tracked {
            Some((cache, stamp)) => cache.remember_zero_normals(stamp, normals),
            None => normals,
        })
    }

    /// Lower the draws and apply `style` to each; the cache keeps the styled bodies so the
    /// next lowering with the same style shares them (#594).
    /// `primitive.styled_draws(color, matrix, None, None, |draw| draw.with_polygon_offset(offset))`
    pub(crate) fn styled_draws(
        &self,
        color: [f32; 4],
        matrix: [f32; 16],
        line: Option<crate::LineStyle>,
        point: Option<crate::PointStyle>,
        style_draw: impl FnMut(PrimitiveDraw) -> PrimitiveDraw,
    ) -> Result<Vec<PrimitiveDraw>, RenderError> {
        let style = DrawStyle {
            color,
            matrix,
            line,
            point: point.as_ref(),
        };
        let retained = self
            .retained
            .as_ref()
            .filter(|_| matrix.iter().all(|value| value.is_finite()));
        let key = GeometryStyle::resolve(color, line, point.is_some());
        if let Some(cache) = retained
            && let Some(source) = GeometryStamp::capture(self)
            && let Some(draws) = cache.draws(&source, key, matrix, line, point.as_ref())
        {
            let styled: Vec<_> = draws.into_iter().map(style_draw).collect();
            cache.adopt_styled(&source, key, &styled);
            return Ok(styled);
        }
        let (mut draws, source) = self.lower_draws(style, retained.is_some())?;
        let Some((cache, source)) = retained.zip(source) else {
            return Ok(draws.into_iter().map(style_draw).collect());
        };
        cache.record(source.clone(), key, &mut draws, self.mutable_geometry());
        let styled: Vec<_> = draws.into_iter().map(style_draw).collect();
        cache.adopt_styled(&source, key, &styled);
        Ok(styled)
    }

    fn mutable_geometry(&self) -> MutableGeometry {
        MutableGeometry {
            vertices: self
                .attributes
                .get()
                .is_some_and(Graphic3dBufferSource::is_mutable),
            indices: self
                .indices
                .get()
                .is_some_and(Graphic3dBufferSource::is_mutable),
        }
    }

    fn lower_draws(&self, style: DrawStyle<'_>, track: bool) -> Result<DrawSnapshot, RenderError> {
        let (positions, position_stamp) = self.position_snapshot(track)?;
        let (indices, index_stamp) = self.index_snapshot(positions.len(), track)?;
        let (vertices, vertex_colors, color_stamp) =
            self.vertices(positions, style.color, track)?;
        let bounds = self.bounds.get().map(Graphic3dBoundBuffer::state);
        let bounds_stamp = track
            .then(|| match bounds.as_deref() {
                Some(state) => BoundsStamp::capture(state).map(Some),
                None => Some(None),
            })
            .flatten();
        let ranges =
            crate::primitive_ranges::ranges_for_state(bounds.as_deref(), &indices, style.color)?;
        drop(bounds);
        let source = geometry_stamp(position_stamp, color_stamp, index_stamp, bounds_stamp);
        let draws = ranges
            .into_iter()
            .map(|range| self.range_draw(range, &vertices, vertex_colors, style))
            .collect::<Result<_, _>>()?;
        Ok((draws, source))
    }

    fn range_draw(
        &self,
        range: crate::primitive_ranges::PrimitiveRange,
        vertices: &[RenderVertex],
        vertex_colors: bool,
        style: DrawStyle<'_>,
    ) -> Result<PrimitiveDraw, RenderError> {
        let indices = if style.point.is_some() {
            range.indices.clone()
        } else if style.line.is_some() {
            crate::primitive_topology::line_indices(self.kind, &range.indices)?
        } else {
            crate::primitive_topology::triangle_indices(self.kind, &range.indices)?
        };
        let vertices = range_vertices(vertices, range.color, vertex_colors);
        if let Some(point) = style.point {
            return PrimitiveDraw::points(vertices, indices, style.matrix, point.clone());
        }
        match style.line {
            Some(line) => PrimitiveDraw::segments(vertices, indices, style.matrix, line),
            None => self.quad_tagged_draw(vertices, indices, style.matrix, &range.indices),
        }
    }
    fn quad_tagged_draw(
        &self,
        vertices: Vec<RenderVertex>,
        indices: Vec<u32>,
        matrix: [f32; 16],
        source: &[u32],
    ) -> Result<PrimitiveDraw, RenderError> {
        let mut draw = PrimitiveDraw::new(vertices, indices, matrix)?;
        draw.native_flat_sources =
            crate::fixed_flat_sources::native_flat_sources(self.kind, source, &draw.indices).into();
        if self.kind == Graphic3dTypeOfPrimitiveArray::Quadrangles {
            draw.quad_corners = source.as_chunks::<4>().0.to_vec().into();
        }
        Ok(draw)
    }
    fn vertices(
        &self,
        positions: Vec<[f32; 4]>,
        fallback: [f32; 4],
        track: bool,
    ) -> Result<(Vec<RenderVertex>, bool, Option<AttributeStamp>), RenderError> {
        let read = self.attributes.read_buffer();
        let stamp = attribute_stamp(&read, track);
        let Some(colors) = read.attribute(Graphic3dTypeOfAttribute::Color) else {
            let vertices = positions
                .into_iter()
                .map(|position| RenderVertex::homogeneous(position, fallback))
                .collect();
            return Ok((vertices, false, stamp));
        };
        let vertices = positions
            .into_iter()
            .enumerate()
            .map(|(index, position)| -> Result<_, RenderError> {
                let color = read_color(&colors, index)?;
                Ok(RenderVertex::homogeneous(position, color))
            })
            .collect::<Result<_, _>>()?;
        Ok((vertices, true, stamp))
    }

    pub(crate) fn read_indices(&self, count: usize) -> Result<Vec<u32>, RenderError> {
        self.index_snapshot(count, false)
            .map(|(indices, _)| indices)
    }

    fn index_snapshot(
        &self,
        count: usize,
        track: bool,
    ) -> Result<(Vec<u32>, Option<Option<IndexStamp>>), RenderError> {
        if self.indices.is_null() {
            let indices = (0..count)
                .map(|i| u32::try_from(i).map_err(|_| rejected(i, "a 32-bit vertex index")))
                .collect::<Result<_, _>>()?;
            return Ok((indices, Some(None)));
        }
        let read = self.indices.read_buffer();
        let stride = read.element_stride();
        if !matches!(stride, 2 | 4) {
            return Err(rejected(stride, "16-bit or 32-bit indices"));
        }
        let indices =
            read_index_words(stride, read.bytes(), validated_count(read.element_count())?)?;
        let stamp = track
            .then(|| IndexStamp::capture(&read))
            .flatten()
            .map(Some);
        Ok((indices, stamp))
    }
}

impl PrimitiveBounds {
    fn from_positions(positions: Vec<[f32; 4]>) -> Self {
        let mut bounds = Graphic3dBndBox4f::new();
        for &[x, y, z, _] in &positions {
            if ![x, y, z].into_iter().all(f32::is_finite) {
                return Self::Unchecked(positions);
            }
            bounds.add(&Vec4::new(x, y, z, 1.));
        }
        Self::Finite(bounds)
    }

    pub(crate) fn append(self, target: &mut Graphic3dBndBox4f) {
        match self {
            Self::Finite(bounds) => target.combine(&bounds),
            Self::Unchecked(positions) => {
                // Ignore the homogeneous weight and preserve the native handling of malformed XYZ.
                for [x, y, z, _] in positions {
                    target.add(&Vec4::new(x, y, z, 1.));
                }
            }
        }
    }
}

fn read_positions(read: &Graphic3dBufferRead<'_>) -> Result<Vec<[f32; 4]>, RenderError> {
    let count = validated_count(read.element_count())?;
    let positions = read
        .attribute(Graphic3dTypeOfAttribute::Pos)
        .ok_or_else(|| {
            rejected(
                "missing position",
                "a Vec2, Vec3 or Vec4 position attribute",
            )
        })?;
    read_vector_attribute(
        &positions,
        count,
        "Vec2, Vec3 or Vec4 positions",
        "allocated position bytes",
        |position| position,
    )
}

fn read_normals(read: &Graphic3dBufferRead<'_>) -> Result<Option<Vec<[f32; 3]>>, RenderError> {
    let Some(normals) = read.attribute(Graphic3dTypeOfAttribute::Norm) else {
        return Ok(None);
    };
    let count = validated_count(read.element_count())?;
    read_vector_attribute(
        &normals,
        count,
        "Vec2, Vec3 or Vec4 positions",
        "allocated position bytes",
        |[x, y, z, _]| [x, y, z],
    )
    .map(Some)
}

fn attribute_stamp(read: &Graphic3dBufferRead<'_>, track: bool) -> Option<AttributeStamp> {
    track.then(|| AttributeStamp::capture(read)).flatten()
}

fn geometry_stamp(
    position: Option<AttributeStamp>,
    color: Option<AttributeStamp>,
    indices: Option<Option<IndexStamp>>,
    bounds: Option<Option<BoundsStamp>>,
) -> Option<GeometryStamp> {
    let attributes = position.filter(|source| color.as_ref() == Some(source))?;
    Some(GeometryStamp {
        attributes,
        indices: indices?,
        bounds: bounds?,
    })
}

fn validated_count(count: i32) -> Result<usize, RenderError> {
    usize::try_from(count).map_err(|_| rejected(count, "a nonnegative buffer element count"))
}

fn range_vertices(
    vertices: &[RenderVertex],
    color: [f32; 4],
    vertex_colors: bool,
) -> Vec<RenderVertex> {
    // GlPrimitiveArray::drawArray sets the bound's uniform; a vertex color still wins.
    vertices
        .iter()
        .map(|vertex| RenderVertex {
            color: if vertex_colors { vertex.color } else { color },
            ..*vertex
        })
        .collect()
}
