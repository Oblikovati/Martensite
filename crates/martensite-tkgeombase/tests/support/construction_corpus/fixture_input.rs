//! The geometric fixture builders consume typed fields without choosing a Reader identity.
pub trait GeometryFixtureInput {
    fn integer(&mut self) -> i32;
    fn real(&mut self) -> f64;
    fn flag(&mut self) -> bool;
    fn vec(&mut self) -> martensite_tkmath::gp::Vector3;
    fn vec2d(&mut self) -> martensite_tkmath::gp::Vector2;
    fn pnt2d(&mut self) -> martensite_tkmath::gp::Point2;
    fn dir2d(&mut self) -> martensite_tkmath::gp::Direction2;
    fn ax22d(&mut self) -> martensite_tkmath::gp::Axis22d;
    fn ax2d(&mut self) -> martensite_tkmath::gp::Axis2d;
    fn pnt(&mut self) -> martensite_tkmath::gp::Point3;
    fn dir(&mut self) -> martensite_tkmath::gp::Direction3;
    fn ax1(&mut self) -> martensite_tkmath::gp::Axis1;
    fn ax2(&mut self) -> martensite_tkmath::gp::Axis2;
    fn ax3(&mut self) -> martensite_tkmath::gp::Axis3;
}

// Each adapter delegates to the same inherent method on its existing reader type.
macro_rules! impl_geometry_fixture_input {
    ($reader:ty, $contract:path) => {
        impl $contract for $reader {
            fn integer(&mut self) -> i32 {
                <$reader>::integer(self)
            }
            fn real(&mut self) -> f64 {
                <$reader>::real(self)
            }
            fn flag(&mut self) -> bool {
                <$reader>::flag(self)
            }
            fn vec(&mut self) -> martensite_tkmath::gp::Vector3 {
                <$reader>::vec(self)
            }
            fn vec2d(&mut self) -> martensite_tkmath::gp::Vector2 {
                <$reader>::vec2d(self)
            }
            fn pnt2d(&mut self) -> martensite_tkmath::gp::Point2 {
                <$reader>::pnt2d(self)
            }
            fn dir2d(&mut self) -> martensite_tkmath::gp::Direction2 {
                <$reader>::dir2d(self)
            }
            fn ax22d(&mut self) -> martensite_tkmath::gp::Axis22d {
                <$reader>::ax22d(self)
            }
            fn ax2d(&mut self) -> martensite_tkmath::gp::Axis2d {
                <$reader>::ax2d(self)
            }
            fn pnt(&mut self) -> martensite_tkmath::gp::Point3 {
                <$reader>::pnt(self)
            }
            fn dir(&mut self) -> martensite_tkmath::gp::Direction3 {
                <$reader>::dir(self)
            }
            fn ax1(&mut self) -> martensite_tkmath::gp::Axis1 {
                <$reader>::ax1(self)
            }
            fn ax2(&mut self) -> martensite_tkmath::gp::Axis2 {
                <$reader>::ax2(self)
            }
            fn ax3(&mut self) -> martensite_tkmath::gp::Axis3 {
                <$reader>::ax3(self)
            }
        }
    };
}
pub(crate) use impl_geometry_fixture_input;
self::impl_geometry_fixture_input!(super::common::Reader<'_>, GeometryFixtureInput);
