fn input(x: usize) -> Vec<u32> {
    let v = std::io::stdin()
                    .lines()
                    .take(x)
                    .map(|v| v.unwrap().trim().parse::<u32>().unwrap())
                    .collect::<Vec<_>>();
    return v;
}

fn main() {
    let [x1, x2, x3] = input(3)[..] else { todo!() };
    println!(
        "Максимальная глубина кодирования: {}",
         x2 * 8 /(2 * x1 * (100 - x3) / 100)
    );
}