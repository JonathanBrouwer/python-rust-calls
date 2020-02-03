use pyo3::prelude::*;

#[pyclass]
struct DummyClass {}

#[pymethods]
impl DummyClass {
    #[staticmethod]
    fn get_42() -> PyResult<usize> {
        Ok(42)
    }
}

#[pyfunction]
fn incrementer_one(inp: usize) -> usize {
    inp + 1
}

#[pymodule]
fn increment(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<DummyClass>()?;

    Ok(())
}
