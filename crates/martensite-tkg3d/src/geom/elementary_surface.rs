//! `GeomElementarySurface`: planes and quadrics placed by a Axis3 (the retained
//! protected `pos`).
use super::protected::{PROTECTED, ProtectedAccess};
use super::surface::{GeomSurface, dump_surface};
use martensite_tkernel::impl_transient_interface;
use martensite_tkernel::standard::Failure;
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkg2d::geom2d::{dump_base, dump_class_name, dump_field};
use martensite_tkmath::gp::{Axis1, Axis3, Point3};
use std::io;

/// A surface in its local coordinate system; the main axis is the axis of
/// revolution or the normal of a plane; `surface.set_location(Point3::new(1.0, 0.0, 0.0))`.
pub trait GeomElementarySurface: GeomSurface {
    /// The local coordinate system (`Position`); `surface.position()`.
    fn position(&self) -> Axis3;
    /// Change the protected `pos` under the object's lock; only Geom calls it.
    #[doc(hidden)]
    fn change_position(&self, access: ProtectedAccess, change: &mut dyn FnMut(&mut Axis3));
    /// Change the protected `pos` by a step that can raise; only Geom calls it.
    #[doc(hidden)]
    fn try_change_position(
        &self,
        access: ProtectedAccess,
        change: &mut dyn FnMut(&mut Axis3) -> Result<(), Failure>,
    ) -> Result<(), Failure>;
    /// Change the main axis, keeping X and Y as close as possible (`SetAxis`);
    /// StandardConstructionError when it is parallel to the old X direction;
    /// `surface.set_axis(axis)?`.
    fn set_axis(&self, axis: Axis1) -> Result<(), Failure> {
        self.try_change_position(PROTECTED, &mut |position| position.set_axis(axis))
    }
    /// Move the origin (`SetLocation`); `surface.set_location(p)`.
    fn set_location(&self, point: Point3) {
        self.change_position(PROTECTED, &mut |position| position.set_location(point));
    }
    /// Replace the local coordinate system (`SetPosition`); `surface.set_position(frame)`.
    fn set_position(&self, frame: Axis3) {
        self.change_position(PROTECTED, &mut |position| *position = frame);
    }
    /// The main axis (`Axis`); `surface.axis()`.
    fn axis(&self) -> Axis1 {
        self.position().axis()
    }
    /// The origin (`Location`); `surface.location()`.
    fn location(&self) -> Point3 {
        self.position().location()
    }
}
impl_transient_interface!(
    dyn GeomElementarySurface,
    "Geom_ElementarySurface",
    dyn GeomSurface
);

/// `GeomElementarySurface::UReverse`: reverse the Y direction of the frame;
/// `elementary_u_reverse(&mut pos)`.
pub fn elementary_u_reverse(position: &mut Axis3) {
    position.y_reverse();
}

/// `GeomElementarySurface::VReverse`: reverse the main direction of the frame;
/// `elementary_v_reverse(&mut pos)`.
pub fn elementary_v_reverse(position: &mut Axis3) {
    position.z_reverse();
}

/// `GeomElementarySurface::DumpJson`; `dump_elementary_surface(&mut stream, -1, pos)?`.
pub fn dump_elementary_surface(
    stream: &mut DumpStream,
    depth: i32,
    position: Axis3,
) -> io::Result<()> {
    dump_class_name(stream, "Geom_ElementarySurface")?;
    dump_base(stream, depth, dump_surface)?;
    dump_field(stream, depth, "pos", |nested, _| position.dump_json(nested))
}

/// Implement the protected position accessors of an elementary surface whose
/// state has a `pos` field.
macro_rules! impl_elementary_position {
    () => {
        fn position(&self) -> martensite_tkmath::gp::Axis3 {
            self.state.read().pos
        }
        fn change_position(
            &self,
            _access: $crate::geom::protected::ProtectedAccess,
            change: &mut dyn FnMut(&mut martensite_tkmath::gp::Axis3),
        ) {
            change(&mut self.state.write().pos);
        }
        fn try_change_position(
            &self,
            _access: $crate::geom::protected::ProtectedAccess,
            change: &mut dyn FnMut(
                &mut martensite_tkmath::gp::Axis3,
            ) -> Result<(), martensite_tkernel::standard::Failure>,
        ) -> Result<(), martensite_tkernel::standard::Failure> {
            change(&mut self.state.write().pos)
        }
    };
}
pub(crate) use impl_elementary_position;
