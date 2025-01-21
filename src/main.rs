use std::io;

fn nth_fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => nth_fibonacci(n - 1) + nth_fibonacci(n - 2),
    }
}

fn main() {
    let n = loop {
        println!("Enter n.");
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Couldn't get n.");

        match n.trim().parse() {
            Ok(n) => break n,
            Err(_) => continue,
        }
    };

    let num = nth_fibonacci(n);

    let suffix = match n {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };

    println!("{n}{suffix} Fibonacci number is {num}.");
}
