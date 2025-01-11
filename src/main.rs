fn repl() {
    println!("Welcome to the Ruje!");
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        println!("{}", input.trim());
    }
}

fn main() {
    println!("Hello, world!");
    repl();
}
