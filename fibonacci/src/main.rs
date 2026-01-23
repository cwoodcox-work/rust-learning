use std::io;

fn main() {
    println!("Give N to calculate the Nth number in the Fibonacci sequence");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read");

    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    println!("The {}th Fibonacci number is {}", n, fibo(n));
}

fn fibo(mut x:u32) -> u64 {
    let mut a = [1,1]; 
    let mut y: u64 = 0;
    if x == 0 {
        return 0;
    }
    if x == 1 || x == 2 {
        return 1;
    }
    while x > 2 {
        y = a[0] + a[1];
        a[0] = a[1];
        a[1] = y;
        x -= 1;} 
    return y;

}