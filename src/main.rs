use mymathboard::repl::Repl;
use std::io;
use std::io::Write;

fn main() {
    let mut repl = Repl::new();
    loop {
        let mut input = String::new();
        print!(">>> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let result = repl.process_input(&input);
        println!("=> {:?}", result);
    }
}
