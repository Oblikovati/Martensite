//! Pre-extraction specialized curve contracts shared by both hosts.
use super::bezier_api;
#[cfg(all(target_os = "linux", feature = "x11-integration-tests"))]
mod native;
mod state;
pub fn spatial() -> martensite_tkernel::standard::Handle<martensite_tkg3d::geom::GeomBezierCurve> {
    use martensite_tkernel::storage::IoStream;
    martensite_tkgeombase::geom_tools::read_curve(&mut IoStream::from_bytes(
        b"6 0 2 -5 0 0 0 5 0 5 0 0".to_vec(),
    ))
    .cast()
}
pub fn planar() -> martensite_tkernel::standard::Handle<martensite_tkg2d::geom2d::Geom2dBezierCurve>
{
    use martensite_tkernel::storage::IoStream;
    martensite_tkgeombase::geom_tools::read_curve2d(&mut IoStream::from_bytes(
        b"6 0 2 -5 0 0 5 5 0".to_vec(),
    ))
    .cast()
}
