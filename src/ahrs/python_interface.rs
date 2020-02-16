use pyo3::prelude::*;
use crate::ahrs::madgwick_filter::Madgwick;
use nalgebra::{Quaternion, Vector3};
use crate::ahrs::ahrs::Ahrs;

use pyo3::create_exception;
create_exception!(module, MadgwickError, pyo3::exceptions::Exception);

#[pyclass]
pub struct MadgwickP {
    mw: Madgwick<f64>
}

#[pymethods]
impl MadgwickP {
    /// Creates a new `Madgwick` AHRS instance with identity quaternion.
    ///
    /// # Arguments
    ///
    /// * `sample_period` - The expected sensor sampling period in seconds.
    /// * `beta` - Filter gain.
    ///
    /// # Example
    ///
    /// ```
    /// use lobster_rust::ahrs::madgwick_filter::Madgwick;
    ///
    /// fn main() {
    ///     let ahrs = Madgwick::new(0.002390625f64, 0.1);
    /// }
    /// ```
    #[new]
    pub fn new(sample_period: f64, beta: f64) -> Self {
        MadgwickP { mw: Madgwick::new(sample_period, beta) }
    }

    /// Creates a new `Madgwick` AHRS instance with given quaternion.
    ///
    /// # Arguments
    ///
    /// * `sample_period` - The expected sensor sampling period in seconds.
    /// * `beta` - Filter gain.
    /// * `quat` - Existing filter state quaternion.
    ///
    /// # Example
    ///
    /// ```
    /// extern crate nalgebra as na;
    ///
    /// use na::Quaternion;
    /// use lobster_rust::ahrs::madgwick_filter::Madgwick;
    ///
    /// fn main() {
    ///     let ahrs = Madgwick::new_with_quat(
    ///         0.002390625f64,
    ///         0.1,
    ///         Quaternion::new(1.0, 0.0, 0.0, 0.0)
    ///     );
    /// }
    /// ```
    #[new]
    pub fn new_with_quat(sample_period: f64, beta: f64, quat: Vec<f64>) -> Self {
        assert_eq!(quat.len(), 4);
        MadgwickP { mw: Madgwick::new_with_quat(sample_period, beta, Quaternion::new(quat[0], quat[1], quat[2], quat[3])) }
    }
}

impl Default for MadgwickP {
    /// Creates a new `Madgwick` instance with default filter parameters:
    ///
    /// ```rust,ignore
    /// Madgwick {
    ///     sample_period: 1.0f64/256.0,
    ///     beta: 0.1f64,
    ///     quat: Quaternion { w: 1.0f64, i: 0.0, j: 0.0, k: 0.0 }
    /// }
    /// ```
    fn default() -> MadgwickP {
        MadgwickP{ mw: Madgwick::<f64>::default() }
    }
}

#[pymethods]
impl MadgwickP {
    fn update(
        &mut self,
        gyroscope: Vec<f64>,
        accelerometer: Vec<f64>,
        magnetometer: Vec<f64>,
    ) -> PyResult<Vec<f64>> {
        //Convert gyro, accel, magneto to nalgebra vectors
        assert_eq!(gyroscope.len(), 3);
        let g3: Vector3<f64> = Vector3::new(gyroscope[0], gyroscope[1], gyroscope[2]);

        assert_eq!(accelerometer.len(), 3);
        let a3: Vector3<f64> = Vector3::new(accelerometer[0], accelerometer[1], accelerometer[2]);

        assert_eq!(magnetometer.len(), 3);
        let m3: Vector3<f64> = Vector3::new(magnetometer[0], magnetometer[1], magnetometer[2]);

        //Update madgwick
        match self.mw.update(&g3, &a3, &m3) {
            Ok(quat) => PyResult::Ok(vec!(quat.i, quat.j, quat.k, quat.w)),
            Err(e) => PyResult::Err(PyErr::new::<MadgwickError, _>(e))
        }
    }

    fn update_imu(
        &mut self,
        gyroscope: Vec<f64>,
        accelerometer: Vec<f64>
    ) -> PyResult<Vec<f64>> {
        //Convert gyro, accel, magneto to nalgebra vectors
        assert_eq!(gyroscope.len(), 3);
        let g3: Vector3<f64> = Vector3::new(gyroscope[0], gyroscope[1], gyroscope[2]);

        assert_eq!(accelerometer.len(), 3);
        let a3: Vector3<f64> = Vector3::new(accelerometer[0], accelerometer[1], accelerometer[2]);

        //Update madgwick
        match self.mw.update_imu(&g3, &a3) {
            Ok(quat) => PyResult::Ok(vec!(quat.i, quat.j, quat.k, quat.w)),
            Err(e) => PyResult::Err(PyErr::new::<MadgwickError, _>(e))
        }
    }
}

#[pymodule]
fn ahrs(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<MadgwickP>()?;

    Ok(())
}