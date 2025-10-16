fn input(x: usize) -> Vec<u16> {
    let v = std::io::stdin()
        .lines()
        .take(x)
        .map(|v| v.unwrap().trim().parse::<u16>().unwrap())
        .collect::<Vec<_>>();
    return v;
}

fn main() {
    let [coint, bill] = input(2)[..] else { todo!() };
    let coints = [1, 2, 5, 10];
    let bills = [5, 10, 50, 100, 200, 500, 1000, 2000, 5000];
    if coints.contains(&coint) {
        println!("Принята монета номинала {coint}");
    } else {
        println!("Монеты такого номинала не принимаются");
    }
    if bills.contains(&bill) {
        println!("Принята купюра номинала {bill}");
    } else {
        println!("Купюры такого номинала не принимаются");
    }
}
