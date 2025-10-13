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
        "Доля сухофруктов относительно свежих фруктов составляет: {:.3}%",
        x2 / x1 * 100.0
    );
    println!(
        "Процент массы, потерянный при сушке: {:.3}%",
        100_f64 - x2 / x1 * 100.0
    );
}
