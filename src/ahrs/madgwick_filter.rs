extern crate nalgebra as na;
extern crate alga;

use alga::general::RealField;
use na::{Matrix4, Matrix6, Quaternion, Vector2, Vector3, Vector4, Vector6};
use crate::ahrs::ahrs::Ahrs;

/// Madgwick AHRS implementation.
#[derive(Eq, PartialEq, Clone, Debug, Hash, Copy)]
pub struct Madgwick<N: RealField> {
    /// Expected sampling period, in seconds.
    sample_period: N,
    /// Filter gain.
    beta: N,
    /// Filter state quaternion.
    pub quat: Quaternion<N>,
}

impl Default for Madgwick<f64> {
    /// Creates a new `Madgwick` instance with default filter parameters:
    ///
    /// ```rust,ignore
    /// Madgwick {
    ///     sample_period: 1.0f64/256.0,
    ///     beta: 0.1f64,
    ///     quat: Quaternion { w: 1.0f64, i: 0.0, j: 0.0, k: 0.0 }
    /// }
    /// ```
    fn default() -> Madgwick<f64> {
        Madgwick {
            sample_period: (1.0f64) / (256.0),
            beta: 0.1f64,
            quat: Quaternion::new(1.0f64, 0.0, 0.0, 0.0),
        }
    }
}

impl<N: RealField> Madgwick<N> {
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
    /// extern crate lobster_rust;
    ///
    /// use lobster_rust::ahrs::madgwick_filter::Madgwick;
    ///
    /// fn main() {
    ///     let ahrs = Madgwick::new(0.002390625f64, 0.1);
    /// }
    /// ```
    pub fn new(sample_period: N, beta: N) -> Self {
        Madgwick::new_with_quat(
            sample_period,
            beta,
            Quaternion::new(N::one(), N::zero(), N::zero(), N::zero()),
        )
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
    pub fn new_with_quat(sample_period: N, beta: N, quat: Quaternion<N>) -> Self {
        Madgwick {
            sample_period: sample_period,
            beta: beta,
            quat: quat,
        }
    }
}

#[cfg(feature = "field_access")]
impl<N: RealField> Madgwick<N> {
    /// Expected sampling period, in seconds.
    pub fn sample_period(&self) -> N {
        self.sample_period
    }

    /// Mutable reference to expected sampling period, in seconds.
    pub fn sample_period_mut(&mut self) -> &mut N {
        &mut self.sample_period
    }

    /// Filter gain.
    pub fn beta(&self) -> N {
        self.beta
    }

    /// Mutable reference to filter gain.
    pub fn beta_mut(&mut self) -> &mut N {
        &mut self.beta
    }

    /// Filter state quaternion.
    pub fn quat(&self) -> Quaternion<N> {
        self.quat
    }

    /// Mutable reference to filter state quaternion.
    pub fn quat_mut(&mut self) -> &mut Quaternion<N> {
        &mut self.quat
    }
}

impl<N: RealField> Ahrs<N> for Madgwick<N> {
    fn update(
        &mut self,
        gyroscope: &Vector3<N>,
        accelerometer: &Vector3<N>,
        magnetometer: &Vector3<N>,
    ) -> Result<&Quaternion<N>, &'static str> {
        let q = self.quat;

        let zero: N = na::zero();
        let two: N = na::convert(2.0);
        let four: N = na::convert(4.0);
        let half: N = na::convert(0.5);

        // Normalize accelerometer measurement
        let accel = match accelerometer.try_normalize(zero) {
            Some(n) => n,
            None => return Err("Accelerometer norm divided by zero."),
        };

        // Normalize magnetometer measurement
        let mag = match magnetometer.try_normalize(zero) {
            Some(n) => n,
            None => {
                return Err("Magnetometer norm divided by zero.");
            }
        };

