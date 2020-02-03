use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyclass]
struct Incrementer {
    by: usize
}

#[pymethods]
impl Incrementer {
    #[new]
    fn new(by: usize) -> Self {
        Incrementer { by }
    }

    fn increment_by(&self, inp: usize) -> PyResult<usize> {
        Ok(inp + self.by)
    }
}

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
