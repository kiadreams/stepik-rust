fn input(x: usize) -> Vec<f64> {
    let v = std::io::stdin()
        .lines()
        .take(x)
        .map(|v| v.unwrap().trim().parse::<f64>().unwrap())
        .collect::<Vec<_>>();
    return v;
}

fn main() {
    let [x1, x2, x3] = input(3)[..] else { todo!() };
    if x1 < x2 && x1 < x3 {
        if x2 < x3 {
            println!("{x1:.1}, {x2:.1}, {x3:.1}");
        } else {
            println!("{x1:.1}, {x3:.1}, {x2:.1}");
        }
    } else if x2 < x1 && x2 < x3 {
        if x1 < x3 {
            println!("{x2:.1}, {x1:.1}, {x3:.1}");
        } else {
            println!("{x2:.1}, {x3:.1}, {x1:.1}");
        }
    } else {
        if x1 < x2 {
            println!("{x3:.1}, {x1:.1}, {x2:.1}");
        } else {
            println!("{x3:.1}, {x2:.1}, {x1:.1}");
        }
    }
}
