fn input(x: usize) -> Vec<i64> {
    let v = std::io::stdin()
        .lines()
        .take(x)
        .map(|v| v.unwrap().trim().parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    return v;
}

fn main() {
    let [year] = input(1)[..] else { todo!() };
    if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
        println!("{year} является високосным годом");
    } else {
        println!("{year} не является високосным годом");
    }
}
