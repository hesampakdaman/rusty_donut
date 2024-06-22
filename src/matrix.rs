use std::fmt;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul};

#[derive(Debug, PartialEq)]
pub struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: Default> Matrix<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        let dim = rows * cols;
        let mut data = Vec::with_capacity(dim);
        for _ in 0..dim {
            data.push(T::default())
        }
        Matrix { data, rows, cols }
    }
}

impl<T> TryFrom<Vec<Vec<T>>> for Matrix<T> {
    type Error = String;

    fn try_from(value: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        let rows = value.len();
        if rows == 0 {
            return Err("Nested vec is empty".to_string());
        }
        let cols = value[0].len();
        if cols == 0 {
            return Err("Nested vec contains an empty row".to_string());
        }
        if !value.iter().all(|v| cols == v.len()) {
            return Err("Nested vec is not a matrix".to_string());
        }
        Ok(Matrix {
            data: value.into_iter().flatten().collect(),
            rows,
            cols,
        })
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.data[i * self.cols + j]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.data[i * self.cols + j]
    }
}

impl<T: Add<Output = T> + Copy + Default> Add for Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, rhs: Self) -> Self::Output {
        assert!(self.rows == rhs.rows && self.cols == rhs.cols);

        let mut res = Matrix::<T>::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                res[(i, j)] = self[(i, j)] + rhs[(i, j)];
            }
        }
        res
    }
}

impl<T: AddAssign + Copy + Default + Mul<Output = T>> Mul for Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        assert!(self.cols == rhs.rows);

        let mut res = Matrix::<T>::new(self.rows, rhs.cols);
        for k in 0..self.cols {
            for i in 0..self.rows {
                for j in 0..rhs.cols {
                    res[(i, j)] += self[(i, k)] * rhs[(k, j)];
                }
            }
        }
        res
    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{}\t", self[(i, j)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! matrix {
        ($([$($elem:expr), *]), *) => {{
            let v = vec![$(vec![$($elem), *], ) *];
            Matrix::try_from(v).expect("Invalid matrix")
        }};
    }

    #[test]
    fn addition() {
        let m1 = matrix![[1, 2], [3, 4]];
        let m2 = matrix![[5, 6], [7, 8]];

        assert_eq!(m1 + m2, matrix![[6, 8], [10, 12]])
    }


    #[test]
    fn matmul() {
        let m1 = matrix![[1, 2], [3, 4]];
        let m2 = matrix![[5, 6, 7], [8, 9, 10]];

        assert_eq!(m1 * m2, matrix![[21, 24, 27], [47, 54, 61]])
    }
}
