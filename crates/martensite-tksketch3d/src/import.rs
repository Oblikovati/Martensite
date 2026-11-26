// SPDX-License-Identifier: GPL-2.0-only
//! Adopt model-space input rows as one checked edit while retaining their source order.
use crate::{
    CurveId, Edit, EditFailure, FitSplineId, FitSplineParameters, FitSplinePolicy, LineId, PointId,
    SketchFitSpline, SketchLine, SketchPoint, Snapshot,
};
use std::{collections::BTreeSet, error::Error, fmt};

/// Curves to build from a batch of ordered, already normalized point rows.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum ImportCurves<'a> {
    /// Keep each point with its supplied role.
    Points,
    /// Join adjacent rows in their input direction using these reserved line IDs.
    Polyline(&'a [LineId]),
    /// Copy explicit directed drawing segments using point IDs from these rows.
    DrawingSegments(&'a [SketchLine]),
    /// Fit the four rows in order and retain the supplied station parameters and limits.
    FitCubic {
        /// Reserved identity for the fitted curve.
        id: FitSplineId,
        /// Parameters of the four source rows in order.
        parameters: FitSplineParameters,
        /// Explicit conditioning, spatial and regularity limits.
        policy: FitSplinePolicy,
    },
}

/// One normalized import transaction. The application resolves source units into model units.
#[derive(Clone, Copy, Debug)]
pub struct SketchImport<'a> {
    /// Source rows in original order with caller-reserved IDs.
    pub points: &'a [SketchPoint],
    /// Topology or fit to construct from the supplied point rows.
    pub curves: ImportCurves<'a>,
    /// Maximum number of new point and curve entities admitted in this import.
    pub max_new_entities: usize,
}

/// Accepted snapshot and the mapping from input rows and uses to retained entities.
#[derive(Clone, Debug, PartialEq)]
pub struct ImportedSketch {
    /// Independent accepted snapshot.
    pub sketch: Snapshot,
    /// Retained point IDs in source row order.
    pub rows: Vec<PointId>,
    /// Retained curve IDs in the supplied order.
    pub curves: Vec<CurveId>,
}

/// Admission, budget or edit failure before an import has been published.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum ImportFailure {
    /// A point and curve request exceeded the caller's entity allowance.
    Limit {
        /// Total requested edits from the imported rows and curves.
        requested: usize,
        /// Caller-selected maximum number of new entities.
        maximum: usize,
    },
    /// The selected topology does not match the point and curve counts.
    Counts {
        /// Number of supplied point rows.
        points: usize,
        /// Number of reserved curve IDs or explicit line uses.
        curves: usize,
        /// Required relationship between the counts.
        expected: &'static str,
    },
    /// A drawing segment relies on a point absent from this import.
    ForeignEndpoint {
        /// Directed drawing segment with the unresolved endpoint.
        line: LineId,
        /// Endpoint value absent from the supplied rows.
        point: PointId,
    },
    /// The caller cancelled during import preparation.
    Cancelled,
    /// The ordinary checked edit batch refused the complete import.
    Edit(EditFailure),
}

impl fmt::Display for ImportFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Limit { requested, maximum } => {
                write!(
                    f,
                    "Sketch3D import of {requested} entities exceeds the allowed {maximum}"
                )
            }
            Self::Counts {
                points,
                curves,
                expected,
            } => {
                write!(
                    f,
                    "Sketch3D import with {points} points and {curves} curves expects {expected}"
                )
            }
            Self::ForeignEndpoint { line, point } => write!(
                f,
                "Sketch3D import line {line:?} refers to {point:?}, expected an endpoint in the import rows"
            ),
            Self::Cancelled => f.write_str("Sketch3D import cancelled during preparation"),
            Self::Edit(cause) => write!(f, "Sketch3D import could not be adopted. {cause}"),
        }
    }
}

impl Error for ImportFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Edit(cause) => Some(cause),
            _ => None,
        }
    }
}

