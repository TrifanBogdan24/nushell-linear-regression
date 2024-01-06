use crate::matrix::MatrixMN;


impl MatrixMN {
    pub fn ones(m: usize, n: usize) -> Self {
        validate_new_matrix_creation(m, n);

        let mut vector: Vec<f64> = Vec::new();

        for i in 0..=(m * n - 1) {
            vector.push(1.0f64);
        }

        return Self::create_matrix(&vector, m, n);
    }
}
