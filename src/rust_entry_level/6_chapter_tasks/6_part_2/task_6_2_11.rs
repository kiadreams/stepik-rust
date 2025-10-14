fn input(x: usize) -> Vec<f64> {
    let v = std::io::stdin()
                    .lines()
                    .take(x)
                    .map(|v| v.unwrap().trim().parse::<f64>().unwrap())
                    .collect::<Vec<_>>();
    return v;
}

fn main() {
    let [x1, x2, x3, x4] = input(4)[..] else { todo!() };
    println!(
        "За {} день турист прошел {:.3} км",
        x2,
        x1 + (2.0 * x4 - 2.0 * x1 * x3) * (x2 - 1.0) / (x3 - 1.0) / x3
    );
}
