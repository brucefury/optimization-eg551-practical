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
        3 => println!("Week 3: not yet implemented"),
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

fn run_week02() -> Result<(), Box<dyn std::error::Error>> {
    println!("Week 2: Neville's Interpolation");

    // Sample points from y = x^2
    let points = vec![(-1.0, 1.0), (0.0, 0.0), (1.0, 1.0), (2.0, 4.0)];
    let test_x = 0.5;

    let result = week02_neville::neville_stock(&points, test_x);
    println!("  Interpolating y = x² at x = {}", test_x);
    println!("  Points: {:?}", points);
    println!("  Neville result: {}", result.value);
    println!("  Expected:       {}", test_x * test_x);
    println!("  Error:          {:.2e}", (result.value - test_x * test_x).abs());

    // Second example with different target
    let test_x2 = 1.5;
    let result2 = week02_neville::neville_stock(&points, test_x2);
    println!();
    println!("  Interpolating y = x² at x = {}", test_x2);
    println!("  Neville result: {}", result2.value);
    println!("  Expected:       {}", test_x2 * test_x2);
    println!("  Error:          {:.2e}", (result2.value - test_x2 * test_x2).abs());

    Ok(())
}
