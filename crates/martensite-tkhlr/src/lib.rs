//! Hidden line removal: interval sets, the polyhedral and exact hidden line algorithms,
//! contour lines of surfaces seen from a direction or a point, and the conversion of the
//! results back to shapes.
//!
//! The crate builds on the boundary representation and the geometric and topological
//! algorithm crates. Each family of algorithms is one module in snake case (`Intrv` is
//! `intrv`, `TopBas` is `top_bas`, `HLRAlgo` is `hlr_algo`, `HLRTopoBRep` is
//! `hlr_topo_brep`).
pub mod contap;
pub mod hlr_algo;
pub mod hlr_appli;
pub mod hlr_brep;
pub mod hlr_topo_brep;
pub mod intrv;
pub mod top_bas;
pub mod top_cnx;
