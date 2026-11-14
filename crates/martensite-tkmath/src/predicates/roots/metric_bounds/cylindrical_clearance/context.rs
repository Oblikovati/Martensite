// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use bounds::HullBox;
use report::Statistics;
use std::collections::BTreeSet;

pub(super) struct CurveNode {
    pub poles: Vec<Point>,
    pub bounds: HullBox,
    pub index: u64,
    pub level: u8,
    pub children: Option<[usize; 2]>,
    pub midpoint: Option<Point>,
}
impl CurveNode {
    pub fn new(poles: Vec<Point>, index: u64, level: u8) -> Self {
        Self {
            bounds: HullBox::of(&poles),
            poles,
            index,
            level,
            children: None,
            midpoint: None,
        }
    }
}
pub(super) struct CurveTree {
    pub nodes: Vec<CurveNode>,
}
#[derive(Clone, Copy)]
pub(super) enum HierarchyKind {
    Leaf(usize),
    Branch([usize; 2]),
}
pub(super) struct HierarchyNode {
    pub bounds: HullBox,
    pub count: usize,
    pub kind: HierarchyKind,
}

pub(super) struct Context<'a, 'q> {
    pub arithmetic: Arithmetic<'a>,
    pub query: CylindricalClearanceQuery<'q>,
    pub curves: Vec<CurveTree>,
    pub hierarchy: Vec<HierarchyNode>,
    pub excluded: BTreeSet<[usize; 2]>,
    pub shift: [Interval; 2],
    pub delta: Interval,
    pub delta_square: Interval,
    pub statistics: Statistics,
}
impl<'a, 'q> Context<'a, 'q> {
    pub fn capture(
        query: CylindricalClearanceQuery<'q>,
        excluded: BTreeSet<[usize; 2]>,
        cancelled: &'a mut (dyn FnMut() -> bool + Send),
    ) -> Checked<Self> {
        let mut arithmetic = Arithmetic::new(
            ArithmeticPolicy {
                fractional_bits: query.options.fractional_bits,
                max_operations: query.options.max_operations,
            },
            cancelled,
        );
        let delta = arithmetic.scalar(query.requested_clearance)?;
        let delta_square = arithmetic.multiply(&delta, &delta)?;
        let offset = query.domain.offset();
        let shift = [arithmetic.scalar(offset[0])?, arithmetic.scalar(offset[1])?];
        let mut result = Self {
            arithmetic,
            query,
            curves: Vec::new(),
            hierarchy: Vec::new(),
            excluded,
            shift,
            delta,
            delta_square,
            statistics: Statistics::default(),
        };
        result.capture_roots()?;
        Ok(result)
    }
    fn capture_roots(&mut self) -> Checked<()> {
        self.reserve_nodes(self.query.sources.len() as u32)?;
        for source in self.query.sources {
            let poles = capture_source(source, &mut self.arithmetic)?;
            self.curves.push(CurveTree {
                nodes: vec![CurveNode::new(poles, 0, 0)],
            });
            self.statistics.curve_nodes += 1;
        }
        Ok(())
    }
    pub fn tick(&mut self) -> Checked<()> {
        self.arithmetic.check_cancelled()?;
        if self.statistics.domains == self.query.options.max_domains {
            return Err(CylindricalClearanceFailure::DomainLimit {
                completed: self.statistics.domains,
                limit: self.query.options.max_domains,
            });
        }
        self.statistics.domains += 1;
        Ok(())
    }
    pub fn reserve_nodes(&self, additional: u32) -> Checked<()> {
        if self.statistics.curve_nodes + additional > self.query.options.max_curve_nodes {
            return Err(CylindricalClearanceFailure::NodeLimit {
                used: self.statistics.curve_nodes,
                additional,
                limit: self.query.options.max_curve_nodes,
            });
        }
        Ok(())
    }
}

fn capture_source(
    source: &CylindricalClearanceSource<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Vec<Point>> {
    let poles = curve::capture_planar(source.authored, arithmetic)?;
    let span = source.interval;
    let x = curve::restrict(&poles[0], span.index(), span.level(), arithmetic)?;
    let z = curve::restrict(&poles[1], span.index(), span.level(), arithmetic)?;
    Ok(curve::align_planar(&x, &z))
}
