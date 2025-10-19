fn input(x: usize) -> Vec<i16> {
    let v = std::io::stdin()
        .lines()
        .take(x)
        .map(|v| v.unwrap().trim().parse::<i16>().unwrap())
        .collect::<Vec<_>>();
    return v;
}

fn main() {
    let [n] = input(1)[..] else { todo!() };
    let m = if n < 0 { -1 } else { 1 };
    let x1 = n / 100;
    let x2 = n % 100 / 10 * m;
    let x3 = n % 10 * m;
    println!("{}", x1 + x2 + x3);
}
