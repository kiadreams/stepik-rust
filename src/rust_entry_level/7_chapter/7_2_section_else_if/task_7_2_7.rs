fn input(x: usize) -> Vec<u8> {
    let v = std::io::stdin()
        .lines()
        .take(x)
        .map(|v| v.unwrap().trim().parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    return v;
}

fn check_sys(sys: u8, min_sys: u8, max_sys: u8) {
    if sys >= min_sys && sys <= max_sys {
        println!("Систолическое АД в норме");
    } else if sys < min_sys {
        println!("Систолическое АД {} ниже нормы на {}", sys, min_sys - sys);
    } else {
        println!("Систолическое АД {} выше нормы на {}", sys, sys - max_sys);
    }
}

fn check_dia(dia: u8, min_dia: u8, max_dia: u8) {
    if dia >= min_dia && dia <= max_dia {
        println!("Диастолическое АД в норме");
    } else if dia < min_dia {
        println!("Диастолическое АД {} ниже нормы на {}", dia, min_dia - dia);
    } else {
        println!("Диастолическое АД {} выше нормы на {}", dia, dia - max_dia);
    }
}

fn check_sys_and_dia(s: u8, d: u8, mn_s: u8, mx_s: u8, mn_d: u8, mx_d: u8) {
    if s >= mn_s && s <= mx_s && d >= mn_d && d <= mx_d {
        println!("Систолическое и Диастолическое АД в норме");
    } else {
        check_sys(s, mn_s, mx_s);
        check_dia(d, mn_d, mx_d);
    }
}

fn main() {
    let [age, sys, dia] = input(3)[..] else { todo!() };
    if age > 59 {
        check_sys_and_dia(sys, dia, 91, 159, 61, 91);
    } else if age > 39 {
        check_sys_and_dia(sys, dia, 91, 149, 61, 91);
    } else if age > 17 {
        check_sys_and_dia(sys, dia, 90, 139, 60, 89);
    }
}
