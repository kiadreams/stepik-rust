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
        "Вероятность того, что чайник прослужит меньше двух лет, но больше года равна: {:.2}",
        x1 - x2
    );
}
