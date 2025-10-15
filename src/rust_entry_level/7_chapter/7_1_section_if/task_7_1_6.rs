fn input(x: usize) -> Vec<i64> {
    let v = std::io::stdin()
        .lines()
        .take(x)
        .map(|v| v.unwrap().trim().parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    return v;
}

fn main() {
    let [x1, x2, x3] = input(3)[..] else { todo!() };
    if x1 == x2 || x2 == x3 || x1 == x3{
        println!("Числа неразличны");
    } else {
        println!("Числа различны");
    }
}
