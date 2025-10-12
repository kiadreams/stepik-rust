fn input(x: usize) -> Vec<f64> {
    let v = std::io::stdin()
                    .lines()
                    .take(x)
                    .map(|v| v.unwrap().trim().parse::<f64>().unwrap())
                    .collect::<Vec<_>>();
    return v;
}

fn main() {
    let [x1, x2] = input(2)[..] else { todo!() };
    println!(
        "{:.3}",
        x1 * x2 / 100.0
    );
}