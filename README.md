# Matrix operation
The structures the store variables in lines and columns:
```rust
pub struct MatrixMN {
    pub values: Vec<Vec<f64>>,
}
```

## Creating a matrix
```rust
let values: Vec<f64> = vec![1.0, 5.0];
let mat: MatrixMN = MatrixMN::create_matrix(&values, 1, 2);    // a vector of f64, nr of lines, nr of columns
```

## Accessing an element
```
mat.values[i][j]
```
- element on the line `i - 1` and `j - 1` column
- indexing starts from `0`

