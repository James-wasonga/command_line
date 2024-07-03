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

//     let contents = fs::read_to_string(config.file_path)
//         .expect("Should have been able to read the file");
//     println!("with text:\n {}",contents);


//calling the run function here
 if let Err(e) = minigrep::run(config){
    println!("Application error: {e}");
    process::exit(1);
 }
}
