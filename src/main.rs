fn repl() {
    println!("Welcome to the Ruje!");
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let result = ruje::core::rep(&input);
        match result {
            Ok(output) => println!("{}", output),
            Err(e) => println!("Error: {}", e),
        }
    }
}

fn main() {
    println!("Hello, world!");
    repl();
}
