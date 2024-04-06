use sr_algo::stack::Stack;
use std::io;

fn main() {
    let mut stack = Stack::new();
    let mut stack_even = Stack::new();
    let mut stack_odd = Stack::new();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().expect("Please, type a number!");

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let number: i32 = input.trim().parse().expect("Please, type a number!");
        stack.push(number);

        if number & 1 == 1 {
            stack_odd.push(number)
        } else {
            stack_even.push(number)
        }
    }

    println!(
        "Stack: {}\nStack Even: {}\nStack Odd: {}",
        stack, stack_even, stack_odd
    );
}
