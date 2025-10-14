fn input(x: usize) -> Vec<f64> {
    let v = std::io::stdin()
                    .lines()
                    .take(x)
                    .map(|v| v.unwrap().trim().parse::<f64>().unwrap())
                    .collect::<Vec<_>>();
    return v;
}

fn main() {
    let [x1, x2, x3] = input(3)[..] else { todo!() };
    println!(
        "Для получения {:.3} кг изюма необходимо {:.3} кг винограда",
        x1,
        (x1 * (1.0 - x3 / 100.0)) / (1.0 - x2 / 100.0)
    );
}
