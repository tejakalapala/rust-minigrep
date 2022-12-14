use std::env;
use std::process;
use minigrep::Config;
use minigrep::run;
fn main() {
    let args:Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("error parsing values:{}",err);
        process::exit(1);
    });

    println!("Searching for :{}",config.query);
    println!("in filename :{}",config.filename);

   if let Err(e) = run(config){
    println!("Application error:{}",e);
    process::exit(1);
   }

}



