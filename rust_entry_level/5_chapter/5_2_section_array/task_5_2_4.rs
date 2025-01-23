use std::io;


fn main() {
    let (mut str_int1, mut str_int2) = (String::new(), String::new());
    io::stdin().read_line(&mut str_int1).expect("msg");
    io::stdin().read_line(&mut str_int2).expect("msg");
    let (int1, int2): (usize, i32) = (
        str_int1.trim().parse().expect("msg"),
        str_int2.trim().parse().expect("msg"),
    );
    let mut arr = [0; 10];
    arr[int1] = int2;
    println!("{:?}", arr);
}