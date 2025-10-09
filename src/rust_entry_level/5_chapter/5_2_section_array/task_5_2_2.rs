use std::io;

fn main() {
    let mut str_fl1 = String::new();
    let mut str_fl2 = String::new();
    let mut str_fl3 = String::new();
    let mut str_fl4 = String::new();
    let mut str_fl5 = String::new();
    let mut str_int1 = String::new();
    io::stdin().read_line(&mut str_fl1).expect("msg");
    io::stdin().read_line(&mut str_fl2).expect("msg");
    io::stdin().read_line(&mut str_fl3).expect("msg");
    io::stdin().read_line(&mut str_fl4).expect("msg");
    io::stdin().read_line(&mut str_fl5).expect("msg");
    io::stdin().read_line(&mut str_int1).expect("msg");
    let (fl1, fl2, fl3, fl4, fl5, int1): (f64, f64, f64, f64, f64, usize) = (
        str_fl1.trim().parse().expect("msg"),
        str_fl2.trim().parse().expect("msg"),
        str_fl3.trim().parse().expect("msg"),
        str_fl4.trim().parse().expect("msg"),
        str_fl5.trim().parse().expect("msg"),
        str_int1.trim().parse().expect("msg"),
    );
    let arr = [fl1, fl2, fl3, fl4, fl5];
    println!("{:.2}", arr[int1]);
}
