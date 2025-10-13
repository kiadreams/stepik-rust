fn input(x: usize) -> Vec<f64> {
    let v = std::io::stdin()
                    .lines()
                    .take(x)
                    .map(|v| v.unwrap().trim().parse::<f64>().unwrap())
                    .collect::<Vec<_>>();
    return v;
}

fn main() {
    let [x1, x2, x3, x4, x5] = input(5)[..] else { todo!() };
    println!(
        "От прибыли {} рублей Тимуру причитается {:.3} рублей",
        x5,
        (1.0 - x2 / 100.0 - x4 - (x3 / x1)) * x5
    );
}