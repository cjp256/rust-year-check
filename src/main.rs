use std::env;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let re = Regex::new(r"^[0-9]{4}$").unwrap();

    if args.len() < 2 {
        println!("Tell me what year it is.")
    } else if re.is_match(&args[1]) {
        println!("The year is {}", args[1]);
    } else {
        println!("I cannot believe that the year is {}", args[1]);
    }
}
