
pub mod matrix-operations {
    pub mod create;
    pub mod determinant;
    pub mod empty;
    pub mod eye;
    pub mod inverse;
    pub mod multiply;
    pub mod ones;
    pub mod shape;
    pub mod transpose;
    pub mod zeros;
}

pub mod matrix;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix::MatrixMN;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }


    #[test]
    fn creation() {
        let mut vector: Vec<f64> = Vec::new();

        for i in 1..=20 {
            vector.push(i as f64);
        }

        let m: usize = 2;       // number of lines
        let n: usize = 10;      // number of columns

        let mat: MatrixMN = MatrixMN::create_matrix(&vector,m, n);

        assert!(true);
    }
}