        // Reference direction of Earth's magnetic field (Quaternion should still be conj of q)
        let h = q * (Quaternion::from_parts(zero, mag.clone()) * q.conjugate());
        let b = Quaternion::new(zero, Vector2::new(h[0], h[1]).norm(), zero, h[2]);

        // Gradient descent algorithm corrective step
        let f = Vector6::new(
            two * (q[0] * q[2] - q[3] * q[1]) - accel[0],
            two * (q[3] * q[0] + q[1] * q[2]) - accel[1],
            two * (half - q[0] * q[0] - q[1] * q[1]) - accel[2],
            two * b[0] * (half - q[1] * q[1] - q[2] * q[2]) + two * b[2] * (q[0] * q[2] - q[3] * q[1]) - mag[0],
            two * b[0] * (q[0] * q[1] - q[3] * q[2]) + two * b[2] * (q[3] * q[0] + q[1] * q[2]) - mag[1],
            two * b[0] * (q[3] * q[1] + q[0] * q[2]) + two * b[2] * (half - q[0] * q[0] - q[1] * q[1]) - mag[2],
        );

        let j_t = Matrix6::new(
            -two * q[1], two * q[0], zero, -two * b[2] * q[1], -two * b[0] * q[2] + two * b[2] * q[0], two * b[0] * q[1],
            two * q[2], two * q[3], -four * q[0], two * b[2] * q[2], two * b[0] * q[1] + two * b[2] * q[3], two * b[0] * q[2] - four * b[2] * q[0],
            -two * q[3], two * q[2], -four * q[1], -four * b[0] * q[1] - two * b[2] * q[3], two * b[0] * q[0] + two * b[2] * q[2], two * b[0] * q[3] - four * b[2] * q[1],
            two * q[0], two * q[1], zero, -four * b[0] * q[2] + two * b[2] * q[0], -two * b[0] * q[3] + two * b[2] * q[1], two * b[0] * q[0],
            zero, zero, zero, zero, zero, zero,
            zero, zero, zero, zero, zero, zero,
        );

        let step = (j_t * f).normalize();

        // Compute rate of change for quaternion
        let q_dot = q * Quaternion::from_parts(zero, (*gyroscope).clone()) * half
            - Quaternion::new(step[0], step[1], step[2], step[3]) * self.beta;

        // Integrate to yield quaternion
        self.quat = (q + q_dot * self.sample_period).normalize();

        Ok(&self.quat)
    }

    fn update_imu(
        &mut self,
        gyroscope: &Vector3<N>,
        accelerometer: &Vector3<N>,
    ) -> Result<&Quaternion<N>, &'static str> {
        let q = self.quat;

        let zero: N = na::zero();
        let two: N = na::convert(2.0);
        let four: N = na::convert(4.0);
        let half: N = na::convert(0.5);

        // Normalize accelerometer measurement
        let accel = match accelerometer.try_normalize(zero) {
            Some(n) => n,
            None => {
                return Err("Accelerator norm divided by zero.");
            }
        };

        // Gradient descent algorithm corrective step
        let f = Vector4::new(
            two * (q[0] * q[2] - q[3] * q[1]) - accel[0],
            two * (q[3] * q[0] + q[1] * q[2]) - accel[1],
            two * (half - q[0] * q[0] - q[1] * q[1]) - accel[2],
            zero,
        );

        let j_t = Matrix4::new(
            -two * q[1], two * q[0], zero, zero,
            two * q[2], two * q[3], -four * q[0], zero,
            -two * q[3], two * q[2], -four * q[1], zero,
            two * q[0], two * q[1], zero, zero,
        );

        let step = (j_t * f).normalize();

        // Compute rate of change of quaternion
        let q_dot = (q * Quaternion::from_parts(zero, (*gyroscope).clone())) * half
            - Quaternion::new(step[0], step[1], step[2], step[3]) * self.beta;

        // Integrate to yield quaternion
        self.quat = (q + q_dot * self.sample_period).normalize();

        Ok(&self.quat)
    }
}
