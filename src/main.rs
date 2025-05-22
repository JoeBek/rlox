
use std::env;
use std::fs;
use std::process;
use std::error::Error;


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


}

/// the big money fn
fn run(content: &str) {

    ()
    
}

