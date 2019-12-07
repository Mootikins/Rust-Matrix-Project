extern crate crossbeam;

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};
use std::ops::{Index, IndexMut, Mul};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<i32>,
}

impl Matrix {
    /// Returns a matrix with the given size and elements
    ///
    /// # Arguments
    ///
    /// * `cols` - The number of columns of the matrix
    /// * `rows` - The number of rows of the matrix
    ///
    /// ```
    /// use matrix::Matrix;
    /// let matrix = Matrix::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// ```
    ///
    /// Author: Matthew Krohn
    pub fn new(cols: usize, rows: usize, data: Vec<i32>) -> Matrix {
        assert_eq!(cols * rows, data.len());
        Matrix { cols, rows, data }
    }

    /// Gets the numbr of columns in this Matrix
    ///
    /// # Arguments
    /// * self - reference to this Matrix
    ///
    /// # Returns
    /// the number of columns in this Matrix
    ///
    /// Author: Matthew Krohn
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Gets the number of Rows of the Matrix
    ///
    /// # Arguments
    /// * self - reference to this Matrix
    ///
    /// # Returns
    /// the number of rows in this Matrix
    ///
    /// Author: Matthew Krohn
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns an iterator of references to the items of the given row
    ///
    /// # Arguments
    ///
    /// * `row_num` - The row number to get an iterator for; is 0-indexed
    ///
    /// ```
    /// use matrix::Matrix;
    /// let matrix = Matrix::new(1, 4, vec![1, 2, 3, 4];
    /// assert_eq!(matrix.row_iter(0).collect(), vec![&1, &2, &3, &4]);
    /// ```
    ///
    /// Author: Matthew Krohn
    fn row_iter<'a>(&'a self, row_num: usize) -> impl Iterator<Item = &i32> + 'a {
        assert!(row_num < self.rows, "Row index out of bounds");
        self.data.iter().skip(self.cols * row_num).take(self.cols)
    }

    /// Returns an iterator of references to the items of the given column
    ///
    /// # Arguments
    ///
    /// * `col_num` - The column number to get an iterator for; is 0-indexed
    ///
    /// ```
    /// use matrix::Matrix;
    /// let matrix = Matrix::new(4, 1, vec![1, 2, 3, 4];
    /// assert_eq!(matrix.col_iter(0).collect(), vec![&1, &2, &3, &4]);
    /// ```
    ///
    /// Author: Matthew Krohn
    fn col_iter<'a>(&'a self, col_num: usize) -> impl Iterator<Item = &i32> + 'a {
        assert!(col_num < self.cols, "Column index out of bounds");
        self.data.iter().skip(col_num).step_by(self.cols)
    }

    /// Returns a new matrix that is the result of two compatible matrices being
    /// multiplied
    ///
    /// # Arguments
    ///
    /// * `self` - The "left" matrix in the multiplication
    /// * `rhs` - The "right" matrix in the multiplication
    ///
    /// ```
    /// let our_mat1 = Matrix::new(3, 2, vec![1, 2, 3, 4, 5, 6]);
    /// let our_mat2 = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]);
    /// let result_mat = Matrix::new(2, 2, vec![22, 28, 49, 64]);
    ///
    /// let new_mat = our_mat1.mul_mat(&our_mat2);
    /// assert_eq!(new_mat, result_mat);
    /// ```
    /// Author: Matthew Krohn
    pub fn mul_mat(&self, rhs: &Matrix) -> Matrix {
        assert_eq!(self.cols, rhs.rows);
        let mut matr_data = vec![0; self.rows * rhs.cols];

        let mut parts: Vec<&mut [i32]> = matr_data.chunks_mut(rhs.cols).collect();

        // Concurrent matrix multiply
        crossbeam::scope(|spawner| {
            for (row_num, part) in &mut parts.iter_mut().enumerate() {
                spawner.spawn(move |_| {
                    for (col_num, cell) in &mut part.iter_mut().enumerate() {
                        *cell = self
                            .row_iter(row_num)
                            .zip(rhs.col_iter(col_num))
                            .fold(0, |sum, (lhs_num, rhs_num)| sum + lhs_num * rhs_num);
                    }
                });
            }
        })
        .unwrap();

        Matrix {
            cols: rhs.cols,
            rows: self.rows,
            data: matr_data,
        }
    }

    /// Adds two matrices with the same dimensions
    ///
    /// # Arguments
    ///
    /// * `self` - The "left" matrix in the addition
    /// * `rhs` - The "right" matrix in the addition
    ///
    /// ```
    /// let our_mat1 = Matrix::new(3, 2, vec![1, 2, 3, 4, 5, 6]);
    /// let our_mat2 = Matrix::new(3, 2, vec![1, 2, 3, 4, 5, 6]);
    /// let result_mat = Matrix::new(3, 2, vec![2, 4, 6, 8, 10, 12]);
    ///
    /// let new_mat = our_mat1.add_mat(&our_mat2);
    /// assert_eq!(new_mat, result_mat);
    /// ```
    ///
    /// Author: Kendric Thompson
    pub fn add_mat(&self, rhs: &Matrix) -> Matrix {
        assert_eq!(self.cols, rhs.cols);
        assert_eq!(self.rows, rhs.rows);

        let matr_data = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(num1, num2)| num1 + num2)
            .collect();

        Matrix {
            cols: self.cols,
            rows: self.rows,
            data: matr_data,
        }
    }

    /// Subtracts two matrices with the same dimensions
    ///
    /// # Arguments
    ///
    /// * `self` - The "left" matrix in the subtraction
    /// * `rhs` - The "right" matrix in the subtraction
    ///
    /// ```
    /// let our_mat1 = Matrix::new(3, 2, vec![1, 2, 3, 4, 5, 6]);
    /// let our_mat2 = Matrix::new(3, 2, vec![1, 2, 3, 4, 5, 6]);
    /// let result_mat = Matrix::new(3, 2, vec![0, 0, 0, 0, 0, 0]);
    ///
    /// let new_mat = our_mat1.add_mat(&our_mat2);
    /// assert_eq!(new_mat, result_mat);
    /// ```
    ///
    /// Author: Kendric Thompson
    pub fn sub_mat(&self, rhs: &Matrix) -> Matrix {
        assert_eq!(self.cols, rhs.cols);
        assert_eq!(self.rows, rhs.rows);

        let matr_data = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(num1, num2)| num1 - num2)
            .collect();

        Matrix {
            cols: self.cols,
            rows: self.rows,
            data: matr_data,
        }
    }
}

