#[allow(unused)]
use optimization_eg551_practical::*;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let week: u32 = match args.get(1).and_then(|s| s.parse().ok()) {
        Some(w) if (1..=10).contains(&w) => w,
        _ => {
            println!("Usage: {} <week>", args[0]);
            println!("Available weeks: 1-10");
            return Ok(());
        }
    };

    match week {
        1 => run_week01()?,
        2 => run_week02()?,
        3 => run_week03(&args)?,
        4 => println!("Week 4: not yet implemented"),
        5 => println!("Week 5: not yet implemented"),
        6 => println!("Week 6: not yet implemented"),
        7 => println!("Week 7: not yet implemented"),
        8 => println!("Week 8: not yet implemented"),
        9 => println!("Week 9: not yet implemented"),
        10 => println!("Week 10: not yet implemented"),
        _ => unreachable!(),
    }

    Ok(())
}

#[cfg(feature = "plotting")]
fn run_week01() -> Result<(), Box<dyn std::error::Error>> {
    let dir = "output/week01";
    std::fs::create_dir_all(dir)?;

    println!("Week 1: Graphing demos and problems");

    println!("  Running demo_line_plot...");
    week01_graphing::demo_line_plot(&format!("{}/demo_line.png", dir))?;

    println!("  Running demo_multi_line_plot...");
    week01_graphing::demo_multi_line_plot(&format!("{}/demo_multi_line.png", dir))?;

    println!("  Running demo_bar_chart...");
    week01_graphing::demo_bar_chart(&format!("{}/demo_bar_chart.png", dir))?;

    println!("  Running problem_table_plot...");
    week01_graphing::problems::problem_table_plot(dir)?;

    println!("  Running problem_sin_reciprocal...");
    week01_graphing::problems::problem_sin_reciprocal(dir)?;

    println!("  Running problem_non_smooth...");
    week01_graphing::problems::problem_non_smooth(dir)?;

    println!("  Running problem_piecewise...");
    week01_graphing::problems::problem_piecewise(dir)?;

    println!("  Running problem_rational...");
    week01_graphing::problems::problem_rational(dir)?;

    println!("Week 1 complete. Output in {}/", dir);
    Ok(())
}

#[cfg(not(feature = "plotting"))]
fn run_week01() -> Result<(), Box<dyn std::error::Error>> {
    println!("Week 1 requires the 'plotting' feature.");
    println!("Run with: cargo run --features plotting -- 1");
    Ok(())
}

#[cfg(feature = "plotting")]
fn run_week02() -> Result<(), Box<dyn std::error::Error>> {
    let dir = "output/week02";
    std::fs::create_dir_all(dir)?;

    println!("Week 2: Neville's Interpolation");

    println!("  Running problem_neville_plots...");
    week02_neville::problems::problem_neville_plots(dir)?;

    println!("Week 2 complete. Output in {}/", dir);
    Ok(())
}

#[cfg(not(feature = "plotting"))]
fn run_week02() -> Result<(), Box<dyn std::error::Error>> {
    println!("Week 2 requires the 'plotting' feature.");
    println!("Run with: cargo run --features plotting -- 2");
    Ok(())
}

fn run_week03(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    use week03_golden_section::{gss, StoppingCriterion};

    let mut a = 0.0_f64;
    let mut b = 2.0_f64;
    let mut eps = 0.1_f64;
    let mut max_iter = 10000_usize;
    let mut criterion = StoppingCriterion::IntervalWidth;

    // Parse optional arguments after the week number
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--a" => {
                i += 1;
                a = args[i].parse().expect("invalid value for --a");
            }
            "--b" => {
                i += 1;
                b = args[i].parse().expect("invalid value for --b");
            }
            "--eps" => {
                i += 1;
                eps = args[i].parse().expect("invalid value for --eps");
            }
            "--max-iter" => {
                i += 1;
                max_iter = args[i].parse().expect("invalid value for --max-iter");
            }
            "--stop" => {
                i += 1;
                criterion = match args[i].as_str() {
                    "interval" => StoppingCriterion::IntervalWidth,
                    "function" => StoppingCriterion::FunctionValueDiff,
                    other => {
                        eprintln!("Unknown stopping criterion: {other}");
                        eprintln!("Use \"interval\" or \"function\"");
                        return Ok(());
                    }
                };
            }
            other => {
                eprintln!("Unknown option: {other}");
                eprintln!("Usage: eg551 3 [--a VALUE] [--b VALUE] [--eps VALUE] [--max-iter VALUE] [--stop interval|function]");
                return Ok(());
            }
        }
        i += 1;
    }

    let criterion_label = match criterion {
        StoppingCriterion::IntervalWidth => format!("Interval Width < {eps}"),
        StoppingCriterion::FunctionValueDiff => format!("|f(x1) - f(x2)| < {eps}"),
    };

    println!("Week 3: Golden Section Search");
    println!("  f(x) = x(x - 1) on [{a}, {b}]");
    println!("  Stopping criterion: {criterion_label}");
    println!("  Max iterations: {max_iter}");
    println!();

    let f = |x: f64| x * (x - 1.0);
    let result = gss(&f, a, b, eps, max_iter, criterion);

    println!("Golden Section Search Results");
    println!("  Stopping criterion: {criterion_label}");
    println!("  Iterations:         {}", result.iterations);
    println!("  Bracket:            [{:.6}, {:.6}]", result.a, result.b);
    println!(
        "  x1 = {:.6},  f(x1) = {:.6}",
        result.x1, result.fx1
    );
    println!(
        "  x2 = {:.6},  f(x2) = {:.6}",
        result.x2, result.fx2
    );
    println!("  Interval width:     {:.6}", result.interval_width);
    println!("  |f(x1) - f(x2)|:   {:.6}", result.function_value_diff);

    Ok(())
}
