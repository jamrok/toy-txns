use std::env;
use std::error::Error;
use std::process;
use toy_txns::libs::transactions::TransactionManager;
use toy_txns::Params;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Params::parse(env::args()).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    // Process Input File (get Plan)
    let output = TransactionManager::process_input_file(args.filename()).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    println!("{}", output);
    Ok(())
}
