use std::io;

fn main() {
    let mut str_fl1 = String::new();
    let mut str_fl2 = String::new();
    io::stdin().read_line(&mut str_fl1).expect("msg");
    io::stdin().read_line(&mut str_fl2).expect("msg");
    let fl1: i32 = str_fl1.trim().parse().expect("msg");
    let fl2: i32 = str_fl2.trim().parse().expect("msg");
    if fl1 % 2 == 0 {
        println!("Число {fl1} является четным");
    } else {
        println!("Число {fl1} является нечетным");
    }
    if fl2 % 2 == 0 {
        println!("Число {fl2} является четным");
    } else {
        println!("Число {fl2} является нечетным");
    }
}
