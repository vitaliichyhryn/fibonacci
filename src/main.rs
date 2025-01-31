use core::panic;
use std::env;

fn rec_nth_fib(n: u128) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => rec_nth_fib(n - 1) + rec_nth_fib(n - 2),
    }
}

fn iter_nth_fib(mut n: u128) -> u128 {
    let (mut a, mut b) = (0, 1);
    while n != 0 {
        (a, b) = (b, a + b);
        n -= 1;
    }
    a
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let approach = &args[1];
    let n = args[2].parse().expect("Couldn't get n.");
    let num = match approach.as_str() {
        "iter" => iter_nth_fib(n),
        "rec" => rec_nth_fib(n),
        _ => panic!(),
    };

    let suffix = match n {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };

    println!("{n}{suffix} Fibonacci number is {num}.");
}
