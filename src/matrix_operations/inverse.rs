use crate::matrix::MatrixMN;


impl MatrixMN {
    pub fn is_invertible(&self) -> bool {
        let m: usize = self.nr_lines();
        let n: usize = self.nr_columns();

        if m * n == 0 || m != n {
            return false;
        }

        if self.det() == 0.0 {
            return false;
        }

        if self.det().abs() < f64::EPSILON {
            return false;
        }

        return true;
    }
}

impl MatrixMN {
    pub fn inverse(&self) -> Self {
        let m: usize = self.nr_lines();
        let n: usize = self.nr_columns();
        let det: f64 = self.det();

        if det.abs() < f64::EPSILON {
            panic!("The matrix is singular, and its inverse does not exist.");
        }

        let mut inverse_matrix = MatrixMN::empty();

        for i in 0..m {
            let mut row = Vec::new();

            for j in 0..n {
                let cofactor = match (i + j) % 2 == 0 {
                    true => self.delete_line_column(i, j).det(),
                    false => self.delete_line_column(i, j).det(),
                };

                row.push(cofactor / det);
            }

            inverse_matrix.values.push(row);
        }

        // Transpose the result because adjugate is transposed
        return inverse_matrix.transpose();

    }
}
