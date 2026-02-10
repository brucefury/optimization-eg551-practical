//! Golden Section Search algorithm implementation.

use super::types::{GssResult, StoppingCriterion};

/// Golden ratio complement: (sqrt(5) - 1) / 2.
const TAU: f64 = 0.6180339887498949;

/// Performs Golden Section Search to find the minimum of `f` on `[a, b]`.
///
/// # Arguments
/// * `f` - Objective function to minimise
/// * `a` - Left bracket endpoint
/// * `b` - Right bracket endpoint
/// * `eps` - Tolerance for the chosen stopping criterion
/// * `max_iter` - Maximum number of iterations (safety limit)
/// * `criterion` - Which stopping criterion to use
pub fn gss(
    f: &dyn Fn(f64) -> f64,
    mut a: f64,
    mut b: f64,
    eps: f64,
    max_iter: usize,
    criterion: StoppingCriterion,
) -> GssResult {
    let mut x1 = b - TAU * (b - a);
    let mut x2 = a + TAU * (b - a);
    let mut fx1 = f(x1);
    let mut fx2 = f(x2);

    let mut iterations = 0;

    for i in 1..=max_iter {
        if fx1 < fx2 {
            b = x2;
            x2 = x1;
            fx2 = fx1;
            x1 = b - TAU * (b - a);
            fx1 = f(x1);
        } else {
            a = x1;
            x1 = x2;
            fx1 = fx2;
            x2 = a + TAU * (b - a);
            fx2 = f(x2);
        }

        iterations = i;

        let converged = match criterion {
            StoppingCriterion::IntervalWidth => (b - a) < eps,
            StoppingCriterion::FunctionValueDiff => (fx1 - fx2).abs() < eps,
        };

        if converged {
            break;
        }
    }

    GssResult {
        a,
        b,
        x1,
        x2,
        fx1,
        fx2,
        iterations,
        interval_width: b - a,
        function_value_diff: (fx1 - fx2).abs(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// f(x) = x(x - 1) = x^2 - x, minimum at x = 0.5, f(0.5) = -0.25
    fn f_quadratic(x: f64) -> f64 {
        x * (x - 1.0)
    }

    #[test]
    fn interval_width_converges_to_half() {
        let result = gss(&f_quadratic, 0.0, 2.0, 0.01, 10000, StoppingCriterion::IntervalWidth);

        let midpoint = (result.x1 + result.x2) / 2.0;
        assert!((midpoint - 0.5).abs() < 0.01, "midpoint={midpoint}");
        assert!(result.interval_width < 0.01);
        assert!(result.iterations > 0);
    }

    #[test]
    fn function_value_diff_converges_to_half() {
        let result = gss(
            &f_quadratic,
            0.0,
            2.0,
            1e-8,
            10000,
            StoppingCriterion::FunctionValueDiff,
        );

        let midpoint = (result.x1 + result.x2) / 2.0;
        assert!((midpoint - 0.5).abs() < 1e-4, "midpoint={midpoint}");
        assert!(result.function_value_diff < 1e-8);
    }

    #[test]
    fn max_iter_is_respected() {
        let result = gss(&f_quadratic, 0.0, 2.0, 1e-100, 5, StoppingCriterion::IntervalWidth);

        assert_eq!(result.iterations, 5);
    }

    #[test]
    fn works_with_x_squared() {
        // f(x) = x^2, minimum at x = 0 on [-2, 3]
        let f = |x: f64| x * x;
        let result = gss(&f, -2.0, 3.0, 0.001, 10000, StoppingCriterion::IntervalWidth);

        let midpoint = (result.x1 + result.x2) / 2.0;
        assert!(midpoint.abs() < 0.01, "midpoint={midpoint}");
        assert!(result.interval_width < 0.001);
    }
}
