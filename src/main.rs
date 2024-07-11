mod generator;
mod parser;
mod domain;

use std::{env, process};
use crate::parser::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing args: {err}");
        process::exit(1);
    });

   match parser::run(config) {
       Ok(domain) => {
           generator::run(domain);
           println!("Generator executed");
       }

       Err(e) => {
           println!("Application error: {e}");
           process::exit(1);
       }
   }
}
