use std::{env, fs, process};
use std::error::Error;
use mini::{Config, run};

fn main() {
   let args:Vec<String> = env::args().collect();
   dbg!(&args);
   // let query = &args[1];
   // let file_path = &args[2];
   // println!("Searching for {query} at {file_path}");
   // let contents = fs::read_to_string(file_path).expect("Cannot read");
   // println!("File content: {:?}", contents);
   let config = Config::build(&args).unwrap_or_else(|err|{
      eprintln!("Error occured: {err}");
      process::exit(1)
   });
   if let Err(e) = run(config){
      println!("Err occured! {e}");
      process::exit(-1);
   }
}
