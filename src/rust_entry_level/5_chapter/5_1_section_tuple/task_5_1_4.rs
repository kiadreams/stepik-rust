use std::io;

fn main() {
    let tup = (10.0, 5.0, -2.0, 100.0, 2000.0, 0.0);
    let mut str_fl1 = String::new();
    let mut str_fl2 = String::new();
    let mut str_fl3 = String::new();
    let mut str_fl4 = String::new();
    let mut str_fl5 = String::new();
    io::stdin().read_line(&mut str_fl1).expect("msg");
    io::stdin().read_line(&mut str_fl2).expect("msg");
    io::stdin().read_line(&mut str_fl3).expect("msg");
    io::stdin().read_line(&mut str_fl4).expect("msg");
    io::stdin().read_line(&mut str_fl5).expect("msg");
    let (fl1, fl2, fl3, fl4, fl5): (f64, f64, f64, f64, f64) = (
        str_fl1.trim().parse().expect("msg"),
        str_fl2.trim().parse().expect("msg"),
        str_fl3.trim().parse().expect("msg"),
        str_fl4.trim().parse().expect("msg"),
        str_fl5.trim().parse().expect("msg"),
    );
    println!(
        "{}, {}, {}, {}, {}, {}",
        fl1 as i32, fl2 as i32, fl3 as i32, fl4 as i32, fl5 as i32, tup.5
    );
}
