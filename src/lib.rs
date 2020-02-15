extern crate nalgebra as na;
extern crate alga;

pub use incrementers::{
    Incrementer
};

pub mod incrementers;
pub mod ahrs;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn incrementer_one(inp: usize) -> usize {
    inp + 1
}

#[pymodule]
fn increment(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(incrementer_one))?;
    m.add_class::<Incrementer>()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{incrementer_one, Incrementer};

    #[test]
    fn test_increment_one() {
        assert_eq!(4, incrementer_one(3));
    }

    #[test]
    fn test_incrementer() {
        assert_eq!(7, Incrementer::new(5).apply(2));
    }
}
