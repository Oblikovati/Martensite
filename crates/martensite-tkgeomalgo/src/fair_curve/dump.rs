//! `Dump` of the batten and the minimal-variation curve: one row per constraint
//! with the computed ("Old") and requested ("New") values. As natively,
//! `o.width(7)` pads only the old value; reals use the general stream format at the
//! stream precision (6 by default, as `%g`).
use super::batten::BattenConstraints;
use super::{FairCurveAnalysisCode, FairCurveBatten, FairCurveMinimalVariation};
use martensite_tkernel::storage::format_general;
use std::io;

/// The precision of the default stream format.
const DEFAULT_PRECISION: usize = 6;

/// A real in the general stream format at `precision` significant digits.
fn g(value: f64, precision: usize) -> String {
    format_general(value, precision)
}

/// `label`, the old value padded to 7 characters, ` | ` and the new value.
fn row(o: &mut impl io::Write, label: &str, old: String, new: String) -> io::Result<()> {
    writeln!(o, "{label}{old:>7} | {new}")
}

/// The rows of the end points and angles.
fn point_rows(
    o: &mut impl io::Write,
    p: usize,
    old: &BattenConstraints,
    new: &BattenConstraints,
) -> io::Result<()> {
    row(o, "  P1    X      |", g(old.p1.x(), p), g(new.p1.x(), p))?;
    row(o, "        Y      |", g(old.p1.y(), p), g(new.p1.y(), p))?;
    row(o, "  P2    X      |", g(old.p2.x(), p), g(new.p2.x(), p))?;
    row(o, "        Y      |", g(old.p2.y(), p), g(new.p2.y(), p))?;
    row(o, "      Angle1   |", g(old.angle1, p), g(new.angle1, p))?;
    row(o, "      Angle2   |", g(old.angle2, p), g(new.angle2, p))
}

/// The rows of the height and slope.
fn height_rows(
    o: &mut impl io::Write,
    p: usize,
    old: &BattenConstraints,
    new: &BattenConstraints,
) -> io::Result<()> {
    row(o, "      Height   |", g(old.height, p), g(new.height, p))?;
    row(o, "      Slope    |", g(old.slope, p), g(new.slope, p))
}

/// The rows of the sliding and constraint orders, then the analysis code.
fn tail_rows(o: &mut impl io::Write, p: usize, batten: &FairCurveBatten) -> io::Result<()> {
    let (old, new) = (&batten.old, &batten.new);
    row(
        o,
        " SlidingFactor |",
        g(old.sliding_factor, p),
        g(new.sliding_factor, p),
    )?;
    let flag = |value: bool| i32::from(value).to_string();
    row(
        o,
        " FreeSliding   |",
        flag(old.free_sliding),
        flag(new.free_sliding),
    )?;
    let (old1, new1) = (old.constraint_order1, new.constraint_order1);
    row(o, " ConstrOrder1  |", old1.to_string(), new1.to_string())?;
    let (old2, new2) = (old.constraint_order2, new.constraint_order2);
    row(o, " ConstrOrder2  |", old2.to_string(), new2.to_string())?;
    let code: &str = match batten.code {
        FairCurveAnalysisCode::Ok => "Ok",
        FairCurveAnalysisCode::NotConverged => "NotConverged",
        FairCurveAnalysisCode::InfiniteSliding => "InfiniteSliding",
        FairCurveAnalysisCode::NullHeight => "NullHeight",
    };
    writeln!(o, "AnalysisCode : {code}")
}

impl FairCurveBatten {
    /// Writes the native report of the constraints; `batten.dump(&mut std::io::stdout())?`.
    pub fn dump(&self, o: &mut impl io::Write) -> io::Result<()> {
        self.dump_with_precision(o, DEFAULT_PRECISION)
    }

    /// `Dump` on a stream whose precision is `p` (a `dump` uses 15);
    /// `batten.dump_with_precision(&mut std::io::stdout(), 15)?`.
    pub fn dump_with_precision(&self, o: &mut impl io::Write, p: usize) -> io::Result<()> {
        writeln!(o, "  Batten       |Old  |   New")?;
        point_rows(o, p, &self.old, &self.new)?;
        height_rows(o, p, &self.old, &self.new)?;
        tail_rows(o, p, self)
    }
}

impl FairCurveMinimalVariation {
    /// Writes the native report, with the curvatures and the physical ratio;
    /// `mvc.dump(&mut std::io::stdout())?`.
    pub fn dump(&self, o: &mut impl io::Write) -> io::Result<()> {
        self.dump_with_precision(o, DEFAULT_PRECISION)
    }

    /// `Dump` on a stream whose precision is `p`;
    /// `mvc.dump_with_precision(&mut std::io::stdout(), 15)?`.
    pub fn dump_with_precision(&self, o: &mut impl io::Write, p: usize) -> io::Result<()> {
        let batten: &FairCurveBatten = self;
        writeln!(o, "  MVCurve      |Old  |   New")?;
        point_rows(o, p, &batten.old, &batten.new)?;
        let (old, new) = (self.old_curvatures, self.new_curvatures);
        row(o, " Curvature1    |", g(old[0], p), g(new[0], p))?;
        row(o, " Curvature2    |", g(old[1], p), g(new[1], p))?;
        height_rows(o, p, &batten.old, &batten.new)?;
        row(
            o,
            " PhysicalRatio |",
            g(self.old_physical_ratio, p),
            g(self.new_physical_ratio, p),
        )?;
        tail_rows(o, p, batten)
    }
}
