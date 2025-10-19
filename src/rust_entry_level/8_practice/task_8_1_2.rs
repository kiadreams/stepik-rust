fn input(x: usize) -> Vec<i16> {
    let v = std::io::stdin()
        .lines()
        .take(x)
        .map(|v| v.unwrap().trim().parse::<i16>().unwrap())
        .collect::<Vec<_>>();
    return v;
}

fn main() {
    let [x, a, b, c] = input(4)[..] else { todo!() };
    if a % x + b % x + c % x > 0 {
        println!("{x} не является делителем всех чисел"); 
    } else {
        println!("{x} является делителем чисел {a}, {b}, {c}"); 
    }
}
