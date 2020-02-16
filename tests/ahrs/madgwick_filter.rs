extern crate nalgebra as na;

use lobster_rust::ahrs::ahrs::Ahrs;
use lobster_rust::ahrs::madgwick_filter::Madgwick;
use na::{Vector3, Quaternion};
use std::f64;

// accel, gyro, mag values
macro_rules! default_sensors(
  () => {
    (
      Vector3::new(0.06640625, 0.9794922, -0.01269531),
      Vector3::new(68.75, 34.25, 3.0625),
      Vector3::new(0.171875, -0.4536133, -0.04101563)
    )
  };
);

#[test]
fn test_update_accel_zero() {

  let mut ahrs = Madgwick::default();

  let g: Vector3<f64> = Vector3::new(1.0, 1.0, 1.0);
  let a: Vector3<f64> = Vector3::new(0.0, 0.0, 0.0);
  let m: Vector3<f64> = Vector3::new(1.0, 1.0, 1.0);

  let res = ahrs.update(&g, &a, &m);

  let fail_message = "Normalizing zero-value accel should have failed.";

  assert!(res.is_err(), fail_message);
}

#[test]
fn test_update_mag_zero() {

  let mut ahrs = Madgwick::default();

  let g: Vector3<f64> = Vector3::new(1.0, 1.0, 1.0);
  let a: Vector3<f64> = Vector3::new(1.0, 1.0, 1.0);
  let m: Vector3<f64> = Vector3::new(0.0, 0.0, 0.0);

  let res = ahrs.update(&g, &a, &m);

  let fail_message = "Normalizing zero-value mag should have failed.";

  assert!(res.is_err(), fail_message);
}

#[test]
fn test_update_imu_accel_zero() {

  let mut ahrs = Madgwick::default();

  let g: Vector3<f64> = Vector3::new(1.0, 1.0, 1.0);
  let a: Vector3<f64> = Vector3::new(0.0, 0.0, 0.0);

  let res = ahrs.update_imu(&g, &a);


  let fail_message = "Normalizing zero-value accel should have failed.";

  assert!(res.is_err(), fail_message);
}

#[test]
fn test_madgwick_update() {

  let start_quat = Quaternion::new( 0.7252997863255918f64,
                                    0.6869689552600526,
                                   -0.04486780259245286,
                                    0.0008687666471569602);

  let mut ahrs = Madgwick::default();
  ahrs.quat = start_quat;

  let (accel, gyro, mag) = default_sensors!();

  let actual = ahrs.update(&(gyro * (f64::consts::PI/180.0)), &accel, &mag).unwrap();

  let expected = Quaternion::new( 0.7235467139148768,
                                  0.6888611247479446,
                                 -0.04412605927634125,
                                  0.001842413287185898);

  let fail_message = format!("quaternions did not match:\n\
        actual: {:?}\n\
        expect: {:?}", actual, expected);

  assert!(relative_eq!(actual, &expected), fail_message);
}

#[test]
fn test_madgwick_update_imu() {
    let start_quat = Quaternion::new(0.7208922848226422,
                                     0.6922487447935516,
                                     -0.01829063767755937,
                                     0.02777483732249482);

    let mut ahrs = Madgwick::default();
    ahrs.quat = start_quat;

    let (accel, gyro, _) = default_sensors!();

    let actual = ahrs.update_imu(&(gyro * (f64::consts::PI / 180.0)), &accel).unwrap();

    let expected = Quaternion::new(0.7190919791549198,
                                   0.694101991692336,
                                   -0.01747200330433749,
                                   0.02870330545992814);

    let fail_message = format!("quaternions did not match:\n\
      actual: {:?}\n\
      expect: {:?}", actual, expected);

    assert!(relative_eq!(actual, &expected), fail_message);
}

