// .

//! The extrema of the distance between two surfaces: the function FuncExtSS, the
//! sampled search GenExtSS and the general class ExtSS.
mod ext_ss;
mod func_ext_ss;
mod gen_ext_ss;

pub use ext_ss::ExtSS;
pub use func_ext_ss::FuncExtSS;
pub use gen_ext_ss::GenExtSS;
