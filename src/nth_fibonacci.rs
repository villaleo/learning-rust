use std::io;

fn calculate(n: i64) -> i64 {
    if n == 1 {
        return 0;
    }
    if n == 2 {
        return 1;
    }

    calculate(n - 1) + calculate(n - 2)
}

pub fn main() {
    println!("Enter n:");
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Error while reading input.");
    
    let n: i64 = n
        .trim()
        .parse()
        .expect("Not an integer!");
    
    println!("Nth fibonacci number is {}.", calculate(n));
}