impl Snapshot {
    /// Import finite normalized rows and build their curves in one checked batch.
    /// Example: `source.import_normalized(import, || false)?`.
    pub fn import_normalized(
        &self,
        import: SketchImport<'_>,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<ImportedSketch, ImportFailure> {
        poll(&mut cancelled)?;
        let count = import.require_counts()?;
        let mut edits = Vec::with_capacity(count);
        let mut rows = Vec::with_capacity(import.points.len());
        for &point in import.points {
            poll(&mut cancelled)?;
            edits.push(Edit::InsertPoint(point));
            rows.push(point.id);
        }
        let curves = import.curves.lower(&rows, &mut edits, &mut cancelled)?;
        let sketch = self.apply(&edits, cancelled).map_err(ImportFailure::Edit)?;
        Ok(ImportedSketch {
            sketch,
            rows,
            curves,
        })
    }
}

impl SketchImport<'_> {
    fn require_counts(self) -> Result<usize, ImportFailure> {
        let points = self.points.len();
        let curves = self.curves.len();
        let requested = points.saturating_add(curves);
        if requested > self.max_new_entities {
            return Err(ImportFailure::Limit {
                requested,
                maximum: self.max_new_entities,
            });
        }
        if let Some(expected) = self.curves.count_violation(points) {
            return Err(ImportFailure::Counts {
                points,
                curves,
                expected,
            });
        }
        Ok(requested)
    }
}

impl ImportCurves<'_> {
    fn len(self) -> usize {
        match self {
            Self::Points => 0,
            Self::Polyline(lines) => lines.len(),
            Self::DrawingSegments(lines) => lines.len(),
            Self::FitCubic { .. } => 1,
        }
    }

    fn count_violation(self, points: usize) -> Option<&'static str> {
        match self {
            Self::Points if points == 0 => Some("at least one point"),
            Self::Polyline(lines) if points < 2 || lines.len() != points - 1 => {
                Some("at least two points and one reserved line per consecutive pair")
            }
            Self::DrawingSegments(lines) if points < 2 || lines.is_empty() => {
                Some("at least two points and one directed drawing segment")
            }
            Self::FitCubic { .. } if points != 4 => Some("four ordered fit stations"),
            _ => None,
        }
    }

    fn lower(
        self,
        rows: &[PointId],
        edits: &mut Vec<Edit>,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Vec<CurveId>, ImportFailure> {
        match self {
            Self::Points => Ok(Vec::new()),
            Self::Polyline(lines) => polyline(rows, lines, edits, cancelled),
            Self::DrawingSegments(lines) => drawing(rows, lines, edits, cancelled),
            Self::FitCubic {
                id,
                parameters,
                policy,
            } => add_fit(rows, id, parameters, policy, edits, cancelled),
        }
    }
}

fn polyline(
    rows: &[PointId],
    lines: &[LineId],
    edits: &mut Vec<Edit>,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Vec<CurveId>, ImportFailure> {
    rows.array_windows::<2>()
        .zip(lines)
        .map(|(&endpoints, &id)| {
            poll(cancelled)?;
            edits.push(Edit::InsertLine(SketchLine { id, endpoints }));
            Ok(CurveId::Line(id))
        })
        .collect()
}

fn drawing(
    rows: &[PointId],
    lines: &[SketchLine],
    edits: &mut Vec<Edit>,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Vec<CurveId>, ImportFailure> {
    let mut imported = BTreeSet::new();
    for &point in rows {
        poll(cancelled)?;
        imported.insert(point);
    }
    lines
        .iter()
        .map(|&line| {
            poll(cancelled)?;
            for endpoint in line.endpoints {
                if !imported.contains(&endpoint) {
                    return Err(ImportFailure::ForeignEndpoint {
                        line: line.id,
                        point: endpoint,
                    });
                }
            }
            edits.push(Edit::InsertLine(line));
            Ok(CurveId::Line(line.id))
        })
        .collect()
}

fn add_fit(
    rows: &[PointId],
    id: FitSplineId,
    parameters: FitSplineParameters,
    policy: FitSplinePolicy,
    edits: &mut Vec<Edit>,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Vec<CurveId>, ImportFailure> {
    poll(cancelled)?;
    let stations = rows.try_into().map_err(|_| ImportFailure::Counts {
        points: rows.len(),
        curves: 1,
        expected: "four ordered fit stations",
    })?;
    edits.push(Edit::InsertFitSpline(SketchFitSpline {
        id,
        stations,
        parameters,
        policy,
    }));
    Ok(vec![CurveId::FitSpline(id)])
}

fn poll(cancelled: &mut impl FnMut() -> bool) -> Result<(), ImportFailure> {
    if cancelled() {
        return Err(ImportFailure::Cancelled);
    }
    Ok(())
}
