use crate::equal;
use crate::vector::Vector;
use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }
}

impl From<[f64; 3]> for Point {
    fn from(array: [f64; 3]) -> Self {
        Self {
            x: array[0],
            y: array[1],
            z: array[2],
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        equal(self.x, other.x) && equal(self.y, other.y) && equal(self.z, other.z)
    }
}

impl ops::Add<Vector> for Point {
    type Output = Self;

    fn add(self, other: Vector) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Sub for Point {
    type Output = Vector;

    fn sub(self, other: Self) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl ops::Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, other: Vector) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Mul<f64> for Point {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl ops::Div<f64> for Point {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl ops::Neg for Point {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[test]
fn should_add_a_vector_to_a_point() {
    let v1 = Vector::new(3.0, -2.0, 5.0);
    let v2 = Vector::new(-2.0, 1.0, 1.0);

    let expected = Vector::new(1.0, -1.0, 6.0);
    let actual = v1 + v2;

    assert_eq!(expected, actual);
}

#[test]
fn should_subtract_two_points() {
    let p1 = Point::new(3.0, 2.0, 1.0);
    let p2 = Point::new(5.0, 6.0, 7.0);

    let expected = Vector::new(-2.0, -4.0, -6.0);
    let actual = p1 - p2;

    assert_eq!(expected, actual);
}

#[test]
fn should_create_tuple_that_is_a_point() {
    assert_eq!(
        Point::new(1.0, 1.0, 1.0),
        Point {
            x: 1.0,
            y: 1.0,
            z: 1.0
        }
    )
}

#[test]
fn should_subtract_vector_from_point() {
    let p1 = Point::new(3.0, 2.0, 1.0);
    let p2 = Vector::new(5.0, 6.0, 7.0);

    let expected = Point::new(-2.0, -4.0, -6.0);
    let actual = p1 - p2;

    assert_eq!(expected, actual);
}

#[test]
fn should_multiply_a_point_by_a_scalar() {
    let a = Point::new(1.0, -2.0, 3.0);

    let expected = Point::new(3.5, -7.0, 10.5);
    let actual = a * 3.5;

    assert_eq!(expected, actual);
}

#[test]
fn should_negate_a_point() {
    let a = Point::new(1.0, -2.0, 3.0);

    let expected = Point::new(-1.0, 2.0, -3.0);
    let actual = -a;

    assert_eq!(expected, actual);
}

#[test]
fn should_divide_a_point_by_a_scalar() {
    let a = Point::new(1.0, -2.0, 3.0);

    let expected = Point::new(0.5, -1.0, 1.5);
    let actual = a / 2.0;

    assert_eq!(expected, actual);
}
