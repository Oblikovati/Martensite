//! Aspect grid snapping.
use super::common::{Reader, attempt, end, real};
use super::print::Output;
use martensite_tkernel::impl_transient;
use martensite_tkernel::standard::StandardTransient;
use martensite_tkservice::aspect::{
    AspectCircularGrid, AspectCircularGridBase, AspectCircularGridMembers, AspectGrid,
    AspectGridBase, AspectGridMembers, AspectRectangularGrid, AspectRectangularGridBase,
};

/// The do-nothing rectangular grid of the native probe (`SvRectGrid`).
struct SvRectGrid {
    transient: StandardTransient,
    base: AspectRectangularGridBase,
}
impl_transient!(SvRectGrid, "SvRectGrid", dyn AspectRectangularGrid, transient,
    interfaces: [dyn AspectGrid, dyn AspectRectangularGrid]);
impl AspectGrid for SvRectGrid {
    fn grid_base(&self) -> &AspectGridBase {
        self.base.grid()
    }
    fn compute(&self, x: f64, y: f64) -> (f64, f64) {
        self.base.compute(x, y)
    }
    fn display(&self) {}
    fn erase(&self) {}
    fn is_displayed(&self) -> bool {
        false
    }
    fn init(&self) {
        self.base.init();
    }
    fn update_display(&self) {}
}
impl AspectRectangularGrid for SvRectGrid {
    fn rectangular_base(&self) -> &AspectRectangularGridBase {
        &self.base
    }
}

/// The do-nothing circular grid of the native probe (`SvCircGrid`).
struct SvCircGrid {
    transient: StandardTransient,
    base: AspectCircularGridBase,
}
impl_transient!(SvCircGrid, "SvCircGrid", dyn AspectCircularGrid, transient,
    interfaces: [dyn AspectGrid, dyn AspectCircularGrid]);
impl AspectGrid for SvCircGrid {
    fn grid_base(&self) -> &AspectGridBase {
        self.base.grid()
    }
    fn compute(&self, x: f64, y: f64) -> (f64, f64) {
        self.base.compute(x, y)
    }
    fn display(&self) {}
    fn erase(&self) {}
    fn is_displayed(&self) -> bool {
        false
    }
    fn init(&self) {
        self.base.init();
    }
    fn update_display(&self) {}
}
impl AspectCircularGrid for SvCircGrid {
    fn circular_base(&self) -> &AspectCircularGridBase {
        &self.base
    }
}

/// `sv_rgrid XStep YStep XOrigin YOrigin FirstAngle SecondAngle Rotation X Y`.
pub fn rectangular_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let p: Vec<f64> = (0..9).map(|_| r.real()).collect();
    attempt(output, || {
        let base = AspectRectangularGridBase::new(p[0], p[1], p[2], p[3], p[4], p[5], p[6])?;
        let grid = SvRectGrid {
            transient: StandardTransient::default(),
            base,
        };
        grid.init();
        let (gx, gy) = grid.compute(p[7], p[8]);
        real(output, gx);
        real(output, gy);
        grid.activate();
        let (hx, hy) = grid.hit(p[7], p[8]);
        real(output, hx);
        real(output, hy);
        Ok(())
    });
    end(output);
}

/// `sv_cgrid RadiusStep Divisions XOrigin YOrigin Rotation X Y`.
pub fn circular_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let step = r.real();
    let divisions = r.integer();
    let p: Vec<f64> = (0..5).map(|_| r.real()).collect();
    attempt(output, || {
        let base = AspectCircularGridBase::new(1.0, 1, 0.0, 0.0, 0.0);
        let grid = SvCircGrid {
            transient: StandardTransient::default(),
            base,
        };
        grid.set_grid_values(p[0], p[1], step, divisions, p[2])?;
        let (gx, gy) = grid.compute(p[3], p[4]);
        real(output, gx);
        real(output, gy);
        Ok(())
    });
    end(output);
}
