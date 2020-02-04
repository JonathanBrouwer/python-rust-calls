use pyo3::prelude::*;

#[pyclass]
pub struct Incrementer {
    by: usize
}

#[pymethods]
impl Incrementer {
    #[new]
    pub fn new(by: usize) -> Self {
        Incrementer { by }
    }

    pub fn apply(&self, inp: usize) -> usize {
        inp + self.by
    }
}
