use crate::equal;
use std::fmt;
use std::ops;

use crate::point::Point;
use crate::vector::Vector;

#[derive(Copy, Clone)]
struct Matrix<const D: usize> {
    entries: [[f64; D]; D],
}

impl<const D: usize> From<[[f64; D]; D]> for Matrix<D> {
    fn from(entries: [[f64; D]; D]) -> Self {
        Matrix { entries }
    }
}

impl<const D: usize> Matrix<D> {
    fn new() -> Matrix<D> {
        Matrix::from([[0.0; D]; D])
    }

    fn transpose(&self) -> Self {
        let mut entries = [[0.0; D]; D];
        for row in 0..D {
            for col in 0..D {
                entries[col][row] = self.entries[row][col]
            }
        }
        Self { entries }
    }
}

impl Matrix<2> {
    fn determinant(&self) -> f64 {
        self[0][0] * self[1][1] - self[1][0] * self[0][1]
    }
    fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }
}

impl Matrix<3> {
    fn determinant(&self) -> f64 {
        let mut determinant = 0.0;
        for (index, element) in self[0].iter().enumerate() {
            determinant += element * self.cofactor(0, index);
        }
        determinant
    }

    fn submatrix(&self, row: usize, col: usize) -> Matrix<2> {
        let mut submatrix = Matrix::new();
        let mut i = 0;
        let mut j = 0;

        //TODO: Can I write self.iter() here?
        for (index, element) in self.entries.iter().enumerate() {
            if index == row {
                continue;
            }
            for (index, entry) in element.iter().enumerate() {
                if index == col {
                    continue;
                }

                submatrix[i][j] = *entry;

                j = match j {
                    1 => 0,
                    _ => 1,
                };
            }

            i = match i {
                1 => 0,
                _ => 1,
            };
        }
        submatrix
    }

    fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if (row + col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }
}

impl Matrix<4> {
    fn determinant(&self) -> f64 {
        let mut determinant = 0.0;
        for (index, element) in self[0].iter().enumerate() {
            determinant += element * self.cofactor(0, index);
        }
        determinant
    }

    fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if (row + col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    fn submatrix(&self, row: usize, col: usize) -> Matrix<3> {
        let mut submatrix = Matrix::new();
        let mut i = 0;
        let mut j = 0;

        for (index, element) in self.entries.iter().enumerate() {
            if index == row {
                continue;
            }
            for (index, entry) in element.iter().enumerate() {
                if index == col {
                    continue;
                }

                submatrix[i][j] = *entry;

                j = match j {
                    2 => 0,
                    _ => j + 1,
                };
            }

            i = match i {
                2 => 0,
                _ => i + 1,
            };
        }
        submatrix
    }

    fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    fn inverse(&self) -> Matrix<4> {
        assert!(self.is_invertible());

        let mut matrix = Matrix::new();
        let determinant = self.determinant();

        for row in 0..4 {
            for col in 0..4 {
                let cofactor = self.cofactor(row, col);
                matrix[col][row] = cofactor / determinant;
            }
        }
        matrix
    }

    fn translation(x: f64, y: f64, z: f64) -> Matrix<4> {
        Matrix::from([
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    fn scaling(x: f64, y: f64, z: f64) -> Matrix<4> {
        Matrix::from([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    fn rotation_x(r: f64) -> Matrix<4> {
        Matrix::from([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, r.cos(), -r.sin(), 0.0],
            [0.0, r.sin(), r.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    fn rotation_y(r: f64) -> Matrix<4> {
        Matrix::from([
            [r.cos(), 0.0, r.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-r.sin(), 0.0, r.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    fn rotation_z(r: f64) -> Matrix<4> {
        Matrix::from([
            [r.cos(), -r.sin(), 0.0, 0.0],
            [r.sin(), r.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    fn shearing(x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Matrix<4> {
        Matrix::from([
            [1.0, x_y, x_z, 0.0],
            [y_x, 1.0, y_z, 0.0],
            [z_x, z_y, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

impl<const D: usize> fmt::Debug for Matrix<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = 10;
        let precision = 5;
        writeln!(f, "");

        for row in 0..D {
            write!(f, "| {0:>width$}", format!("{0:.precision$}", self[row][0]))?;
            writeln!(f, "{0:>width$}", format!("{0:.precision$} |", self[row][1]))?;
        }

        Ok(())
    }
}

impl<const D: usize> ops::Index<usize> for Matrix<D> {
    type Output = [f64; D];

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl<const D: usize> ops::IndexMut<usize> for Matrix<D> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

impl<const D: usize> PartialEq<Self> for Matrix<D> {
    fn eq(&self, other: &Matrix<D>) -> bool {
        for row in 0..D {
            for col in 0..D {
                if equal(self[row][col], other[row][col]) == false {
                    return false;
                }
            }
        }
        true
    }

    fn ne(&self, other: &Matrix<D>) -> bool {
        !self.eq(other)
    }
}

impl<const D: usize> ops::Mul<Matrix<D>> for Matrix<D> {
    type Output = Self;

    fn mul(self, other: Matrix<D>) -> Self::Output {
        let mut matrix = Matrix::new();
        for row in 0..D {
            for col in 0..D {
                for i in 0..D {
                    matrix[row][col] += self[row][i] * other[i][col];
                }
            }
        }

        matrix
    }
}

impl ops::Mul<Point> for Matrix<4> {
    type Output = Point;

    fn mul(self, other: Point) -> Self::Output {
        let mut result = [0.0; 3];
        for row in 0..3 {
            result[row] = self[row][0] * other.x
                + self[row][1] * other.y
                + self[row][2] * other.z
                + self[row][3]
        }

        Point::from(result)
    }
}

impl ops::Mul<Vector> for Matrix<4> {
    type Output = Vector;

    fn mul(self, other: Vector) -> Self::Output {
        let mut result = [0.0; 3];
        for row in 0..3 {
            result[row] = self[row][0] * other.x + self[row][1] * other.y + self[row][2] * other.z
        }

        Vector::from(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn should_construct_and_inspect_a_4_by_4_matrix() {
        let m = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[0][3], 4.0);
        assert_eq!(m[1][0], 5.5);
        assert_eq!(m[1][2], 7.5);
        assert_eq!(m[2][2], 11.0);
        assert_eq!(m[3][0], 13.5);
        assert_eq!(m[3][2], 15.5);
    }

    #[test]
    fn should_compare_identical_matrices() {
        let a = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let b = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        assert_eq!(a, b);
    }

    #[test]
    fn should_compare_different_matrices() {
        let a = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let b = Matrix::from([
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);

        assert_ne!(a, b);
    }

    #[test]
    fn should_multiply_two_matrices() {
        let a = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let b = Matrix::from([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);

        let expected = Matrix::from([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);

        let actual = a * b;

        assert_eq!(expected, actual);
    }

    #[test]
    fn should_multiply_a_matrix_with_a_point() {
        let a = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let b = Point::new(1.0, 2.0, 3.0);
        let expected = Point::new(18.0, 24.0, 33.0);
        let actual = a * b;

        assert_eq!(expected, actual);
    }

    #[test]
    fn should_multiply_matrix_by_the_identity_matrix() {
        let a = Matrix::from([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);

        let identity_matrix = Matrix::from([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let actual = a * identity_matrix;
        assert_eq!(a, actual);
    }

    #[test]
    fn should_transpose_a_matrix() {
        let a = Matrix::from([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);

        let expected = Matrix::from([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]);

        assert_eq!(expected, a.transpose());
    }

    #[test]
    fn should_transpose_identity_matrix() {
        let identity_matrix = Matrix::from([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        assert_eq!(identity_matrix, identity_matrix.transpose());
    }

    #[test]
    fn should_calculate_the_determinant() {
        let a = Matrix::from([[1.0, 5.0], [-3.0, 2.0]]);
        let expected = 17.0;
        let actual = a.determinant();

        assert_eq!(expected, actual);
    }

    #[test]
    fn should_return_a_2_x_2_submatrix() {
        let a = Matrix::from([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);
        let expected = Matrix::from([[-3.0, 2.0], [0.0, 6.0]]);
        assert_eq!(expected, a.submatrix(0, 2));
    }

    #[test]
    fn should_return_a_3_x_3_submatrix() {
        let a = Matrix::from([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);

        let expected = Matrix::from([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]]);
        let actual = a.submatrix(2, 1);
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_return_minor_of_3_by_3_matrix() {
        let a = Matrix::from([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        assert_eq!(a.minor(1, 0), 25.0);
    }

    #[test]
    fn should_return_cofactor_of_3_by_3_matrix() {
        let a = Matrix::from([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        assert_eq!(a.cofactor(0, 0), -12.0);
        assert_eq!(a.cofactor(1, 0), -25.0);
    }

    #[test]
    fn should_calculate_the_determinant_of_a_3_by_3_matrix() {
        let a = Matrix::from([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);
        assert_eq!(a.determinant(), -196.0);
    }

    #[test]
    fn should_calculate_the_determinant_of_a_4_by_4_matrix() {
        let a = Matrix::from([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);
        assert_eq!(a.determinant(), -4071.0);
    }

    #[test]
    fn should_determine_if_a_matrix_is_invertible() {
        let a = Matrix::from([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);
        assert!(a.is_invertible());

        let b = Matrix::from([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);
        assert!(!b.is_invertible());
    }

    #[test]
    fn should_calculate_the_inverse_of_a_matrix() {
        let a = Matrix::from([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);

        let expected = Matrix::from([
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        ]);

        let b = a.inverse();

        assert_eq!(a.determinant(), 532.0);
        assert_eq!(a.cofactor(2, 3), -160.0);
        assert_eq!(a.cofactor(3, 2), 105.0);
        assert_eq!(-160.0 / 532.0, b[3][2]);
        assert_eq!(105.0 / 532.0, b[2][3]);
        assert_eq!(expected, b);
    }

    #[test]
    fn should_calculate_the_inverse_of_another_matrix() {
        let a = Matrix::from([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);

        let expected = Matrix::from([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]);

        assert_eq!(expected, a.inverse());
    }

    #[test]
    fn should_calculate_the_inverse_of_a_third_matrix() {
        let a = Matrix::from([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]);

        let expected = Matrix::from([
            [-0.04074, -0.07778, 0.14444, -0.22222],
            [-0.07778, 0.03333, 0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926, 0.12963],
            [0.17778, 0.06667, -0.26667, 0.33333],
        ]);

        assert_eq!(expected, a.inverse());
    }

    #[test]
    fn should_multiply_a_product_by_its_inverse() {
        let a = Matrix::from([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);

        let b = Matrix::from([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);

        let c = a * b;

        assert_eq!(c * b.inverse(), a);
    }

    #[test]
    fn should_multiply_by_a_translation_matrix() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let p = Point::new(-3.0, 4.0, 5.0);
        let expected = Point::new(2.0, 1.0, 7.0);
        assert_eq!(transform * p, expected);
    }

    #[test]
    fn should_multiply_by_the_inverse_of_a_translation_matrix() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let inv = transform.inverse();
        let p = Point::new(-3.0, 4.0, 5.0);
        let expected = Point::new(-8.0, 7.0, 3.0);

        assert_eq!(inv * p, expected)
    }

    #[test]
    fn should_scale_a_matrix_applied_to_a_point() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let p = Point::new(-4.0, 6.0, 8.0);
        assert_eq!(transform * p, Point::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn should_scale_a_matrix_applied_to_a_vector() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let v = Vector::from([-4.0, 6.0, 8.0]);
        assert_eq!(transform * v, Vector::from([-8.0, 18.0, 32.0]));
    }

    #[test]
    fn should_multiply_by_the_inverse_of_a_scaling_matrix() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse();
        let v = Vector::from([-4.0, 6.0, 8.0]);
        assert_eq!(inv * v, Vector::from([-2.0, 2.0, 2.0]));
    }

    #[test]
    fn should_reflect_by_scaling_with_a_negative_value() {
        let transform = Matrix::scaling(-1.0, 1.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Point::new(-2.0, 3.0, 4.0));
    }

    #[test]
    fn should_rotate_a_point_around_the_x_axis() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_x(PI / 4.0);
        let full_quarter = Matrix::rotation_x(PI / 2.0);
        assert_eq!(
            half_quarter * p,
            Point::new(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        );
        assert_eq!(full_quarter * p, Point::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn should_rotate_a_point_in_the_x_axis_in_the_inverse_direction() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_x(PI / 4.0);
        let inv = half_quarter.inverse();
        assert_eq!(
            inv * p,
            Point::new(0.0, 2.0_f64.sqrt() / 2.0, -(2.0_f64.sqrt() / 2.0))
        );
    }

    #[test]
    fn should_rotate_a_point_around_the_y_axis() {
        let p = Point::new(0.0, 0.0, 1.0);
        let half_quarter = Matrix::rotation_y(PI / 4.0);
        let full_quarter = Matrix::rotation_y(PI / 2.0);

        assert_eq!(
            half_quarter * p,
            Point::new(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0)
        );
        assert_eq!(full_quarter * p, Point::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn should_rotate_a_point_around_the_z_axis() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_z(PI / 4.0);
        let full_quarter = Matrix::rotation_z(PI / 2.0);

        assert_eq!(
            half_quarter * p,
            Point::new(-(2.0_f64.sqrt() / 2.0), 2.0_f64.sqrt() / 2.0, 0.0)
        );
        assert_eq!(full_quarter * p, Point::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn should_move_x_in_proportion_to_y() {
        let transform = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Point::new(5.0, 3.0, 4.0));
    }

    #[test]
    fn should_move_x_in_proportion_to_z() {
        let transform = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Point::new(6.0, 3.0, 4.0));
    }

    #[test]
    fn should_move_y_in_proportion_to_x() {
        let transform = Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Point::new(2.0, 5.0, 4.0));
    }

    #[test]
    fn should_move_y_in_proportion_to_z() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Point::new(2.0, 7.0, 4.0));
    }

    #[test]
    fn should_move_z_in_proportion_to_x() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Point::new(2.0, 3.0, 6.0));
    }

    #[test]
    fn should_move_z_in_proportion_to_y() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Point::new(2.0, 3.0, 7.0));
    }

    #[test]
    fn should_apply_transformations_in_sequence() {
        let p = Point::new(1.0, 0.0, 1.0);
        let a = Matrix::rotation_x(PI / 2.0);
        let b = Matrix::scaling(5.0, 5.0, 5.0);
        let c = Matrix::translation(10.0, 5.0, 7.0);

        let p2 = a * p;
        assert_eq!(p2, Point::new(1.0, -1.0, 0.0));

        let p3 = b * p2;
        assert_eq!(p3, Point::new(5.0, -5.0, 0.0));

        let p4 = c * p3;
        assert_eq!(p4, Point::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn should_transform_in_reversed_order() {
        let p = Point::new(1.0, 0.0, 1.0);

        let a = Matrix::rotation_x(PI / 2.0);
        let b = Matrix::scaling(5.0, 5.0, 5.0);
        let c = Matrix::translation(10.0, 5.0, 7.0);

        let t = c * b * a;
        assert_eq!(t * p, Point::new(15.0, 0.0, 7.0));
    }
}
