use std::io;


fn main() {
    let (mut a, mut b) = (String::new(), String::new());
    io::stdin()
        .read_line(&mut a)
        .expect("Ждал первое число!");
    io::stdin()
        .read_line(&mut b)
        .expect("Ждал второе число");
    let a: i32 = a.trim().parse().expect("Это не число");
    let b: i32 = b.trim().parse().expect("Это не число");
    println!("{a:#b} + {b:#b} = {:#b}", a + b);
    println!("{a:#o} + {b:#o} = {:#o}", a + b);
    println!("{a:#x} + {b:#x} = {:#x}\n", a + b);

    println!("{a:#b} - {b:#b} = {:#b}", a - b);
    println!("{a:#o} - {b:#o} = {:#o}", a - b);
    println!("{a:#x} - {b:#x} = {:#x}\n", a - b);

    println!("{a:#b} * {b:#b} = {:#b}", a * b);
    println!("{a:#o} * {b:#o} = {:#o}", a * b);
    println!("{a:#x} * {b:#x} = {:#x}\n", a * b);

    println!("{a:#b} / {b:#b} = {:#b}", a / b);
    println!("{a:#o} / {b:#o} = {:#o}", a / b);
    println!("{a:#x} / {b:#x} = {:#x}\n", a / b);

    println!("{a:#b} % {b:#b} = {:#b}", a % b);
    println!("{a:#o} % {b:#o} = {:#o}", a % b);
    println!("{a:#x} % {b:#x} = {:#x}\n", a % b);
}