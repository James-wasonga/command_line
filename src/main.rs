use std::env;
use std::process; 

use minigrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    // let query = &args[1];
    // let file_path = &args[2];
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem passing an arguments: {err}");
        process::exit(1);
    });
    
    println!("Searching for {}",config.query);
    println!("In file {}",config.file_path);


//calling the run function here
 if let Err(e) = minigrep::run(config){
    println!("Application error: {e}");
    process::exit(1);
 }
}
