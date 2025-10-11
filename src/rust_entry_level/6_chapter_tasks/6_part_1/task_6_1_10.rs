fn input(x: usize) -> Vec<u32> {
    let v = std::io::stdin()
                    .lines()
                    .take(x)
                    .map(|v| v.unwrap().trim().parse::<u32>().unwrap())
                    .collect::<Vec<_>>();
    return v;
}

fn main() {
    let [x1, x2, x3, x4] = input(4)[..] else { todo!() };
    println!(
        "Время передачи голосового сообщения: {} секунд",
         x4 * x3 * x2 * 2 / x1
    );
}