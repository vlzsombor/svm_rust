// Copyright 2018-2024 argmin developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

mod main3;

#[allow(unused_imports)]
use argmin::{
    core::{observers::ObserverMode, CostFunction, Error, Executor, Gradient},
    solver::{
        gradientdescent::SteepestDescent,
        linesearch::{HagerZhangLineSearch, MoreThuenteLineSearch},
    },
};
use argmin_observer_slog::SlogLogger;
use argmin_testfunctions::{rosenbrock, rosenbrock_derivative};
use ndarray::{Array1, Array2};

struct Rosenbrock {
    pub gram_x: Array2<f64>,
    pub y: Array1<f64>
}

impl CostFunction for Rosenbrock {
    type Param = Array1<f64>;
    type Output = f64;

    fn cost(&self, p: &Self::Param) -> Result<Self::Output, Error> {
        //        let f = |x: &f64| (x-3.).powf(2.)-7.;
        //        let test = f(p); //p.iter().map(f).sum();
        let alpha_sum: f64 = p.iter().sum();
        let sum = p.dot(&p.dot(&self.gram_x));
        let res = alpha_sum - 0.5 * sum;

        let sum_y_alpha: f64 = p.iter().zip(self.y.iter()).map(|(a,yi)| a*yi).sum();
        let penalty = sum_y_alpha.powi(2);

        let alpha_penalty: f64 = p.iter()
            .map(|a| {
                let mut p = 0.0;
                if *a < 0.0 { p += a.powi(2); }
                p
            }).sum();
        return Ok(res + alpha_penalty + penalty);
        //        Err(Error::msg("shit"))
        //Ok(test)
        //Ok(rosenbrock(p))
    }
}

impl Gradient for Rosenbrock {
    type Param = Array1<f64>;
    type Gradient = Array1<f64>;

    fn gradient(&self, p: &Self::Param) -> Result<Self::Gradient, Error> {
        //        let f = |x: &f64| 2. * (x-3.);
        //        let test = p.iter().map(f).collect();
        //        let test = p.iter().map(|x| x).sum();
        //        Ok(test)
        //Ok(rosenbrock_derivative(p))
        let res1 :Array1<f64> = Array1::ones(p.len());
        let alphaDot = p.dot(&self.gram_x);
        let res = res1 - alphaDot;

        let sum_y_alpha: f64 = p.iter().zip(self.y.iter()).map(|(a, yi)| a * yi).sum();
        let d_penalty: Array1<f64> = self.y.iter()
            .map(|yi| 2.0* yi * sum_y_alpha)
            .collect();

        let d_alpha_penalty: Array1<f64> = p.iter()
            .map(|a| {
                if *a < 0.0 {
                    2.0 * a
                } else {
                    0.0
                }
            })
            .collect();

        let res2 = d_penalty + d_alpha_penalty;
        Ok(res + res2)
        //        Err(Error::msg("fdas"))

    }
}

fn run(gram_x: Array2<f64>, y: Array1<f64>) -> Result<(), Error> {
    // Define cost function (must implement `CostFunction` and `Gradient`)
    let N = y.len();
    let cost = Rosenbrock { gram_x, y};

    // Define initial parameter vector
    // easy case
    //let init_param: f64 = 30.0000000000001;
    // tough case
    //let init_param: Array1<f64> = Array1:: (cost.y.len());

    let init_param = Array1::from_elem(N, 0.1);
    // Pick a line search.
    let linesearch = HagerZhangLineSearch::new();
    //    let linesearch = MoreThuenteLineSearch::new();

    // Set up solver
    let solver = SteepestDescent::new(linesearch);

    // Run solver
    let res = Executor::new(cost, solver)
        .configure(|state| state.param(init_param).max_iters(1000))
        .add_observer(SlogLogger::term(), ObserverMode::Always)
        .run()?;

    // print result
    println!("{res}");
    Ok(())
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
fn main() {
    let y: Array1<f64> = Array1::from_vec(vec![1.0, -1.0, 1.0, 1.0, 1.0]);
    let x = vec![[2.0, 3.0], [1.0, -1.0], [2.0, 1.0], [4.,4.], [5., 5.]];

    //    let x = Array2::from_shape_vec((2, 2), vec![1.0, 2.0, 3.0, 4.0]).unwrap();
    let x_array =  Array2::from(x);

    let N = y.len();
    let yX = matrix_vector_multiply(&x_array,&y);
    let gramX = yX.dot(&yX.t());


    if let Err(ref e) = run(gramX, y) {
        println!("{e}");
        std::process::exit(1);
    }
}
