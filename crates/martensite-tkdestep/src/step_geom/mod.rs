//! `StepGeom`: the STEP entity classes of this package.

mod coordinates;
mod entities_01;
mod entities_02;
mod entities_03;
mod entities_04;
mod entities_05;
mod entities_06;
mod entities_07;
mod entities_08;
mod entities_09;
mod entities_10;
mod entities_11;
mod entities_12;
mod trimming;
pub use coordinates::*;
pub use entities_01::*;
pub use entities_02::*;
pub use entities_03::*;
pub use entities_04::*;
pub use entities_05::*;
pub use entities_06::*;
pub use entities_07::*;
pub use entities_08::*;
pub use entities_09::*;
pub use entities_10::*;
pub use entities_11::*;
pub use entities_12::*;
pub use trimming::*;
#[cfg(test)]
mod tests_01;
#[cfg(test)]
mod tests_02;
#[cfg(test)]
mod tests_03;
#[cfg(test)]
mod tests_04;
#[cfg(test)]
mod tests_05;