// In Rust, traits are not normally documented since they are used for
// interoperability between crates and operands
impl Index<[usize; 2]> for Matrix {
    type Output = i32;

    /// Indexes into the Matrix
    ///
    /// # Arguments
    /// self - reference to this Matrix
    /// index - spot to get
    ///
    /// # Return
    /// Returns the value at index
    ///
    /// Author: Matthew Krohn
    fn index(&self, index: [usize; 2]) -> &i32 {
        assert!(
            index[0] < self.rows,
            "Row index is greater than row dimension."
        );
        assert!(
            index[1] < self.cols,
            "Column index is greater than column dimension."
        );
        &self.data[index[0] * self.cols + index[1]]
    }
}

impl IndexMut<[usize; 2]> for Matrix {
    /// Indexes into the Matrix - mutable
    ///
    /// # Arguments
    /// self - reference to this Matrix
    /// index - index of spot to get
    ///
    /// # Return
    /// Returns a mutable reference to the value at index
    ///
    /// Author: Matthew Krohn
    fn index_mut(&mut self, index: [usize; 2]) -> &mut i32 {
        assert!(
            index[0] < self.rows,
            "Row index is greater than row dimension."
        );
        assert!(
            index[1] < self.cols,
            "Column index is greater than column dimension."
        );
        &mut self.data[index[0] * self.cols + index[1]]
    }
}

impl Display for Matrix {
    /// Formats the matrix for display
    ///
    /// # Arguments
    /// * self - reference to this Matrix
    /// * f - reference to the formatter to write to
    /// # Return
    /// Returns success of write
    ///
    /// Author: Matthew Krohn
    fn fmt(&self, f: &mut Formatter) -> Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{: >6} ", self[[row, col]])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Mul<i32> for Matrix {
    type Output = Self;

    /// Multiplies the Matrix elements by the scalar
    ///
    /// # Arguments
    ///
    /// * `self` - The matrix
    /// * `rhs` - The scalar to multiply by
    ///
    /// Author: Jennifer Kulich
    fn mul(self, rhs: i32) -> Self {
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self.data.iter().map(|num| *num * rhs).collect(),
        }
    }
}

impl Mul<Matrix> for i32 {
    type Output = Matrix;

