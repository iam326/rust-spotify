use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: cargo run --bin rust-spotify <MESSAGE>");
        return;
    }

    let msg = &args[1];
    println!("Result: {}", msg);
}