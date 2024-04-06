use sr_algo::parentheses::check_string;

fn main() {
    let mut str = String::new();

    std::io::stdin().read_line(&mut str).unwrap();

    if check_string(str.trim()) {
        println!("Correct");
    } else {
        println!("Incorrect");
    }
}
