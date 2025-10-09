use std::io;

fn main() {
    let mut str_fl1 = String::new();
    io::stdin().read_line(&mut str_fl1).expect("msg");
    let fl1: f64 = str_fl1.trim().parse().expect("msg");
    if fl1 > 0_f64 {
        println!("Число {:.1} является положительным", fl1);
    } else {
        println!("Число {:.1} является отрицательным", fl1);
    }
}
