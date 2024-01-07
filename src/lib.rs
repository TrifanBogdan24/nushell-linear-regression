
pub mod matrix_operations {
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
    use crate::matrix_operations::*;


    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }


    #[test]
    fn creating_a_matrix() {
        let mut vector: Vec<f64> = Vec::new();

        for i in 1..=20 {
            vector.push(i as f64);
        }

        let m: usize = 2;       // number of lines
        let n: usize = 10;      // number of columns

        let mat: MatrixMN = MatrixMN::create_matrix(&vector,m, n);

        assert!(true);
    }

    #[test]
    fn check_matrix_dimensions() {
        let mut vector: Vec<f64> = Vec::new();

        for i in 1..=20 {
            vector.push(i as f64);
        }

        let m: usize = 2;       // number of lines
        let n: usize = 10;      // number of columns

        let mat: MatrixMN = MatrixMN::create_matrix(&vector,m, n);

        if mat.nr_lines() != m || mat.height() != m {
            assert!(false);
        }
        if mat.nr_columns() != n || mat.length() != n {
            assert!(false);
        }
        assert!(true);
    }


    #[test]
    fn check_matrix_elements() {
        let mut vector: Vec<f64> = Vec::new();

        for i in 1..=20 {
            vector.push(i as f64);
        }

        let m: usize = 2;       // number of lines
        let n: usize = 10;      // number of columns
        let mat: MatrixMN = MatrixMN::create_matrix(&vector,m, n);

        let values: Vec<f64> = mat.get_vector();

        if values.len() != vector.len() {
            assert!(false);
        }

        for i in 0..=19 {
            if values[i] != vector[i] {
                assert!(false);
            }
        }

        assert!(true);
    }
    #[test]
    /// the function will verify
    /// the elements, the height and the length of the reshaped matrix
    fn change_matrix_dimensions_1() {
        let mut vector: Vec<f64> = Vec::new();
        for i in 1..=20 {
            vector.push(i as f64);
        }

        let m: usize = 2;       // number of lines
        let n: usize = 10;      // number of columns

        let mut mat: MatrixMN = MatrixMN::create_matrix(&vector,m, n);
        let values: Vec<f64> = mat.get_vector();

        mat.set_sizes(1, 20);

        if mat.nr_lines() != 1 || mat.height() != 1 {
            assert!(false);
        }
        if mat.nr_columns() != 20 || mat.length() != 20 {
            assert!(false);
        }
        if values.len() != vector.len() {
            assert!(false);
        }

        for i in 0..=19 {
            if values[i] != vector[i] {
                assert!(false);
            }
        }

        assert!(true);
    }


    #[test]
    /// the function will verify
    /// the elements, the height and the length of the reshaped matrix
    fn change_matrix_dimensions_2() {
        let mut vector: Vec<f64> = Vec::new();
        for i in 1..=20 {
            vector.push(i as f64);
        }

        let m: usize = 2;       // number of lines
        let n: usize = 10;      // number of columns

        let mut mat: MatrixMN = MatrixMN::create_matrix(&vector, m, n);
        let values: Vec<f64> = mat.get_vector();

        mat.set_sizes(2, 10);

        if mat.nr_lines() != 2 || mat.height() != 2 {
            assert!(false);
        }
        if mat.nr_columns() != 10 || mat.length() != 10 {
            assert!(false);
        }
        if values.len() != vector.len() {
            assert!(false);
        }

        for i in 0..=19 {
            if values[i] != vector[i] {
                assert!(false);
            }
        }

        assert!(true);
    }

    #[test]
    /// the function will verify
    /// the elements, the height and the length of the reshaped matrix
    fn change_matrix_dimensions_3() {
        let mut vector: Vec<f64> = Vec::new();
        for i in 1..=20 {
            vector.push(i as f64);
        }

        let m: usize = 2;       // number of lines
        let n: usize = 10;      // number of columns

        let mut mat: MatrixMN = MatrixMN::create_matrix(&vector,m, n);
        let mut values: Vec<f64> = mat.get_vector();

        mat.set_sizes(4, 5);

        if mat.nr_lines() != 4 || mat.height() != 4 {
            assert!(false);
        }
        if mat.nr_columns() != 5 || mat.length() != 5 {
            assert!(false);
        }
        if values.len() != vector.len() {
            assert!(false);
        }

        for i in 0..=19 {
            if values[i] != vector[i] {
                assert!(false);
            }
        }

        assert!(true);
    }

    #[test]
    /// the function will verify
    /// the elements, the height and the length of the reshaped matrix
    fn change_matrix_dimensions_4() {
        let mut vector: Vec<f64> = Vec::new();
        for i in 1..=20 {
            vector.push(i as f64);
        }

        let m: usize = 2;       // number of lines
        let n: usize = 10;      // number of columns

        let mut mat: MatrixMN = MatrixMN::create_matrix(&vector,m, n);
        let values: Vec<f64> = mat.get_vector();

        mat.set_sizes(20, 1);

        if mat.nr_lines() != 20 || mat.height() != 20 {
            assert!(false);
        }
        if mat.nr_columns() != 1 || mat.length() != 1 {
            assert!(false);
        }
        if values.len() != vector.len() {
            assert!(false);
        }

        for i in 0..=19 {
            if values[i] != vector[i] {
                assert!(false);
            }
        }

        assert!(true);
    }

    #[test]
    #[should_panic]
    fn change_matrix_invalid_dimensions_1() {
        let mut vector: Vec<f64> = Vec::new();
        for i in 1..=20 {
            vector.push(i as f64);
        }

        let m: usize = 2;       // number of lines
        let n: usize = 10;      // number of columns

        let mut mat: MatrixMN = MatrixMN::create_matrix(&vector,m, n);
        let values: Vec<f64> = mat.get_vector();

        // setting a smaller number of elements
        mat.set_sizes(4, 4);        // this should panic!
        assert!(false, "The reshaped matrix must have the same number of elements as the initial matrix does.");
    }

    #[test]
    #[should_panic]
    fn change_matrix_invalid_dimensions_2() {
        let mut vector: Vec<f64> = Vec::new();
        for i in 1..=20 {
            vector.push(i as f64);
        }

        let m: usize = 2;       // number of lines
        let n: usize = 10;      // number of columns

        let mut mat: MatrixMN = MatrixMN::create_matrix(&vector,m, n);
        let values: Vec<f64> = mat.get_vector();

        // setting with a bigger number of elements
        mat.set_sizes(4, 10);       // this should panic!
        assert!(false, "The reshaped matrix must have the same number of elements as the initial matrix does.");
    }


    #[test]
    /// m = the height of the matrix = number of lines of the matrix
    fn reshape_by_setting_height_1() {
        let mut vector: Vec<f64> = Vec::new();
        for i in 1..=20 {
            vector.push(i as f64);
        }

        let m: usize = 2;       // number of lines
        let n: usize = 10;      // number of columns

        let mut mat: MatrixMN = MatrixMN::create_matrix(&vector,m, n);

        mat.set_height(1);      // height = number of lines
        let values: Vec<f64> = mat.get_vector();

        if mat.nr_lines() != 1 || mat.height() != 1 {
            assert!(false);
        }
        if mat.nr_columns() != 20 || mat.length() != 20 {
            assert!(false);
        }
        if values.len() != vector.len() {
            assert!(false);
        }

        for i in 0..=19 {
            if values[i] != vector[i] {
                assert!(false);
            }
        }

        assert!(true);
    }


    #[test]
    /// m = the height of the matrix = number of lines of the matrix
    fn reshape_by_setting_height_2() {
        let mut vector: Vec<f64> = Vec::new();
        for i in 1..=20 {
            vector.push(i as f64);
        }

        let m: usize = 2;       // number of lines
        let n: usize = 10;      // number of columns

        let mut mat: MatrixMN = MatrixMN::create_matrix(&vector,m, n);

        mat.set_height(2);      // height = number of lines
        let values: Vec<f64> = mat.get_vector();

        if mat.nr_lines() != 2 || mat.height() != 2 {
            assert!(false);
        }
        if mat.nr_columns() != 10 || mat.length() != 10 {
            assert!(false);
        }
        if values.len() != vector.len() {
            assert!(false);
        }

        for i in 0..=19 {
            if values[i] != vector[i] {
                assert!(false);
            }
        }

        assert!(true);
    }


    #[test]
    /// m = the height of the matrix = number of lines of the matrix
    fn reshape_by_setting_height_3() {
        let mut vector: Vec<f64> = Vec::new();
        for i in 1..=20 {
            vector.push(i as f64);
        }

        let m: usize = 2;       // number of lines
        let n: usize = 10;      // number of columns

        let mut mat: MatrixMN = MatrixMN::create_matrix(&vector,m, n);

        mat.set_height(4);      // height = number of lines
        let values: Vec<f64> = mat.get_vector();

        if mat.nr_lines() != 4 || mat.height() != 4 {
            assert!(false);
        }
        if mat.nr_columns() != 5 || mat.length() != 5 {
            assert!(false);
        }
        if values.len() != vector.len() {
            assert!(false);
        }

        for i in 0..=19 {
            if values[i] != vector[i] {
                assert!(false);
            }
        }

        assert!(true);
    }


    #[test]
    /// m = the height of the matrix = number of lines of the matrix
    fn reshape_by_setting_height_4() {
        let mut vector: Vec<f64> = Vec::new();
        for i in 1..=20 {
            vector.push(i as f64);
        }

        let m: usize = 2;       // number of lines
        let n: usize = 10;      // number of columns

        let mut mat: MatrixMN = MatrixMN::create_matrix(&vector,m, n);

        mat.set_height(5);      // height = number of lines
        let values: Vec<f64> = mat.get_vector();

        if mat.nr_lines() != 5 || mat.height() != 5 {
            assert!(false);
        }
        if mat.nr_columns() != 4 || mat.length() != 4 {
            assert!(false);
        }
        if values.len() != vector.len() {
            assert!(false);
        }

        for i in 0..=19 {
            if values[i] != vector[i] {
                assert!(false);
            }
        }

        assert!(true);
    }


    #[test]
    #[should_panic]
    /// m = the height of the matrix = number of lines of the matrix
    fn invalid_reshape_by_setting_height_1() {
        let mut vector: Vec<f64> = Vec::new();
        for i in 1..=20 {
            vector.push(i as f64);
        }

        let m: usize = 2;       // number of lines
        let n: usize = 10;      // number of columns

        let mut mat: MatrixMN = MatrixMN::create_matrix(&vector, m, n);

        mat.set_height(0);      // should panic
        assert!(false, "Cannot assign ZERO to be number of lines.");
    }

    #[test]
    #[should_panic]
    /// m = the height of the matrix = number of lines of the matrix
    fn invalid_reshape_by_setting_height_2() {
        let mut vector: Vec<f64> = Vec::new();
        for i in 1..=20 {
            vector.push(i as f64);
        }

        let m: usize = 2;       // number of lines
        let n: usize = 10;      // number of columns

        let mut mat: MatrixMN = MatrixMN::create_matrix(&vector, m, n);

        mat.set_height(3);      // should panic
        assert!(false, "The number of line must be divisor of the number of all elements.");
    }

    #[test]
    #[should_panic]
    /// m = the height of the matrix = number of lines of the matrix
    fn invalid_reshape_by_setting_height_3() {
        let mut vector: Vec<f64> = Vec::new();
        for i in 1..=20 {
            vector.push(i as f64);
        }

        let m: usize = 2;       // number of lines
        let n: usize = 10;      // number of columns

        let mut mat: MatrixMN = MatrixMN::create_matrix(&vector, m, n);

        mat.set_height(21);      // should panic
        assert!(false, "The number of line must be divisor of the number of all elements.");
    }


    #[test]
    /// n = the length of the matrix = number of columns of the matrix
    fn reshape_by_setting_length_1() {
        let mut vector: Vec<f64> = Vec::new();
        for i in 1..=20 {
            vector.push(i as f64);
        }

        let m: usize = 2;       // number of lines
        let n: usize = 10;      // number of columns

        let mut mat: MatrixMN = MatrixMN::create_matrix(&vector,m, n);

        mat.set_length(1);      // length = number of columns

        let values: Vec<f64> = mat.get_vector();

        if mat.nr_lines() != 20 || mat.height() != 20 {
            assert!(false);
        }
        if mat.nr_columns() != 1 || mat.length() != 1 {
            assert!(false);
        }
        if values.len() != vector.len() {
            assert!(false);
        }

        for i in 0..=19 {
            if values[i] != vector[i] {
                assert!(false);
            }
        }

        assert!(true);
    }


    #[test]
    /// n = the length of the matrix = number of columns of the matrix
    fn reshape_by_setting_length_2() {
        let mut vector: Vec<f64> = Vec::new();
        for i in 1..=20 {
            vector.push(i as f64);
        }

        let m: usize = 2;       // number of lines
        let n: usize = 10;      // number of columns

        let mut mat: MatrixMN = MatrixMN::create_matrix(&vector,m, n);

        mat.set_length(2);      // length = number of columns

        let values: Vec<f64> = mat.get_vector();

        if mat.nr_lines() != 10 || mat.height() != 10 {
            assert!(false);
        }
        if mat.nr_columns() != 2 || mat.length() != 2 {
            assert!(false);
        }
        if values.len() != vector.len() {
            assert!(false);
        }

        for i in 0..=19 {
            if values[i] != vector[i] {
                assert!(false);
            }
        }

        assert!(true);
    }


    #[test]
    /// n = the length of the matrix = number of columns of the matrix
    fn reshape_by_setting_length_3() {
        let mut vector: Vec<f64> = Vec::new();
        for i in 1..=20 {
            vector.push(i as f64);
        }

        let m: usize = 2;       // number of lines
        let n: usize = 10;      // number of columns

        let mut mat: MatrixMN = MatrixMN::create_matrix(&vector,m, n);

        mat.set_length(4);      // length = number of columns

        let values: Vec<f64> = mat.get_vector();

        if mat.nr_lines() != 5 || mat.height() != 5 {
            assert!(false);
        }
        if mat.nr_columns() != 4 || mat.length() != 4 {
            assert!(false);
        }
        if values.len() != vector.len() {
            assert!(false);
        }

        for i in 0..=19 {
            if values[i] != vector[i] {
                assert!(false);
            }
        }

        assert!(true);
    }


    #[test]
    /// n = the length of the matrix = number of columns of the matrix
    fn reshape_by_setting_length_4() {
        let mut vector: Vec<f64> = Vec::new();
        for i in 1..=20 {
            vector.push(i as f64);
        }

        let m: usize = 2;       // number of lines
        let n: usize = 10;      // number of columns

        let mut mat: MatrixMN = MatrixMN::create_matrix(&vector,m, n);

        mat.set_length(5);      // length = number of columns
        let values: Vec<f64> = mat.get_vector();

        if mat.nr_lines() != 4 || mat.height() != 4 {
            assert!(false);
        }
        if mat.nr_columns() != 5 || mat.length() != 5 {
            assert!(false);
        }
        if values.len() != vector.len() {
            assert!(false);
        }

        for i in 0..=19 {
            if values[i] != vector[i] {
                assert!(false);
            }
        }

        assert!(true);
    }


    #[test]
    #[should_panic]
    /// n = the length of the matrix = number of columns of the matrix
    fn invalid_reshape_by_setting_length_1() {
        let mut vector: Vec<f64> = Vec::new();
        for i in 1..=20 {
            vector.push(i as f64);
        }

        let m: usize = 2;       // number of lines
        let n: usize = 10;      // number of columns

        let mut mat: MatrixMN = MatrixMN::create_matrix(&vector, m, n);

        mat.set_length(0);      // should panic
        assert!(false, "Cannot assign ZERO to be number of lines.");
    }

    #[test]
    #[should_panic]
    /// n = the length of the matrix = number of columns of the matrix
    fn invalid_reshape_by_setting_length_2() {
        let mut vector: Vec<f64> = Vec::new();
        for i in 1..=20 {
            vector.push(i as f64);
        }

        let m: usize = 2;       // number of lines
        let n: usize = 10;      // number of columns

        let mut mat: MatrixMN = MatrixMN::create_matrix(&vector, m, n);

        mat.set_length(3);      // should panic
        assert!(false, "The number of columns must be divisor of the number of all elements.");
    }

    #[test]
    #[should_panic]
    /// n = the length of the matrix = number of columns of the matrix
    fn invalid_reshape_by_setting_length_3() {
        let mut vector: Vec<f64> = Vec::new();
        for i in 1..=20 {
            vector.push(i as f64);
        }

        let m: usize = 2;       // number of lines
        let n: usize = 10;      // number of columns

        let mut mat: MatrixMN = MatrixMN::create_matrix(&vector, m, n);

        mat.set_length(21);      // should panic
        assert!(false, "The number of columns must be divisor of the number of all elements.");
    }

    #[test]
    fn resize_matrix_1() {
        let mut vector: Vec<f64> = Vec::new();
        for i in 1..=20 {
            vector.push(i as f64);
        }

        let m: usize = 2;       // number of lines
        let n: usize = 10;      // number of columns

        let mat1: MatrixMN = MatrixMN::create_matrix(&vector, m, n);
        let values1: Vec<f64> = mat1.get_vector();

        let mat2: MatrixMN = mat1.resize(1, 20);
        let values2: Vec<f64> = mat2.get_vector();

        if mat2.nr_lines() != 1 || mat2.height() != 1 {
            assert!(false);
        }
        if mat2.nr_columns() != 20 || mat2.length() != 20 {
            assert!(false);
        }

        if values1.len() != values2.len() {
            assert!(false);
        }

        for i in 0..=19 {
            if values1[i] != values2[i] {
                assert!(false);
            }

            assert!(true);
        }
    }

    #[test]
    fn resize_matrix_2() {
        let mut vector: Vec<f64> = Vec::new();
        for i in 1..=20 {
            vector.push(i as f64);
        }

        let m: usize = 2;       // number of lines
        let n: usize = 10;      // number of columns

        let mat1: MatrixMN = MatrixMN::create_matrix(&vector, m, n);
        let values1: Vec<f64> = mat1.get_vector();

        let mat2: MatrixMN = mat1.resize(2, 10);
        let values2: Vec<f64> = mat2.get_vector();

        if mat2.nr_lines() != 2 || mat2.height() != 2 {
            assert!(false);
        }
        if mat2.nr_columns() != 10 || mat2.length() != 10 {
            assert!(false);
        }

        if values1.len() != values2.len() {
            assert!(false);
        }

        for i in 0..=19 {
            if values1[i] != values2[i] {
                assert!(false);
            }

            assert!(true);
        }
    }

    #[test]
    fn resize_matrix_3() {
        let mut vector: Vec<f64> = Vec::new();
        for i in 1..=20 {
            vector.push(i as f64);
        }

        let m: usize = 2;       // number of lines
        let n: usize = 10;      // number of columns

        let mat1: MatrixMN = MatrixMN::create_matrix(&vector, m, n);
        let values1: Vec<f64> = mat1.get_vector();

        let mat2: MatrixMN = mat1.resize(4, 5);
        let values2: Vec<f64> = mat2.get_vector();

        if mat2.nr_lines() != 4 || mat2.height() != 4 {
            assert!(false);
        }
        if mat2.nr_columns() != 5 || mat2.length() != 5 {
            assert!(false);
        }

        if values1.len() != values2.len() {
            assert!(false);
        }

        for i in 0..=19 {
            if values1[i] != values2[i] {
                assert!(false);
            }

            assert!(true);
        }
    }

    #[test]
    fn resize_matrix_4() {
        let mut vector: Vec<f64> = Vec::new();
        for i in 1..=20 {
            vector.push(i as f64);
        }

        let m: usize = 2;       // number of lines
        let n: usize = 10;      // number of columns

        let mat1: MatrixMN = MatrixMN::create_matrix(&vector, m, n);
        let values1: Vec<f64> = mat1.get_vector();

        let mat2: MatrixMN = mat1.resize(5, 4);
        let values2: Vec<f64> = mat2.get_vector();

        if mat2.nr_lines() != 5 || mat2.height() != 5 {
            assert!(false);
        }
        if mat2.nr_columns() != 4 || mat2.length() != 4 {
            assert!(false);
        }

        if values1.len() != values2.len() {
            assert!(false);
        }

        for i in 0..=19 {
            if values1[i] != values2[i] {
                assert!(false);
            }

            assert!(true);
        }
    }

    #[test]
    fn resize_matrix_5() {
        let mut vector: Vec<f64> = Vec::new();
        for i in 1..=20 {
            vector.push(i as f64);
        }

        let m: usize = 2;       // number of lines
        let n: usize = 10;      // number of columns

        let mat1: MatrixMN = MatrixMN::create_matrix(&vector, m, n);
        let values1: Vec<f64> = mat1.get_vector();

        let mat2: MatrixMN = mat1.resize(10, 2);
        let values2: Vec<f64> = mat2.get_vector();

        if mat2.nr_lines() != 10 || mat2.height() != 10 {
            assert!(false);
        }
        if mat2.nr_columns() != 2 || mat2.length() != 2 {
            assert!(false);
        }

        if values1.len() != values2.len() {
            assert!(false);
        }

        for i in 0..=19 {
            if values1[i] != values2[i] {
                assert!(false);
            }

            assert!(true);
        }
    }

    #[test]
    fn resize_matrix_6() {
        let mut vector: Vec<f64> = Vec::new();
        for i in 1..=20 {
            vector.push(i as f64);
        }

        let m: usize = 2;       // number of lines
        let n: usize = 10;      // number of columns

        let mat1: MatrixMN = MatrixMN::create_matrix(&vector, m, n);
        let values1: Vec<f64> = mat1.get_vector();

        let mat2: MatrixMN = mat1.resize(20, 1);
        let values2: Vec<f64> = mat2.get_vector();

        if mat2.nr_lines() != 20 || mat2.height() != 20 {
            assert!(false);
        }
        if mat2.nr_columns() != 1 || mat2.length() != 1 {
            assert!(false);
        }

        if values1.len() != values2.len() {
            assert!(false);
        }

        for i in 0..=19 {
            if values1[i] != values2[i] {
                assert!(false);
            }

            assert!(true);
        }
    }

    #[test]
    #[should_panic]
    fn invalid_resize_matrix_1() {
        let mut vector: Vec<f64> = Vec::new();
        for i in 1..=20 {
            vector.push(i as f64);
        }

        let m: usize = 2;       // number of lines
        let n: usize = 10;      // number of columns

        let mat1: MatrixMN = MatrixMN::create_matrix(&vector, m, n);

        // setting a smaller number of elements
        let mat2: MatrixMN = mat1.resize(4, 4);     // this should panic!
        assert!(false, "The new resized matrix must have the same number of elements as the initial one.");
    }

    #[test]
    #[should_panic]
    fn invalid_resize_matrix_2() {
        let mut vector: Vec<f64> = Vec::new();
        for i in 1..=20 {
            vector.push(i as f64);
        }

        let m: usize = 2;       // number of lines
        let n: usize = 10;      // number of columns

        let mat1: MatrixMN = MatrixMN::create_matrix(&vector, m, n);

        // setting a bigger number of elements
        let mat2: MatrixMN = mat1.resize(5, 5);     // this should panic!
        assert!(false, "The new resized matrix must have the same number of elements as the initial one.");
    }
}
