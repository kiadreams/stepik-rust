fn input(x: usize) -> Vec<u16> {
    let v = std::io::stdin()
                    .lines()
                    .take(x)
                    .map(|v| v.unwrap().trim().parse::<u16>().unwrap())
                    .collect::<Vec<_>>();
    return v;
}

fn main() {
    let [x1, x2, x3] = input(3)[..] else { todo!() };
    let entrance = (x3 - 1) / (x1 * x2) + 1;
    let floor = ((x3 - 1) / x2) % x1 + 1;
    println!(
        "Квартира с номером {} находится в подъезде {} на {} этаже",
        x3,
        entrance,
        floor
    );
}