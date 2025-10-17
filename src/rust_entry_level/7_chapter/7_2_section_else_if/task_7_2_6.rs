fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("");
    return buf.trim().to_string();
}

fn input_num(x: usize) -> Vec<u16> {
    let v = std::io::stdin()
        .lines()
        .take(x)
        .map(|v| v.unwrap().trim().parse::<u16>().unwrap())
        .collect::<Vec<_>>();
    return v;
}

fn main() {
    let city = input();
    let [x1, x2] = input_num(2)[..] else { todo!() };
    if city == "Четный" {
        if x1 % 2 == 0 {
            println!("{x1} в город {city} вход разрешен");
        } else {
            println!("{x1} в город {city} вход запрещен");
        }
        if x2 % 2 == 0 {
            println!("{x2} в город {city} вход разрешен");
        } else {
            println!("{x2} в город {city} вход запрещен");
        }
    } else {
        if x1 % 2 == 0 {
            println!("{x1} в город {city} вход запрещен");
        } else {
            println!("{x1} в город {city} вход разрешен");
        }
        if x2 % 2 == 0 {
            println!("{x2} в город {city} вход запрещен");
        } else {
            println!("{x2} в город {city} вход разрешен");
        }
    }
}
