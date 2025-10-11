fn input(x: usize) -> Vec<u16> {
    let v = std::io::stdin()
                    .lines()
                    .take(x)
                    .map(|v| v.unwrap().trim().parse::<u16>().unwrap())
                    .collect::<Vec<_>>();
    return v;
}

fn main() {
    let [x1, x2] = input(2)[..] else { todo!() };
    println!("Вероятность попадания выученного вопроса: {:.3}",
        1.0 - x2 as f64 / x1 as f64
    );
}
