use std::io;

fn main() {
    let mut str_int1 = String::new();
    let mut str_int2 = String::new();
    let mut str_int3 = String::new();
    let mut str_int4 = String::new();
    let mut str_int5 = String::new();
    io::stdin().read_line(&mut str_int1).expect("msg");
    io::stdin().read_line(&mut str_int2).expect("msg");
    io::stdin().read_line(&mut str_int3).expect("msg");
    io::stdin().read_line(&mut str_int4).expect("msg");
    io::stdin().read_line(&mut str_int5).expect("msg");
    let (int1, int2, int3, int4, int5): (usize, usize, usize, usize, usize) = (
        str_int1.trim().parse().expect("msg"),
        str_int2.trim().parse().expect("msg"),
        str_int3.trim().parse().expect("msg"),
        str_int4.trim().parse().expect("msg"),
        str_int5.trim().parse().expect("msg"),
    );
    let arr = [int1, int2, int3, int4, int5];
    println!(
        "{}, {}, {}, {}, {}",
        arr[int1], arr[int2], arr[int3], arr[int4], arr[int5],
    );
}
