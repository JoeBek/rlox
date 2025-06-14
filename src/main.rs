
use std::env;
use std::fs;
use std::io::{stdin,stdout, Write};
use std::process;
use std::error::Error;

mod token;
mod token_type;
mod scanner;
mod error;

fn main() {

    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    if args.len() > 2 {
        println!("too many args dummy");
        process::exit(64);
    }
    else if args.len() == 2 {
        println!("running file: {}", &args[1]);
        if let Err(e) = run_file(&args[1]) {
            eprintln!("error: {} for path: {}", e, &args[1]);
            process::exit(65);
        }
    }
    else {
        run_prompt();
    }

    
}

/// runs the interpreter from a source file
fn run_file(filepath: &str) -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(filepath)?;

    println!("parsing: {}\n", contents);

    run(&contents);


    Ok(())
    
    

}

/// runs the interpreter from command line
fn run_prompt() {

    println!("Welcome to the REPL");

    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).expect("could not read stdin");
        
        let input = input.trim();

        if input.is_empty() {
            break;
        }
        run(&input);

    }


}

/// the big money fn
fn run(content: &str) {

    // create scanner and token list

    /* 
    let scanner:Scanner = Scanner::new();
    let tokens:vec<Token> = vec::new();


    for token in &tokens {
        println!(token);
    }
    */

    ()


}

