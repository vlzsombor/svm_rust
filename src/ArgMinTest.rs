/*use argmin::prelude::*;
use argmin::solver::gradientdescent::SteepestDescent;
use argmin::core::Error;

// Definiere eine einfache quadratische Funktion
struct QuadraticFunction;

impl ArgminOp for QuadraticFunction {
    type Param = f64;
    type Output = f64;
    type Hessian = ();
    type Jacobian = ();

    fn apply(&self, param: &Self::Param) -> Result<Self::Output, Error> {
        Ok((param - 2.0).powi(2))
    }
}

// Führe die Optimierung durch
let operator = QuadraticFunction;
let solver = SteepestDescent::new();
let init_param = 0.0f64;

let result = Executor::new(operator, solver, init_param)
.add_observer(ArgminSlogLogger::term(), ObserverMode::Always)
.max_iters(10)
.run()
.unwrap();

println!("Minimum bei: {}", result.state().best_param);

 */