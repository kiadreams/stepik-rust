fn input(x: usize) -> Vec<u16> {
    let v = std::io::stdin()
        .lines()
        .take(x)
        .map(|v| v.unwrap().trim().parse::<u16>().unwrap())
        .collect::<Vec<_>>();
    return v;
}

fn main() {
    let [coint] = input(1)[..] else { todo!() };
    if coint == 1 || coint == 2 || coint == 5 || coint == 10 {
        println!("Принята монета номинала {coint}");
    } else {
        println!("Монеты такого номинала не принимаются");
    }
}
