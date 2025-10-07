use ndarray::{Array, Array1, Array2, Ix};
pub struct Svm{
    pub weights: Vec<f64>,
    pub bias: f64,
    pub c: f64
}

impl Svm{
    fn new(feature_dimension: usize, c: Option<f64>)->Svm{
        Svm {
            weights: vec![0.0; feature_dimension],
            bias: 0.0,
            c: c.unwrap_or(1.0)
        }
    }

    fn calculate_gram_matrix(x: Array2<f64>, row_index: Ix, column_index: Ix) -> f64 {
        let row1 = x.row(row_index);
        let row2 = x.row(column_index);
        row1.dot(&row2)
    }

    fn fit(&mut self, x: Array2<f64>, y: Array1<f64>){
        let N = y.len();
        let row0 = x.row(0);
        row0.dot(&row0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fit_test(){
        let c = 1.0;
        let w_dim = 16;
        let mut svm = Svm::new(w_dim, Option::from(c));
        let arr1 = Array1::from_vec(vec![1.0, 2.0, 3.0]);
        let arr2 = Array2::from_shape_vec((2, 2), vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let arr3 = Array2::from(vec![[1,2],[3,4]]);
        let arr: Array2<i32> = Array2::from_shape_vec((2, 2), vec![1, 2, 3, 4]).unwrap();
        let two = arr[[0, 1]];
        let three = arr[[1, 0]];
        let res = Svm::calculate_gram_matrix(arr2, 0, 0);

//        svm.fit(arr2, arr1);
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
