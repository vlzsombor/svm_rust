use ndarray::{Array, Array1, Array2, Ix, arr2};
pub struct Svm{
    pub weights: Vec<f64>,
    pub bias: f64,
    pub c: f64
}

impl Svm{
    pub fn new(feature_dimension: usize, c: Option<f64>)->Svm{
        Svm {
            weights: vec![0.0; feature_dimension],
            bias: 0.0,
            c: c.unwrap_or(1.0)
        }
    }

    fn calculate_matrix_dot_product(x: &Array2<f64>, row_index: Ix, column_index: Ix) -> f64 {
        let row1 = x.row(row_index);
        let row2 = x.row(column_index);
        row1.dot(&row2)
    }

    fn calculate_gram_matrix(x: &Array2<f64>) -> Array2<f64> {
        let length = x.nrows();
        let mut return_array: Array2<f64> = Array2::zeros((length, length));
        for i in 0..length{
            for j in 0..length{
                let dot_product = Self::calculate_matrix_dot_product(x, i, j);
                return_array[[i,j]] = dot_product;
            }
        }
        return_array
    }
    fn matrix_vector_multiply(x: &Array2<f64>, y: &Array1<f64>) -> Array2<f64> {
        let n = y.len();
        let mut xy: Array2<f64> = Array2::zeros((n, n));
        for i in 0..x.nrows() {
            for j in 0..x.ncols() {
                xy[[i, j]] = x[[i,j]] * y[i];
            }
        }
        xy
    }
    fn fit(&mut self, x: Array2<f64>, y: Array1<f64>){
        let N = y.len();
        let yX = Self::matrix_vector_multiply(&x,&y);
        let gramX = yX.dot(&yX.t());

    }


    fn optimize_test(){

    }

    fn objective(x: &Array1<f64>) -> f64 {
        // Example bound transform for x within [0, 2]
        let constrained_x = 2.0 / (1.0 + (-x[0]).exp());
        // Use constrained_x in the objective
        (constrained_x - 1.0).powi(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_matrix_dot_product_test(){
        let arr = Array2::from_shape_vec((2, 2), vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let res = Svm::calculate_matrix_dot_product(&arr, 0, 0);

        assert_eq!(res, 5.0);
    }


    #[test]
    fn matrix_vector_test(){
        use ndarray::arr2;
        let x = Array2::from_shape_vec((2, 2), vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let y = Array1::from_shape_vec(2, vec![-1.,1.]).unwrap();
        let res = Svm::matrix_vector_multiply(&x, &y);
        assert_eq!(res, arr2(&[[-1.,-2.], [3.,4.]]));
    }
    #[test]
    fn gram_matrix_test(){
        let arr = Array2::from_shape_vec((2, 2), vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let res = Svm::calculate_gram_matrix(&arr);
        assert_eq!(res, arr2(&[[5.0, 11.0], [11.0, 25.0]]));
        let matmul = arr.dot(&arr.t());
        assert_eq!(res, &matmul);
    }

    #[test]
    fn new_test() {
        let c = 1.0;
        let w_dim = 16;
        let svm = Svm::new(w_dim, Option::from(c));

        assert_eq!(svm.c, c);
        assert_eq!(svm.bias, 0.0);
        assert_eq!(svm.weights.len(), w_dim);
    }

    #[test]
    fn new_test2() {
        let c = 1.0;
        let w_dim = 16;
        let svm = Svm::new(w_dim, None);

        assert_eq!(svm.c, c);
        assert_eq!(svm.bias, 0.0);
        assert_eq!(svm.weights.len(), w_dim);
    }
}