    /// Multiplies the Matrix elements by the scalar
    ///
    /// # Arguments
    ///
    /// * `self` - The scalar to multiply by
    /// * `rhs` - The matrix
    ///
    /// Author: Jennifer Kulich
    fn mul(self, rhs: Matrix) -> Matrix {
        rhs * self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test matrix multiple
    /// Author: Jennifer Kulich
    #[test]
    fn test_mul_mat_i32() {
        let our_mat = Matrix {
            rows: 3,
            cols: 3,
            data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        };
        let ref_mat = Matrix {
            rows: 3,
            cols: 3,
            data: vec![3, 6, 9, 12, 15, 18, 21, 24, 27],
        };

        let new_mat_1 = our_mat.clone() * 3; // need clone since Mul taKendrs by value
        let new_mat_2 = 3 * our_mat;
        assert_eq!(ref_mat, new_mat_1);
        assert_eq!(ref_mat, new_mat_2);
    }

    /// Test matrix row and column iterators
    /// Author: Jennifer Kulich
    #[test]
    fn test_row_and_col_iter_small() {
        let our_mat = Matrix {
            cols: 3,
            rows: 2,
            data: vec![1, 2, 3, 4, 5, 6],
        };

        let mut rows: Vec<Vec<&i32>> = Vec::new();
        let mut cols: Vec<Vec<&i32>> = Vec::new();
        for row in 0..our_mat.rows {
            rows.push(our_mat.row_iter(row).collect());
        }
        for col in 0..our_mat.cols {
            cols.push(our_mat.col_iter(col).collect());
        }

        assert_eq!(rows[0], vec!(&1, &2, &3));
        assert_eq!(rows[1], vec!(&4, &5, &6));

        assert_eq!(cols[0], vec!(&1, &4));
        assert_eq!(cols[1], vec!(&2, &5));
        assert_eq!(cols[2], vec!(&3, &6));
    }

    /// Test Matrix index
    /// Author: Jennifer Kulich
    #[test]
    fn test_index() {
        let our_mat = Matrix {
            cols: 3,
            rows: 3,
            data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        };
        assert_eq!(our_mat[[0, 0]], 1);
        assert_eq!(our_mat[[0, 1]], 2);
        assert_eq!(our_mat[[0, 2]], 3);
        assert_eq!(our_mat[[1, 0]], 4);
        assert_eq!(our_mat[[1, 1]], 5);
        assert_eq!(our_mat[[1, 2]], 6);
        assert_eq!(our_mat[[2, 0]], 7);
        assert_eq!(our_mat[[2, 1]], 8);
        assert_eq!(our_mat[[2, 2]], 9);
    }

    /// Test mutation index
    /// Author: Jennifer Kulich
    #[test]
    fn test_index_mut() {
        let mut our_mat = Matrix {
            cols: 1,
            rows: 1,
            data: vec![1],
        };
        our_mat[[0, 0]] = 2;
        assert_eq!(our_mat[[0, 0]], 2);
    }

    /// Test Matrix Multiply
    /// Author: Jennifer Kulich
    #[test]
    fn test_matrix_multiply() {
        let our_mat1 = Matrix {
            cols: 3,
            rows: 2,
            data: vec![1, 2, 3, 4, 5, 6],
        };
        let our_mat2 = Matrix {
            cols: 2,
            rows: 3,
            data: vec![1, 2, 3, 4, 5, 6],
        };
        let result_mat = Matrix {
            cols: 2,
            rows: 2,
            data: vec![22, 28, 49, 64],
        };
        let new_mat = our_mat1.mul_mat(&our_mat2);
        assert_eq!(new_mat, result_mat);
    }

    // Author: Kendric Thompson
    // This tests the adding of two matrices
    #[test]
    fn test_matrix_addition() {
        let our_mat1 = Matrix {
            cols: 3,
            rows: 2,
            data: vec![1, 2, 3, 4, 5, 6],
        };
        let our_mat2 = Matrix {
            cols: 3,
            rows: 2,
            data: vec![1, 2, 3, 4, 5, 6],
        };
        let result_mat = Matrix {
            cols: 3,
            rows: 2,
            data: vec![2, 4, 6, 8, 10, 12],
        };
        let new_mat = our_mat1.add_mat(&our_mat2);
        assert_eq!(new_mat, result_mat);
    }

    // Author: Kendric Thompson
    // This tests the subbtracting of two matrices
    #[test]
    fn test_matrix_subtraction() {
        let our_mat1 = Matrix {
            cols: 3,
            rows: 2,
            data: vec![1, 2, 3, 4, 5, 6],
        };
        let our_mat2 = Matrix {
            cols: 3,
            rows: 2,
            data: vec![1, 2, 3, 4, 5, 6],
        };
        let result_mat = Matrix {
            cols: 3,
            rows: 2,
            data: vec![0, 0, 0, 0, 0, 0],
        };
        let new_mat = our_mat1.sub_mat(&our_mat2);
        assert_eq!(new_mat, result_mat);
    }
}
