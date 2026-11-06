//! Positive endpoint indices for an intersection interval.
use martensite_tkernel::standard::{Failure, exceptions::program_error};

/// Two endpoint indices, checked when read; `TopOpeBRepBipoint::new(1, 2).i1()?`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TopOpeBRepBipoint {
    first: i32,
    second: i32,
}

impl TopOpeBRepBipoint {
    /// Stores both indices without validation; `TopOpeBRepBipoint::new(3, 7)`.
    pub fn new(first: i32, second: i32) -> Self {
        Self { first, second }
    }

    /// Reads the first positive index; `TopOpeBRepBipoint::new(3, 7).i1()?`.
    pub fn i1(&self) -> Result<i32, Failure> {
        positive_endpoint(self.first, "I1")
    }

    /// Reads the second positive index; `TopOpeBRepBipoint::new(3, 7).i2()?`.
    pub fn i2(&self) -> Result<i32, Failure> {
        positive_endpoint(self.second, "I2")
    }
}

fn positive_endpoint(index: i32, endpoint: &str) -> Result<i32, Failure> {
    if index <= 0 {
        // The compatibility diagnostic says zero for every nonpositive endpoint.
        return Err(program_error(&format!("TopOpeBRepBipoint {endpoint}=0")));
    }
    Ok(index)
}
