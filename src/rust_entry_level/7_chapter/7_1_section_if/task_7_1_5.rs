fn input(x: usize) -> Vec<f64> {
    let v = std::io::stdin()
        .lines()
        .take(x)
        .map(|v| v.unwrap().trim().parse::<f64>().unwrap())
        .collect::<Vec<_>>();
    return v;
}

fn main() {
    let [tempr] = input(1)[..] else { todo!() };
    if tempr < 24.0 {
        println!("Температура {:.1}°C ниже нормы 24.0°C, включаю отопление", tempr);
    } else if tempr > 28.0 {
        println!("Температура {:.1}°C выше нормы 28.0°C, отключаю отопление", tempr);
    } else {
        println!("Температура {:.1}°C в пределах нормы", tempr);
    }
}
