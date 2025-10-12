fn input(x: usize) -> Vec<f64> {
    let v = std::io::stdin()
                    .lines()
                    .take(x)
                    .map(|v| v.unwrap().trim().parse::<f64>().unwrap())
                    .collect::<Vec<_>>();
    return v;
}

fn main() {
    let [x1] = input(1)[..] else { todo!() };
    println!(
        "Из {:.3} тонн(ы) получится {:.3} кг сахара",
        x1,
        x1 * 1000.0 * 0.18
    );
}