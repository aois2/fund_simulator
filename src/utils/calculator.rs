pub fn run_simulation(initial: f64, monthly: f64, rate: f64, years: u32) -> Vec<(u32, f64)> {
    let mut value = initial;
    let months = years * 12;
    let monthly_rate = rate / 12.0 / 100.0;
    let mut timeline = Vec::new();

    for month in 0..=months {
        timeline.push((month, value));
        value += monthly;
        value *= 1.0 + monthly_rate;
    }

    timeline
}
