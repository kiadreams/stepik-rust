use std::io;

fn main() {
    let mut str_fl1 = String::new();
    let mut str_fl2 = String::new();
    io::stdin().read_line(&mut str_fl1).expect("msg");
    io::stdin().read_line(&mut str_fl2).expect("msg");
    let fl1: f64 = str_fl1.trim().parse().expect("msg");
    let fl2: f64 = str_fl2.trim().parse().expect("msg");
    if fl1 > fl2 {
        println!("Из {fl1:.3} и {fl2:.3} больше {fl1:.3}");
    } else {
        println!("Из {fl1:.3} и {fl2:.3} больше {fl2:.3}");
    }
}
