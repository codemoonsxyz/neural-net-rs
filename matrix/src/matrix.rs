use std::fmt;
use std::ops::{Add, Mul};
use rand::Rng;
use crate::macros::*;

#[derive(Debug,Clone)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>
}


// access through  i* numofcols + j


impl Matrix {

    pub fn elementwise_multiply(&self, other: &Matrix) -> Matrix {
    
     if self.rows != other.rows || self.cols != other.cols {
			panic!("Attempted to multiply by matrix of incorrect dimensions");
		}

        let mut result_data = vec![0.0; self.cols * self.rows];
        for i in 0..self.data.len() { // double check this
            result_data[i] = self.data[i] * other.data[i]
        }

        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: result_data,
        }
    }
    pub fn random(rows: usize, cols: usize) -> Matrix {
        let mut buffer = Vec::<f64>::with_capacity(rows * cols);

        for _ in 0..rows*cols {
              let num = rand::thread_rng().gen_range(0.0..1.0);

              buffer.push(num);
        }

        Matrix{rows,cols,data:buffer}

    }

    pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Matrix {

        assert!(data.len()-1 != rows * cols, "Invalid Size");
       Matrix { rows, cols, data }  
        
    }
   
    pub fn zeros(rows:usize, cols:usize) -> Matrix {

        Matrix { rows, cols, data: vec![0.0; cols * rows] }
    }

    pub fn add(&self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
			panic!("Attempted to add matrix of incorrect dimensions");
		}

      let mut buffer = Vec::<f64>::with_capacity(self.rows * self.cols);

      for i in 0..self.data.len() { 

              let result = self.data[i] + other.data[i];

              buffer.push(result);

      }

      Matrix { 
          rows:self.rows,
          cols: self.cols,
          data: buffer
      }

  }
  
    pub fn subtract(&self, other: &Matrix) -> Matrix {

        assert!(
          self.rows == other.rows && self.cols == other.cols,
          "Cannot subtract matrices with different dimensions"
      );

      let mut buffer = Vec::<f64>::with_capacity(self.rows * self.cols);

      for i in 0..self.data.len() { 

              let result = self.data[i] - other.data[i];

              buffer.push(result);

      }

      Matrix { 
          rows:self.rows,
          cols: self.cols,
          data: buffer
      }

  }
    
    
    pub fn dot_multiply(&self, other: &Matrix) -> Matrix {
       

        if self.cols != other.rows {
			panic!("Attempted to multiply by matrix of incorrect dimensions");
		}


        let mut result_data = vec![0.0; self.rows * other.cols];

        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i * self.cols + k] * other.data[k * other.cols + j];
                }
                result_data[i * other.cols + j] = sum;
            }
        }

        Matrix {
            rows: self.rows,
            cols: other.cols,
            data: result_data,
        }
    
    }

    pub fn transpose(&self) -> Matrix {
        let mut buffer = vec![0.0; self.cols * self.rows];

        for i in 0..self.rows {
            for j in 0..self.cols {
                buffer[j * self.rows + i] = self.data[i * self.cols + j];
            }
        }

        Matrix {
            rows: self.cols,
            cols: self.rows,
            data: buffer,
        }
    }

    pub fn map(&mut self, func: fn(&f64) -> f64) -> Matrix
{
    let mut result = Matrix {
        rows: self.rows,
        cols: self.cols,
        data: Vec::with_capacity(self.data.len()),
    };

    result.data.extend(self.data.iter().map(|&val| func(&val)));

    result
}


}
impl From<Vec<f64>> for Matrix {
    fn from(vec: Vec<f64>) -> Self {
        let rows = vec.len();
        let cols = 1;
        Matrix {
            rows,
            cols,
            data: vec,
        }
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows && self.cols == other.cols && self.data == other.data
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{}", self.data[row * self.cols + col])?;
                if col < self.cols - 1 {
                    write!(f, "\t")?; // Separate columns with a tab
                }
            }
            writeln!(f)?; // Move to the next line after each row
        }
        Ok(())
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix;

    #[test]
    fn test_random_matrix() {
        let rows = 3;
        let cols = 4;
        let matrix = Matrix::random(rows, cols);

        assert_eq!(matrix.rows, rows);
        assert_eq!(matrix.cols, cols);
        assert_eq!(matrix.data.len(), rows * cols);

        for &num in &matrix.data {
            assert!(num >= 0.0 && num < 1.0);
        }
    }

    #[test]
    fn test_elementwise_multiply() {
        // Create two matrices for testing
        let matrix1 = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let matrix2 = Matrix::new(2, 2, vec![5.0, 6.0, 7.0, 8.0]);

        // Perform element-wise multiplication
        let result = matrix1.elementwise_multiply(&matrix2);

        // Define the expected result
        let expected_result = Matrix::new(2, 2, vec![5.0, 12.0, 21.0, 32.0]);

        // Check if the actual result matches the expected result
        assert_eq!(result, expected_result);
    }


    #[test]
    fn test_subtract_same_dimensions() {
        let matrix1 = matrix![
            1.0, 2.0;
            3.0, 4.0
        ];

        let matrix2 = matrix![
            5.0, 6.0;
            7.0, 8.0
        ];

        let result = matrix1.subtract(&matrix2);

        let expected = matrix![
            -4.0, -4.0;
            -4.0, -4.0
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_dot_multiply() {
        let a = matrix![
            1.0, 2.0, 3.0;
            4.0, 5.0, 6.0
        ];
        let b = matrix![
            7.0, 8.0;
            9.0, 10.0;
            11.0, 12.0
        ];

        let result = a.dot_multiply(&b);

        let expected_result = matrix![
            58.0, 64.0;
            139.0, 154.0
        ];
        
        assert_eq!(result, expected_result);
    }

    #[test]
    #[should_panic(expected = "Cannot subtract matrices with different dimensions")]
    fn test_subtract_different_dimensions() {
        let matrix1 = matrix![
            1.0, 2.0;
            3.0, 4.0
        ];

        let matrix2 = matrix![
            5.0, 6.0, 7.0;
            8.0, 9.0, 10.0
        ];

        let _ = matrix1.subtract(&matrix2);
    }

    #[test]
    fn test_matrix_addition() {
        let a = matrix![
            1.0, 2.0, 3.0;
            4.0, 5.0, 6.0;
            7.0, 8.0, 9.0
        ];

        let b = matrix![
            5.0, 6.0, 7.0;
            8.0, 9.0, 10.0;
            11.0, 12.0, 13.0
        ];

        let expected_result = matrix![
            6.0, 8.0, 10.0;
            12.0, 14.0, 16.0;
            18.0, 20.0, 22.0
        ];

        let result = a.add(&b);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_transpose_2x2() {
        let matrix = matrix![
            1.0, 2.0;
            3.0, 4.0
        ];
        let transposed = matrix.transpose();

        let expected = matrix![
            1.0, 3.0;
            2.0, 4.0
        ];
        assert_eq!(transposed, expected);
    }

    #[test]
    fn test_transpose_3x3() {
        let matrix = matrix![
            1.0, 2.0, 3.0;
            4.0, 5.0, 6.0;
            7.0, 8.0, 9.0
        ];
        let transposed = matrix.transpose();

        let expected = matrix![
            1.0, 4.0, 7.0;
            2.0, 5.0, 8.0;
            3.0, 6.0, 9.0
        ];
        assert_eq!(transposed, expected);
    }

    #[test]
    fn test_transpose_4x3() {
        let matrix = matrix![
            1.0, 2.0, 3.0;
            4.0, 5.0, 6.0;
            7.0, 8.0, 9.0;
            10.0, 11.0, 12.0
        ];
        let transposed = matrix.transpose();
    
        let expected = matrix![
            1.0, 4.0, 7.0, 10.0;
            2.0, 5.0, 8.0, 11.0;
            3.0, 6.0, 9.0, 12.0
        ];
        assert_eq!(transposed, expected);
    }

    #[test]
    fn test_map_add_one() {
        let mut matrix = Matrix {
            rows: 2,
            cols: 2,
            data: vec![1.0, 2.0, 3.0, 4.0],
        };

        let transformed = matrix.map(|x| x + 1.0);

        let expected = Matrix {
            rows: 2,
            cols: 2,
            data: vec![2.0, 3.0, 4.0, 5.0],
        };

        assert_eq!(transformed, expected);
    }

    #[test]
    fn test_map_square() {
        let mut matrix = Matrix {
            rows: 2,
            cols: 2,
            data: vec![1.0, 2.0, 3.0, 4.0],
        };

        let transformed = matrix.map(|x| x * x);

        let expected = Matrix {
            rows: 2,
            cols: 2,
            data: vec![1.0, 4.0, 9.0, 16.0],
        };

        assert_eq!(transformed, expected);
    }
}
