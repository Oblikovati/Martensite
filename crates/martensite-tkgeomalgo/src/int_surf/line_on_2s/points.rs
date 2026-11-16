//! Keep caller-owned allocation policies while the default walking path uses contiguous points.
use super::{BaseAllocator, Failure, IntSurfPntOn2S};
use martensite_tkernel::ncollection::Sequence;

pub(super) enum LinePoints {
    Contiguous(Vec<IntSurfPntOn2S>),
    Allocated(Sequence<IntSurfPntOn2S>),
}
impl LinePoints {
    pub(super) fn new(allocator: Option<BaseAllocator>) -> Self {
        match allocator {
            Some(resource) => Self::Allocated(Sequence::with_allocator(resource)),
            None => Self::Contiguous(Vec::new()),
        }
    }
    pub(super) fn len(&self) -> i32 {
        match self {
            Self::Contiguous(points) => i32::try_from(points.len()).unwrap_or(i32::MAX),
            Self::Allocated(points) => points.length(),
        }
    }
    pub(super) fn is_empty(&self) -> bool {
        match self {
            Self::Contiguous(points) => points.is_empty(),
            Self::Allocated(points) => points.is_empty(),
        }
    }
    pub(super) fn append(&mut self, point: IntSurfPntOn2S) {
        match self {
            Self::Contiguous(points) => points.push(point),
            Self::Allocated(points) => {
                points.append(point);
            }
        }
    }
    pub(super) fn point(&self, rank: usize) -> Result<IntSurfPntOn2S, Failure> {
        match self {
            Self::Contiguous(points) => Ok(points[rank - 1]),
            Self::Allocated(points) => points.value(rank).copied(),
        }
    }
    pub(super) fn point_mut(&mut self, rank: usize) -> Result<&mut IntSurfPntOn2S, Failure> {
        match self {
            Self::Contiguous(points) => Ok(&mut points[rank - 1]),
            Self::Allocated(points) => points.change_value(rank),
        }
    }
    pub(super) fn reverse(&mut self) {
        match self {
            Self::Contiguous(points) => points.reverse(),
            Self::Allocated(points) => points.reverse(),
        }
    }
    pub(super) fn clear(&mut self) {
        match self {
            Self::Contiguous(points) => points.clear(),
            Self::Allocated(points) => points.clear(None),
        }
    }
    pub(super) fn insert_before(
        &mut self,
        rank: usize,
        point: IntSurfPntOn2S,
    ) -> Result<(), Failure> {
        match self {
            Self::Contiguous(points) => {
                points.insert(rank - 1, point);
                Ok(())
            }
            Self::Allocated(points) => points.insert_before(rank, point).map(|_| ()),
        }
    }
    pub(super) fn remove(&mut self, rank: usize) -> Result<(), Failure> {
        match self {
            Self::Contiguous(points) => {
                points.remove(rank - 1);
                Ok(())
            }
            Self::Allocated(points) => points.remove(rank),
        }
    }
    pub(super) fn split(&mut self, rank: usize) -> Result<Self, Failure> {
        match self {
            Self::Contiguous(points) => Ok(Self::Contiguous(points.split_off(rank - 1))),
            Self::Allocated(points) => {
                let mut tail = Sequence::default();
                points.split(rank, &mut tail)?;
                Ok(Self::Allocated(tail))
            }
        }
    }
    // Dispatch once per traversal; default box rebuilds retain slice iteration.
    pub(super) fn for_each(&self, visit: impl FnMut(&IntSurfPntOn2S)) {
        match self {
            Self::Contiguous(points) => points.iter().for_each(visit),
            Self::Allocated(points) => points.iter().for_each(visit),
        }
    }
}
