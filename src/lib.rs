pub mod ahrs;
pub mod increment;

use crate::increment::increment::PyInit_increment;
use crate::ahrs::python_interface::PyInit_ahrs;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
fn lobster_rust(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_wrapped(wrap_pymodule!(increment))?;
    module.add_wrapped(wrap_pymodule!(ahrs))?;
    Ok(())
}